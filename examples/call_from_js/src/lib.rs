#![feature(wasm_custom_section, wasm_import_module, use_extern_macros)]

#[macro_use]
extern crate graphql_client;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;
use std::sync::Mutex;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/puppy_smiles.graphql"
)]
struct PuppySmiles;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;
    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    type Element;
    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html: &str);
    #[wasm_bindgen(method, js_name = appendChild)]
    fn append_child(this: &Element, other: Element);
    #[wasm_bindgen(method, js_name = addEventListener)]
    fn add_event_listener(this: &Element, event: &str, cb: &Closure<Fn()>);
}

#[wasm_bindgen(module = "./convenient_fetch")]
extern "C" {
    fn convenient_post(
        req: &str,
        body: String,
        on_complete: &Closure<Fn(String)>,
        on_error: &Closure<Fn()>,
    );
}

lazy_static! {
    static ref LAST_ENTRY: Mutex<RefCell<Option<String>>> = Mutex::new(RefCell::new(None));
}

fn load_more() {
    let cb = cb();
    let on_error = on_error();
    convenient_post(
        "https://www.graphqlhub.com/graphql",
        serde_json::to_string(&PuppySmiles::build_query(puppy_smiles::Variables {
            after: LAST_ENTRY
                .lock()
                .ok()
                .and_then(|opt| opt.borrow().to_owned()),
        })).unwrap(),
        &cb,
        &on_error,
    );

    cb.forget();
    on_error.forget();
}

fn add_load_more_button() {
    let btn = document.createElement("button");
    btn.set_inner_html("I WANT MORE PUPPIES");
    let on_click = Closure::new(move || load_more());
    btn.add_event_listener("click", &on_click);

    let doc = document.body();
    doc.append_child(btn);

    on_click.forget();
}

fn cb() -> Closure<Fn(String)> {
    use std::fmt::Write;

    Closure::new(move |s: String| {
        log(&format!("response body\n\n{}", s));

        let parent = document.body();

        let json: Response<puppy_smiles::ResponseData> =
            serde_json::from_str(&s).expect("failed to deserialize");
        let response = document.createElement("div");
        let mut inner_html = String::new();
        let listings = json
            .data
            .expect("response data")
            .reddit
            .expect("reddit")
            .subreddit
            .expect("puppy smiles subreddit")
            .new_listings;

        let new_cursor: Option<String> = listings[listings.len() - 1]
            .as_ref()
            .map(|puppy| puppy.fullname_id.clone())
            .to_owned();
        LAST_ENTRY.lock().unwrap().replace(new_cursor);

        for puppy in &listings {
            if let Some(puppy) = puppy {
                write!(
                    inner_html,
                    r#"
                    <div class="card" style="width: 26rem;">
                        <img class="img-thumbnail card-img-top" alt="{}" src="{}" />
                        <div class="card-body">
                            <h5 class="card-title">{}</h5>
                        </div>
                    </div>
                    "#,
                    puppy.title, puppy.url, puppy.title
                ).expect("write to string");
            }
        }
        response.set_inner_html(&format!(
            "<h2>response:</h2><div class=\"container\"><div class=\"row\">{}</div></div>",
            inner_html
        ));
        parent.append_child(response);
    })
}

fn on_error() -> Closure<Fn()> {
    Closure::new(|| log("sad :("))
}

#[wasm_bindgen]
pub fn run() {
    log("Hello there");
    let message_area = document.createElement("div");
    message_area.set_inner_html("<p>good morning</p>");
    let parent = document.body();
    parent.append_child(message_area);

    load_more();
    add_load_more_button();

    log("Bye");
}
