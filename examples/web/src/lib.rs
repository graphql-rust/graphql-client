use graphql_client::{web, GraphQLQuery, Response};
use lazy_static::*;
use std::{cell::RefCell, sync::Mutex};
use wasm_bindgen::{prelude::*, JsCast};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/puppy_smiles.graphql",
    response_derives = "Debug"
)]
struct PuppySmiles;

fn log(s: &str) {
    web_sys::console::log_1(&JsValue::from_str(s))
}

lazy_static! {
    static ref LAST_ENTRY: Mutex<RefCell<Option<String>>> = Mutex::new(RefCell::new(None));
}

async fn load_more() -> Result<JsValue, JsValue> {
    let client = web::Client::new("https://www.graphqlhub.com/graphql");
    let variables = puppy_smiles::Variables {
        after: LAST_ENTRY
            .lock()
            .ok()
            .and_then(|opt| opt.borrow().to_owned()),
    };
    let response = client.call(PuppySmiles, variables).await.map_err(|err| {
        log(&format!(
            "Could not fetch puppies. graphql_client_web error: {:?}",
            err
        ));
        JsValue::NULL
    })?;

    render_response(response);
    Ok(JsValue::NULL)
}

fn document() -> web_sys::Document {
    web_sys::window()
        .expect_throw("no window")
        .document()
        .expect_throw("no document")
}

fn add_load_more_button() {
    let btn = document()
        .create_element("button")
        .expect_throw("could not create button");
    btn.set_inner_html("I WANT MORE PUPPIES");
    let on_click = Closure::wrap(
        Box::new(move || {
            wasm_bindgen_futures::spawn_local(async {
                let _ = load_more().await;
            });
            JsValue::NULL
        }) as Box<dyn FnMut() -> JsValue>, // Box::new(move || future_to_promise(load_more().boxed())) as Box<dyn FnMut() -> js_sys::Promise>
    );
    btn.add_event_listener_with_callback(
        "click",
        &on_click
            .as_ref()
            .dyn_ref()
            .expect_throw("on click is not a Function"),
    )
    .expect_throw("could not add event listener to load more button");

    let doc = document().body().expect_throw("no body");
    doc.append_child(&btn)
        .expect_throw("could not append button");

    on_click.forget();
}

fn render_response(response: Response<puppy_smiles::ResponseData>) {
    use std::fmt::Write;

    log(&format!("response body\n\n{:?}", response));

    let parent = document().body().expect_throw("no body");

    let json: Response<puppy_smiles::ResponseData> = response;
    let response = document()
        .create_element("div")
        .expect_throw("could not create div");
    let mut inner_html = String::new();
    let listings = json
        .data
        .expect_throw("response data")
        .reddit
        .expect_throw("reddit")
        .subreddit
        .expect_throw("puppy smiles subreddit")
        .new_listings;

    let new_cursor: Option<String> = listings[listings.len() - 1]
        .as_ref()
        .map(|puppy| puppy.fullname_id.clone());
    LAST_ENTRY.lock().unwrap_throw().replace(new_cursor);

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
            )
            .expect_throw("write to string");
        }
    }
    response.set_inner_html(&format!(
        "<h2>response:</h2><div class=\"container\"><div class=\"row\">{}</div></div>",
        inner_html
    ));
    parent
        .append_child(&response)
        .expect_throw("could not append response");
}

#[wasm_bindgen(start)]
pub fn run() {
    log("Hello there");
    let message_area = document()
        .create_element("div")
        .expect_throw("could not create div");
    message_area.set_inner_html("<p>good morning</p>");
    let parent = document().body().unwrap_throw();
    parent
        .append_child(&message_area)
        .expect_throw("could not append message area");

    add_load_more_button();

    log("Bye");
}
