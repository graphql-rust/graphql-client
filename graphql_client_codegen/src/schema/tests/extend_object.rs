use crate::schema::Schema;

const SCHEMA_JSON: &str = include_str!("extend_object_schema.json");
const SCHEMA_GRAPHQL: &str = include_str!("extend_object_schema.graphql");

#[test]
fn ast_from_graphql_and_json_produce_the_same_schema() {
    let json: graphql_introspection_query::introspection_response::IntrospectionResponse =
        serde_json::from_str(SCHEMA_JSON).unwrap();
    let graphql_parser_schema = graphql_parser::parse_schema(SCHEMA_GRAPHQL)
        .unwrap()
        .into_static();
    let mut json = Schema::from(json);
    let mut gql = Schema::from(graphql_parser_schema);

    assert!(vecs_match(&json.stored_scalars, &gql.stored_scalars));

    // Root objects
    {
        assert_eq!(
            json.get_object(json.query_type()).name,
            gql.get_object(gql.query_type()).name
        );
        assert_eq!(
            json.mutation_type().map(|t| &json.get_object(t).name),
            gql.mutation_type().map(|t| &gql.get_object(t).name),
            "Mutation types don't match."
        );
        assert_eq!(
            json.subscription_type().map(|t| &json.get_object(t).name),
            gql.subscription_type().map(|t| &gql.get_object(t).name),
            "Subscription types don't match."
        );
    }

    // Objects
    {
        let mut json_stored_objects: Vec<_> = json
            .stored_objects
            .drain(..)
            .filter(|obj| !obj.name.starts_with("__"))
            .collect();

        assert_eq!(
            json_stored_objects.len(),
            gql.stored_objects.len(),
            "Objects count matches."
        );

        json_stored_objects.sort_by(|a, b| a.name.cmp(&b.name));
        gql.stored_objects.sort_by(|a, b| a.name.cmp(&b.name));

        for (j, g) in json_stored_objects
            .iter_mut()
            .filter(|obj| !obj.name.starts_with("__"))
            .zip(gql.stored_objects.iter_mut())
        {
            assert_eq!(j.name, g.name);
            assert_eq!(
                j.implements_interfaces.len(),
                g.implements_interfaces.len(),
                "{}",
                j.name
            );
            assert_eq!(j.fields.len(), g.fields.len(), "{}", j.name);
        }
    }

    // Unions
    {
        assert_eq!(json.stored_unions.len(), gql.stored_unions.len());

        json.stored_unions.sort_by(|a, b| a.name.cmp(&b.name));
        gql.stored_unions.sort_by(|a, b| a.name.cmp(&b.name));

        for (json, gql) in json.stored_unions.iter().zip(gql.stored_unions.iter()) {
            assert_eq!(json.variants.len(), gql.variants.len());
        }
    }

    // Interfaces
    {
        assert_eq!(json.stored_interfaces.len(), gql.stored_interfaces.len());

        json.stored_interfaces.sort_by(|a, b| a.name.cmp(&b.name));
        gql.stored_interfaces.sort_by(|a, b| a.name.cmp(&b.name));

        for (json, gql) in json
            .stored_interfaces
            .iter()
            .zip(gql.stored_interfaces.iter())
        {
            assert_eq!(json.fields.len(), gql.fields.len());
        }
    }

    // Input objects
    {
        json.stored_enums = json
            .stored_enums
            .drain(..)
            .filter(|enm| !enm.name.starts_with("__"))
            .collect();
        assert_eq!(json.stored_inputs.len(), gql.stored_inputs.len());

        json.stored_inputs.sort_by(|a, b| a.name.cmp(&b.name));
        gql.stored_inputs.sort_by(|a, b| a.name.cmp(&b.name));

        for (json, gql) in json.stored_inputs.iter().zip(gql.stored_inputs.iter()) {
            assert_eq!(json.fields.len(), gql.fields.len());
        }
    }

    // Enums
    {
        assert_eq!(json.stored_enums.len(), gql.stored_enums.len());

        json.stored_enums.sort_by(|a, b| a.name.cmp(&b.name));
        gql.stored_enums.sort_by(|a, b| a.name.cmp(&b.name));

        for (json, gql) in json.stored_enums.iter().zip(gql.stored_enums.iter()) {
            assert_eq!(json.variants.len(), gql.variants.len());
        }
    }
}

fn vecs_match<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.len() == b.len() && a.iter().all(|a| b.iter().any(|b| a == b))
}
