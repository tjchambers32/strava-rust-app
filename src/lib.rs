// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{ prelude::*, * };
use chrono::{ DateTime, Utc };
use chrono::format::ParseError;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { 
        counter: 0,
        runners: vec!["Kelsey".to_string(), "Travis".to_string(), "Rocky".to_string(), "Ava".to_string(), "Stella".to_string()],
        strava_runners: vec![StravaRunner{id: 1, name: "Kelsey".to_string()}, StravaRunner{id: 2, name: "Travis".to_string()}],
        stats: Vec::new()
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counter: i32,
    runners: Vec<String>,
    strava_runners: Vec<StravaRunner>,
    stats: Vec<Stat>,
}

struct Stat {
    runs: Vec<Run>,
}

struct Run {
    distance: f64,
    date: DateTime::<Utc>,
}

struct StravaRunner {
    id: i32,
    name: String,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
    UpdateStats,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        // TODO: actually call strava APIs for defined strava athletes
        Msg::UpdateStats => model.stats = update_stats(model.runners, model.strava_runners),
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            "These are our athletes: ",
            C!["runners"],
            span![model.runners.join(", ")]
        ],
        div![
            button!["Update", ev(Ev::Click, |_| Msg::UpdateStats),],
        ] 
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("strava-app", init, update, view);
}