use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/enum_defaults/query.graphql",
    schema_path = "tests/enum_defaults/schema.graphql",
    variables_derives = "Debug, PartialEq, Eq"
)]
pub struct EnumDefaults;

#[test]
fn enum_literal_as_a_scalar_default() {
    use enum_defaults::*;

    assert_eq!(Variables::default_kind(), Kind::ACTIVE);
}

#[test]
fn enum_literal_in_a_list_default() {
    use enum_defaults::*;

    assert_eq!(
        Variables::default_kinds(),
        vec![Kind::ACTIVE, Kind::ARCHIVED]
    );
}

#[test]
fn enum_literal_in_an_input_object_default() {
    use enum_defaults::*;

    assert_eq!(
        Variables::default_filter(),
        Filter {
            kind: Some(Kind::ARCHIVED),
            label: None,
        }
    );
}
