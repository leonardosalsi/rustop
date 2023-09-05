//https://github.com/seed-rs/seed-quickstart
#![allow(clippy::wildcard_imports)]


use seed::{prelude::*, *};
use sysinfo::{System, SystemExt};


// ------ ------
//     Init
// ------ ------
fn ttt() {
    let mut sys = System::new_all();
    sys.refresh_all();
    log("=> disks:");
    log(sys.disks());
    log("Hello, worlwd!");
}
// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    ttt();
    
    Model { counter: 0 }
    
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counter: i32,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(_model: &Model) -> Node<Msg> {
    table![
        tr![
            td!["Row 1, Column 1"],
            td!["Row 1, Column 2"],
        ],
        tr![
            td!["Row 2, Column 1"],
            td!["Row 2, Column 2"],
        ],
        tr![
            td!["Row 3, Column 1"],
            td!["Row 3, Column 2"],
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
