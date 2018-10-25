use constants::*;
use graphql_parser::query::OperationDefinition;
use heck::SnakeCase;
use proc_macro2::{Span, TokenStream};
use query::QueryContext;
use selection::Selection;
use syn::Ident;
use variables::Variable;

#[derive(Debug, Clone)]
pub enum OperationType {
    Query,
    Mutation,
    Subscription,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub name: String,
    pub operation_type: OperationType,
    pub variables: Vec<Variable>,
    pub selection: Selection,
}

impl Operation {
    pub(crate) fn root_name(&self, schema: &::schema::Schema) -> Option<String> {
        match self.operation_type {
            OperationType::Query => schema.query_type.clone(),
            OperationType::Mutation => schema.mutation_type.clone(),
            OperationType::Subscription => schema.subscription_type.clone(),
        }
    }

    pub(crate) fn is_subscription(&self) -> bool {
        match self.operation_type {
            OperationType::Subscription => true,
            _ => false,
        }
    }

    /// Generate the Variables struct and all the necessary supporting code.
    pub(crate) fn expand_variables(
        &self,
        context: &QueryContext,
        operation_name: &str,
        mulutiple_operation: bool,
    ) -> TokenStream {
        let variables = &self.variables;
        let variables_struct_name = if mulutiple_operation {
            Ident::new(
                format!("{}Variables", operation_name).as_str(),
                Span::call_site(),
            )
        } else {
            Ident::new("Variables", Span::call_site())
        };

        let variables_derives = context.variables_derives();

        if variables.is_empty() {
            return quote!(#variables_derives
            pub struct #variables_struct_name;);
        }

        let fields = variables.iter().map(|variable| {
            let name = &variable.name;
            let ty = variable.ty.to_rust(context, "");
            let snake_case_name = name.to_snake_case();
            let rename = ::shared::field_rename_annotation(&name, &snake_case_name);
            let name = Ident::new(&snake_case_name, Span::call_site());

            quote!(#rename pub #name: #ty)
        });

        let default_constructors = variables
            .iter()
            .map(|variable| variable.generate_default_value_constructor(context));

        quote! {
            #variables_derives
            pub struct #variables_struct_name {
                #(#fields,)*
            }

            impl #variables_struct_name {
                #(#default_constructors)*
            }
        }
    }
}

impl ::std::convert::From<OperationDefinition> for Operation {
    fn from(definition: OperationDefinition) -> Operation {
        match definition {
            OperationDefinition::Query(q) => Operation {
                name: q.name.expect("unnamed operation"),
                operation_type: OperationType::Query,
                variables: q
                    .variable_definitions
                    .iter()
                    .map(|v| v.clone().into())
                    .collect(),
                selection: (&q.selection_set).into(),
            },
            OperationDefinition::Mutation(m) => Operation {
                name: m.name.expect("unnamed operation"),
                operation_type: OperationType::Mutation,
                variables: m
                    .variable_definitions
                    .iter()
                    .map(|v| v.clone().into())
                    .collect(),
                selection: (&m.selection_set).into(),
            },
            OperationDefinition::Subscription(s) => Operation {
                name: s.name.expect("unnamed operation"),
                operation_type: OperationType::Subscription,
                variables: s
                    .variable_definitions
                    .iter()
                    .map(|v| v.clone().into())
                    .collect(),
                selection: (&s.selection_set).into(),
            },
            OperationDefinition::SelectionSet(_) => panic!(SELECTION_SET_AT_ROOT),
        }
    }
}

impl<'a> ::std::convert::From<&'a OperationDefinition> for Operation {
    fn from(definition: &OperationDefinition) -> Operation {
        match *definition {
            OperationDefinition::Query(ref q) => Operation {
                name: q.name.clone().expect("unnamed operation"),
                operation_type: OperationType::Query,
                variables: q
                    .variable_definitions
                    .iter()
                    .map(|v| v.clone().into())
                    .collect(),
                selection: (&q.selection_set).into(),
            },
            OperationDefinition::Mutation(ref m) => Operation {
                name: m.name.clone().expect("unnamed operation"),
                operation_type: OperationType::Mutation,
                variables: m
                    .variable_definitions
                    .iter()
                    .map(|v| v.clone().into())
                    .collect(),
                selection: (&m.selection_set).into(),
            },
            OperationDefinition::Subscription(ref s) => Operation {
                name: s.name.clone().expect("unnamed operation"),
                operation_type: OperationType::Subscription,
                variables: s
                    .variable_definitions
                    .iter()
                    .map(|v| v.clone().into())
                    .collect(),
                selection: (&s.selection_set).into(),
            },
            OperationDefinition::SelectionSet(_) => panic!(SELECTION_SET_AT_ROOT),
        }
    }
}
