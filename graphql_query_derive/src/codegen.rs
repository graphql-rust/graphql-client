use constants::*;
use failure;
use fragments::GqlFragment;
use graphql_parser::query;
use proc_macro2::TokenStream;
use query::QueryContext;
use schema;
use selection::Selection;

pub(crate) fn response_for_query(
    schema: schema::Schema,
    query: query::Document,
    selected_operation: Option<String>,
) -> Result<TokenStream, failure::Error> {
    let mut context = QueryContext::new(schema);
    let mut definitions = Vec::new();

    for definition in query.definitions {
        match definition {
            query::Definition::Operation(query::OperationDefinition::Query(q)) => {
                context.root = {
                    let definition = context
                        .schema
                        .query_type
                        .clone()
                        .and_then(|query_type| context.schema.objects.get(&query_type))
                        .expect("query type is defined");
                    let prefix = &q.name.expect("unnamed operation");
                    let prefix = format!("RUST_{}", prefix);
                    let selection = Selection::from(&q.selection_set);

                    definitions.extend(
                        definition.field_impls_for_selection(&context, &selection, &prefix)?,
                    );
                    Some(definition.response_fields_for_selection(&context, &selection, &prefix)?)
                };

                context.register_variables(&q.variable_definitions);
            }
            query::Definition::Operation(query::OperationDefinition::Mutation(q)) => {
                context.root = {
                    let definition = context
                        .schema
                        .mutation_type
                        .clone()
                        .and_then(|mutation_type| context.schema.objects.get(&mutation_type))
                        .expect("mutation type is defined");
                    let prefix = &q.name.expect("unnamed operation");
                    let prefix = format!("RUST_{}", prefix);
                    let selection = Selection::from(&q.selection_set);

                    definitions.extend(
                        definition.field_impls_for_selection(&context, &selection, &prefix)?,
                    );
                    Some(definition.response_fields_for_selection(&context, &selection, &prefix)?)
                };

                context.register_variables(&q.variable_definitions);
            }
            query::Definition::Operation(query::OperationDefinition::Subscription(q)) => {
                context.root = {
                    let definition = context
                        .schema
                        .subscription_type
                        .clone()
                        .and_then(|subscription_type| {
                            context.schema.objects.get(&subscription_type)
                        })
                        .expect("subscription type is defined");
                    let prefix = &q.name.expect("unnamed operation");
                    let prefix = format!("RUST_{}", prefix);
                    let selection = Selection::from(&q.selection_set);

                    if selection.0.len() > 1 {
                        Err(format_err!(
                            "{}",
                            ::constants::MULTIPLE_SUBSCRIPTION_FIELDS_ERROR
                        ))?
                    }

                    definitions.extend(
                        definition.field_impls_for_selection(&context, &selection, &prefix)?,
                    );
                    Some(definition.response_fields_for_selection(&context, &selection, &prefix)?)
                };

                context.register_variables(&q.variable_definitions);
            }
            query::Definition::Operation(query::OperationDefinition::SelectionSet(_)) => {
                panic!(SELECTION_SET_AT_ROOT)
            }
            query::Definition::Fragment(fragment) => {
                let query::TypeCondition::On(on) = fragment.type_condition;
                context.fragments.insert(
                    fragment.name.clone(),
                    GqlFragment {
                        name: fragment.name,
                        selection: Selection::from(&fragment.selection_set),
                        on,
                    },
                );
            }
        }
    }

    let enum_definitions = context.schema.enums.values().map(|enm| enm.to_rust());
    let fragment_definitions: Result<Vec<TokenStream>, _> = context
        .fragments
        .values()
        .map(|fragment| fragment.to_rust(&context))
        .collect();
    let fragment_definitions = fragment_definitions?;
    let variables_struct = context.expand_variables();
    let response_data_fields = context.root.as_ref().expect("no selection defined");

    let input_object_definitions: Result<Vec<TokenStream>, _> = context
        .schema
        .inputs
        .values()
        .map(|i| i.to_rust(&context))
        .collect();
    let input_object_definitions = input_object_definitions?;

    let scalar_definitions: Vec<TokenStream> = context
        .schema
        .scalars
        .values()
        .map(|s| s.to_rust())
        .collect();

    Ok(quote! {
        type Boolean = bool;
        type Float = f64;
        type Int = i64;
        type ID = String;

        #(#scalar_definitions)*

        #(#input_object_definitions)*

        #(#enum_definitions)*

        #(#fragment_definitions)*

        #(#definitions)*

        #variables_struct

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct ResponseData {
            #(#response_data_fields)*,
        }

    })
}
