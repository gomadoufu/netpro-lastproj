#![allow(clippy::enum_variant_names, clippy::large_enum_variant)]

use seed::{prelude::*, *};

mod app_home;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    app_home: app_home::Model,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    AppHome(app_home::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AppHome(msg) => {
            app_home::update(msg, &mut model.app_home, &mut orders.proxy(Msg::AppHome));
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        style! {
            St::FontFamily => "sans-serif";
            St::MaxWidth => px(460);
            St::Margin => "auto";
        },
        app_home::view(&model.app_home, view_intro).map_msg(Msg::AppHome),
    ]
}

fn view_intro<Ms>(title: &str, description: &str) -> Vec<Node<Ms>> {
    vec![
        hr![],
        h2![title],
        div![style! {St::MarginBottom => px(15)}, description],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
