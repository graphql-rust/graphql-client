use failure;
use fragments::GqlFragment;
use graphql_parser::query;
use operations::Operation;
use proc_macro2::TokenStream;
use query::QueryContext;
use schema;
use selection::Selection;

pub(crate) fn response_for_query(
    schema: schema::Schema,
    query: query::Document,
    selected_operation: String,
) -> Result<TokenStream, failure::Error> {
    let mut context = QueryContext::new(schema);
    let mut definitions = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();

    for definition in query.definitions {
        match definition {
            query::Definition::Operation(op) => {
                operations.push(op.into());
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

    context.selected_operation = operations
        .iter()
        .find(|op| op.name == selected_operation)
        .map(|i| i.to_owned());

    let operation = context.selected_operation.clone().unwrap_or_else(|| {
        operations
            .iter()
            .next()
            .map(|i| i.to_owned())
            .expect("no operation in query document")
    });

    let response_data_fields = {
        let root_name: String = operation
            .root_name(&context.schema)
            .expect("operation type not in schema");
        let definition = context
            .schema
            .objects
            .get(&root_name)
            .expect("schema declaration is invalid");
        let prefix = format!("RUST_{}", operation.name);
        let selection = &operation.selection;

        if operation.is_subscription() && selection.0.len() > 1 {
            Err(format_err!(
                "{}",
                ::constants::MULTIPLE_SUBSCRIPTION_FIELDS_ERROR
            ))?
        }

        definitions.extend(
            definition
                .field_impls_for_selection(&context, &selection, &prefix)
                .unwrap(),
        );
        definition
            .response_fields_for_selection(&context, &selection, &prefix)
            .unwrap()
    };

    let enum_definitions = context.schema.enums.values().map(|enm| enm.to_rust());
    let fragment_definitions: Result<Vec<TokenStream>, _> = context
        .fragments
        .values()
        .map(|fragment| fragment.to_rust(&context))
        .collect();
    let fragment_definitions = fragment_definitions?;
    let variables_struct = operation.expand_variables(&context);

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
            #(#response_data_fields,)*
        }

    })
}
