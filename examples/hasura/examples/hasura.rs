use graphql_client::*;
use log::*;
use prettytable::*;

type Bpchar = String;
type Timestamptz = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "examples/schema.graphql",
    query_path = "examples/query_1.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
struct UpsertIssue;

fn main() -> Result<(), anyhow::Error> {
    use upsert_issue::{IssuesUpdateColumn::*, *};
    dotenv::dotenv().ok();
    env_logger::init();

    let q = UpsertIssue::build_query(Variables {
        issues: vec![IssuesInsertInput {
            id: Some("001000000000000".to_string()),
            name: Some("Name".to_string()),
            status: Some("Draft".to_string()),
            salesforce_updated_at: Some("2019-06-11T08:14:28Z".to_string()),
        }],
        update_columns: vec![Name, Status, SalesforceUpdatedAt],
    });

    let client = reqwest::Client::new();

    let mut res = client
        .post("https://localhost:8080/v1/graphql")
        .json(&q)
        .send()?;

    let response_body: Response<ResponseData> = res.json()?;
    info!("{:?}", response_body);

    if let Some(errors) = response_body.errors {
        error!("there are errors:");

        for error in &errors {
            error!("{:?}", error);
        }
    }

    let response_data = response_body.data.expect("missing response data");

    let mut table = prettytable::Table::new();

    table.add_row(row!(b => "id", "name"));

    for issue in &response_data
        .insert_issues
        .expect("Inserted Issues")
        .returning
    {
        table.add_row(row!(issue.id, issue.name));
    }

    table.printstd();
    Ok(())
}
