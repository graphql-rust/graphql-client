use constants::*;
use deprecation::DeprecationStatus;
use failure;
use field_type::FieldType;
use graphql_parser::schema;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use schema::Schema;
use selection::*;
use shared::{field_impls_for_selection, response_fields_for_selection};
use std::borrow::Cow;
use std::cell::Cell;

#[derive(Debug, Clone, PartialEq)]
pub struct GqlObject {
    pub description: Option<String>,
    pub fields: Vec<GqlObjectField>,
    pub name: String,
    pub is_required: Cell<bool>,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct GqlObjectField {
    pub description: Option<String>,
    pub name: String,
    pub type_: FieldType,
    pub deprecation: DeprecationStatus,
}

fn parse_deprecation_info(field: &schema::Field) -> DeprecationStatus {
    let deprecated = field
        .directives
        .iter()
        .filter(|x| x.name.to_lowercase() == "deprecated")
        .nth(0);
    let reason = if let Some(d) = deprecated {
        if let Some((_, value)) = d
            .arguments
            .iter()
            .filter(|x| x.0.to_lowercase() == "reason")
            .nth(0)
        {
            match value {
                schema::Value::String(reason) => Some(reason.clone()),
                schema::Value::Null => None,
                _ => panic!("deprecation reason is not a string"),
            }
        } else {
            None
        }
    } else {
        None
    };
    match deprecated {
        Some(_) => DeprecationStatus::Deprecated(reason),
        None => DeprecationStatus::Current,
    }
}

impl GqlObject {
    pub fn new(name: Cow<str>, description: Option<&str>) -> GqlObject {
        GqlObject {
            description: description.map(|s| s.to_owned()),
            name: name.into_owned(),
            fields: vec![typename_field()],
            is_required: false.into(),
        }
    }

    pub fn from_graphql_parser_object(obj: schema::ObjectType) -> Self {
        let description = obj.description.as_ref().map(|s| s.as_str());
        let mut item = GqlObject::new(obj.name.into(), description);
        item.fields.extend(obj.fields.iter().map(|f| {
            let deprecation = parse_deprecation_info(&f);
            GqlObjectField {
                description: f.description.clone(),
                name: f.name.clone(),
                type_: FieldType::from(f.field_type.clone()),
                deprecation,
            }
        }));
        item
    }

    pub fn from_introspected_schema_json(obj: &::introspection_response::FullType) -> Self {
        let description = obj.description.as_ref().map(|s| s.as_str());
        let mut item = GqlObject::new(
            obj.name.clone().expect("missing object name").into(),
            description,
        );
        let fields = obj.fields.clone().unwrap().into_iter().filter_map(|t| {
            t.map(|t| {
                let deprecation = if t.is_deprecated.unwrap_or(false) {
                    DeprecationStatus::Deprecated(t.deprecation_reason)
                } else {
                    DeprecationStatus::Current
                };
                GqlObjectField {
                    description: t.description.clone(),
                    name: t.name.expect("field name"),
                    type_: FieldType::from(t.type_.expect("field type")),
                    deprecation,
                }
            })
        });

        item.fields.extend(fields);

        item
    }

    pub(crate) fn require(&self, schema: &Schema) {
        if self.is_required.get() {
            return;
        }
        self.is_required.set(true);
        self.fields
            .iter()
            .for_each(|field| {
                schema.require(&field.type_.inner_name_string());
            })
    }

    pub(crate) fn response_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<TokenStream, failure::Error> {
        let derives = query_context.response_derives();
        let name = Ident::new(prefix, Span::call_site());
        let fields = self.response_fields_for_selection(query_context, selection, prefix)?;
        let field_impls = self.field_impls_for_selection(query_context, selection, &prefix)?;
        let description = self.description.as_ref().map(|desc| quote!(#[doc = #desc]));
        Ok(quote! {
            #(#field_impls)*

            #derives
            #description
            pub struct #name {
                #(#fields,)*
            }
        })
    }

    pub(crate) fn field_impls_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<Vec<TokenStream>, failure::Error> {
        field_impls_for_selection(&self.fields, query_context, selection, prefix)
    }

    pub(crate) fn response_fields_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<Vec<TokenStream>, failure::Error> {
        response_fields_for_selection(&self.fields, query_context, selection, prefix)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use graphql_parser::query;
    use graphql_parser::Pos;

    fn mock_field(directives: Vec<schema::Directive>) -> schema::Field {
        schema::Field {
            position: Pos::default(),
            description: None,
            name: "foo".to_string(),
            arguments: vec![],
            field_type: schema::Type::NamedType("x".to_string()),
            directives,
        }
    }

    #[test]
    fn deprecation_no_reason() {
        let directive = schema::Directive {
            position: Pos::default(),
            name: "deprecated".to_string(),
            arguments: vec![],
        };
        let result = parse_deprecation_info(&mock_field(vec![directive]));
        assert_eq!(DeprecationStatus::Deprecated(None), result);
    }

    #[test]
    fn deprecation_with_reason() {
        let directive = schema::Directive {
            position: Pos::default(),
            name: "deprecated".to_string(),
            arguments: vec![(
                "reason".to_string(),
                query::Value::String("whatever".to_string()),
            )],
        };
        let result = parse_deprecation_info(&mock_field(vec![directive]));
        assert_eq!(
            DeprecationStatus::Deprecated(Some("whatever".to_string())),
            result
        );
    }

    #[test]
    fn null_deprecation_reason() {
        let directive = schema::Directive {
            position: Pos::default(),
            name: "deprecated".to_string(),
            arguments: vec![("reason".to_string(), query::Value::Null)],
        };
        let result = parse_deprecation_info(&mock_field(vec![directive]));
        assert_eq!(DeprecationStatus::Deprecated(None), result);
    }

    #[test]
    #[should_panic]
    fn invalid_deprecation_reason() {
        let directive = schema::Directive {
            position: Pos::default(),
            name: "deprecated".to_string(),
            arguments: vec![("reason".to_string(), query::Value::Boolean(true))],
        };
        let _ = parse_deprecation_info(&mock_field(vec![directive]));
    }

    #[test]
    fn no_deprecation() {
        let result = parse_deprecation_info(&mock_field(vec![]));
        assert_eq!(DeprecationStatus::Current, result);
    }
}
