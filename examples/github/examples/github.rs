use anyhow::*;
use graphql_client::*;
use log::*;
use prettytable::*;
use serde::*;
use serde::de::DeserializeOwned;
use structopt::StructOpt;

type URI = String;

trait GraphQLClient {
    fn send_request<REQ: Serialize + ?Sized, RES: DeserializeOwned>(
        &self,
        request_json: &REQ,
    ) -> Result<RES, anyhow::Error>;
}

trait ExecutableQuery {
    type Item: GraphQLQuery;

    fn execute(
        variables: <<Self as ExecutableQuery>::Item as GraphQLQuery>::Variables,
        client: &impl GraphQLClient,
    ) -> Result<Response<<<Self as ExecutableQuery>::Item as GraphQLQuery>::ResponseData>, anyhow::Error>
    {
        let query = <<Self as ExecutableQuery>::Item as GraphQLQuery>::build_query(variables);

        let response_body: Response<
            <<Self as ExecutableQuery>::Item as GraphQLQuery>::ResponseData,
        > = client.send_request(&query)?;
        return Ok(response_body);
    }
}
// implement ExecutableQuery for all GraphQLQuery implementations
impl<T: GraphQLQuery> ExecutableQuery for T {
    type Item = Self;
}

struct GraphQLClientImpl {
    client: reqwest::Client,
    github_api_token: String,
    url: String,
}

impl GraphQLClientImpl {
    fn new(github_api_token: String) -> GraphQLClientImpl {
        let client = reqwest::Client::new();
        return GraphQLClientImpl {
            client,
            github_api_token,
            url: "https://api.github.com/graphql".to_string(),
        };
    }
}

impl GraphQLClient for GraphQLClientImpl {
    fn send_request<REQ: Serialize + ?Sized, RES: DeserializeOwned>(
        &self,
        request_json: &REQ,
    ) -> Result<RES, anyhow::Error> {

        let request_builder = self
            .client
            .post(&self.url)
            .bearer_auth(&self.github_api_token)
            .json(&request_json);
        let mut response = request_builder.send()?;
        return response.json().map_err(|e| anyhow::Error::new(e));
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "examples/schema.graphql",
    query_path = "examples/query_1.graphql",
    response_derives = "Debug"
)]
struct RepoView;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "examples/schema.graphql",
    query_path = "examples/query_1.graphql",
    response_derives = "Debug"
)]
struct CodesOfConduct;

#[derive(StructOpt)]
#[structopt(author, about)]
struct Command {
    #[structopt(name = "repository")]
    repo: String,
}

#[derive(Deserialize, Debug)]
struct Env {
    github_api_token: String,
}

fn parse_repo_name(repo_name: &str) -> Result<(&str, &str), anyhow::Error> {
    let mut parts = repo_name.split('/');
    match (parts.next(), parts.next()) {
        (Some(owner), Some(name)) => Ok((owner, name)),
        _ => Err(format_err!("wrong format for the repository name param (we expect something like facebook/graphql)"))
    }
}

fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config: Env = envy::from_env().context("while reading from environment")?;

    let args = Command::from_args();

    let client = GraphQLClientImpl::new(config.github_api_token);

    let response_body = CodesOfConduct::execute(codes_of_conduct::Variables {}, &client)?;

    info!("{:?}", response_body);

    let repo = args.repo;
    let (owner, name) = parse_repo_name(&repo).unwrap_or(("tomhoule", "graphql-client"));

    let response_body = RepoView::execute(repo_view::Variables {
        owner: owner.to_string(),
        name: name.to_string(),
    }, &client)?;

    info!("{:?}", response_body);

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    }

    let response_data: repo_view::ResponseData = response_body.data.expect("missing response data");

    let stars: Option<i64> = response_data
        .repository
        .as_ref()
        .map(|repo| repo.stargazers.total_count);

    println!("{}/{} - 🌟 {}", owner, name, stars.unwrap_or(0),);

    let mut table = prettytable::Table::new();

    table.add_row(row!(b => "issue", "comments"));

    for issue in &response_data
        .repository
        .expect("missing repository")
        .issues
        .nodes
        .expect("issue nodes is null")
    {
        if let Some(issue) = issue {
            table.add_row(row!(issue.title, issue.comments.total_count));
        }
    }

    table.printstd();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_repo_name_works() {
        assert_eq!(
            parse_repo_name("graphql-rust/graphql-client").unwrap(),
            ("graphql-rust", "graphql-client")
        );
        assert!(parse_repo_name("abcd").is_err());
    }
}
