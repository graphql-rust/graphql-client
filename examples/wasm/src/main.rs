extern crate failure;
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
    link: ComponentLink<Model>,
    search: String,
    response: Option<GraphQLResponse<requests::station_query::ResponseData>>,
}

enum Msg {
    Edit(String),
    Noop,
    Receive(Option<GraphQLResponse<requests::station_query::ResponseData>>),
    Submit,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            _console: ConsoleService::new(),
            fetch: FetchService::new(),
            link,
            search: String::new(),
            response: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Noop => (),
            Msg::Receive(rcv) => self.response = rcv,
            Msg::Edit(s) => self.search = s,
            Msg::Submit => {
                let body =
                    requests::StationQuery::build_query(requests::station_query::Variables {
                        searchTerm: self.search.clone(),
                    });
                let request = ::http::Request::post("https://api.deutschebahn.com/1bahnql/")
                    .header("Accept", "application/json")
                    .header("Content-Type", "application/json")
                    .header("Authorization", "Bearer 59ea201e2a09a99126edad345f7cd1f0")
                    .body(Json(&body))
                    .expect("failed to build request");
                let callback = self.link.send_back(
                    |response: http::Response<
                        Json<
                            Result<
                                GraphQLResponse<requests::station_query::ResponseData>,
                                failure::Error,
                            >,
                        >,
                    >| {
                        let (meta, Json(data)) = response.into_parts();
                        if meta.status.is_success() {
                            Msg::Noop
                        } else {
                            Msg::Receive(data.ok())
                        }
                    },
                );
                self.fetch.fetch(request, callback);
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
                    <div>
                        {"Search for a train station: "}
                        <input
                            onchange=|changedata| match changedata { ChangeData::Value(v) => Msg::Edit(v), _ => unreachable!() }
                        ,/>
                        <button onclick=|_| Msg::Submit,>{"Go!"}</button>
                    </div>
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
