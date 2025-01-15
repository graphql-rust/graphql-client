use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection_schema.graphql",
    query_path = "src/graphql/introspection_query.graphql",
    response_derives = "Serialize",
    variable_derives = "Deserialize"
)]
#[allow(dead_code)]
pub struct IntrospectionQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection_schema.graphql",
    query_path = "src/graphql/introspection_query_with_is_one_of.graphql",
    response_derives = "Serialize",
    variable_derives = "Deserialize"
)]
#[allow(dead_code)]
pub struct IntrospectionQueryWithIsOneOf;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection_schema.graphql",
    query_path = "src/graphql/introspection_query_with_specified_by.graphql",
    response_derives = "Serialize",
    variable_derives = "Deserialize"
)]
#[allow(dead_code)]
pub struct IntrospectionQueryWithSpecifiedBy;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection_schema.graphql",
    query_path = "src/graphql/introspection_query_with_isOneOf_specifiedByUrl.graphql",
    response_derives = "Serialize",
    variable_derives = "Deserialize"
)]
#[allow(dead_code)]
pub struct IntrospectionQueryWithIsOneOfSpecifiedByURL;
