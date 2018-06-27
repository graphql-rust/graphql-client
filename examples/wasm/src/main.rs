#[macro_use]
extern crate graphql_client;
extern crate stdweb;
#[macro_use]
extern crate yew;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod requests;

use yew::prelude::*;
use yew::services::ConsoleService;

struct Model {
    console: ConsoleService,
    search: String,
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
            console: ConsoleService::new(),
            search: String::new(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
