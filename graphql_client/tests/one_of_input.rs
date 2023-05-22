use graphql_client::*;
use serde_json::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/one_of_input/schema.graphql",
    query_path = "tests/one_of_input/query.graphql",
    variables_derives = "Clone"
)]
pub struct OneOfMutation;

#[test]
fn one_of_input() {
    use one_of_mutation::*;

    let author = Param::Author(Author { id: 1 });
    let _ = Param::Name("Mark Twain".to_string());
    let _ = Param::RecursiveDirect(Box::new(author.clone()));
    let _ = Param::RecursiveIndirect(Box::new(Recursive {
        param: Box::new(author.clone()),
    }));
    let _ = Param::RequiredInts(vec![1]);
    let _ = Param::OptionalInts(vec![Some(1)]);

    let query = OneOfMutation::build_query(Variables { param: author });
    assert_eq!(
        json!({ "param": { "author":{ "id": 1 } } }),
        serde_json::to_value(&query.variables).expect("json"),
    );
}
