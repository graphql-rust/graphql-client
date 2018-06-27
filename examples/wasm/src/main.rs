#[macro_use]
extern crate graphql_client;
extern crate http;
extern crate stdweb;
#[macro_use]
extern crate yew;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod requests;

use graphql_client::{GraphQLQuery, GraphQLResponse};
use yew::format::Json;
use yew::prelude::*;
use yew::services::{ConsoleService, FetchService};

struct Model {
    _console: ConsoleService,
    fetch: FetchService,
    search: String,
    response: Option<GraphQLResponse<requests::station_query::ResponseData>>,
}

enum Msg {
    Edit(String),
    Submit,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            _console: ConsoleService::new(),
            fetch: FetchService::new(),
            search: String::new(),
            response: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Edit(s) => self.search = s,
            Msg::Submit => {
                let body =
                    requests::StationQuery::build_query(requests::station_query::Variables {
                        searchTerm: self.search.clone(),
                    });
                let request = ::http::Request::post("https://bahnql.herokuapp.com/graphql")
                    .header("Content-Type", "application/json")
                    .body(Json(body))
                    .expect("failed to build request");
                self.fetch.fetch(request, |res| self.response = res);
            }
        }

        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "Hello WebAssembly" }</h1>
                <div>
                  <p>
                    {"With data from the "}
                    <a href="https://bahnql.herokuapp.com/graphql",>
                        {"Public Deutsche Bahn GraphQL API"}
                    </a>
                  </p>
                  <p>
                    <form>
                        {"Search for a train station: "}
                        <input
                            onchange=|changedata| match changedata { ChangeData::Value(v) => Msg::Edit(v), _ => unreachable!() }
                        ,/>
                        <button role="submit", onclick=|_| Msg::Submit,>{"Go!"}</button>
                    </form>
                  </p>
                </div>
                <div>
                    {serde_json::to_string(&requests::StationQuery::build_query(requests::station_query::Variables {
                        searchTerm: self.search.clone(),

                    })).unwrap()}
                </div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
