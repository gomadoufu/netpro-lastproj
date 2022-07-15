use seed::{prelude::*, *};
use std::borrow::Cow;
use std::mem;
use web_sys::{self, FormData};

pub const TITLE: &str = "リアクションペーパー 📝りあぺぱ📝";
pub const DESCRIPTION: &str = "名前と学籍番号、今日の授業の感想を入力してください❗️";
pub const THANKYOU: &str = "ご回答ありがとうございました🐈🦀";

fn get_request_url() -> impl Into<Cow<'static, str>> {
    "/api/form"
}

// ------ ------
//     Model
// ------ ------

#[derive(Default, Debug)]
pub struct Form {
    name: String,
    id: String,
    contents: String,
}

impl Form {
    fn to_form_data(&self) -> Result<web_sys::FormData, JsValue> {
        let form_data = web_sys::FormData::new()?;
        form_data.append_with_str("name", &self.name)?;
        form_data.append_with_str("id", &self.id)?;
        form_data.append_with_str("contents", &self.contents)?;
        Ok(form_data)
    }
}

pub enum Model {
    ReadyToSubmit(Form),
    WaitingForResponse(Form),
    Finish,
}

impl Default for Model {
    fn default() -> Self {
        Self::ReadyToSubmit(Form {
            name: "".into(),
            id: "".into(),
            contents: "".into(),
        })
    }
}

impl Model {
    const fn form(&self) -> &Form {
        match self {
            Self::ReadyToSubmit(form) | Self::WaitingForResponse(form) => form,
            Self::Finish => unreachable!(),
        }
    }
    fn form_mut(&mut self) -> &mut Form {
        match self {
            Self::ReadyToSubmit(form) | Self::WaitingForResponse(form) => form,
            Self::Finish => unreachable!(),
        }
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    NameChanged(String),
    IdChanged(String),
    ContentsChanged(String),
    FormSubmitted,
    ServerResponded(fetch::Result<String>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::NameChanged(name) => model.form_mut().name = name,
        Msg::IdChanged(id) => model.form_mut().id = id,
        Msg::ContentsChanged(contents) => model.form_mut().contents = contents,
        Msg::FormSubmitted => {
            let form = mem::take(model.form_mut());
            let form_data = form.to_form_data().expect("create from data from form");
            orders.perform_cmd(async { Msg::ServerResponded(send_request(form_data).await) });
            *model = Model::WaitingForResponse(form);
            log!("form submitted");
        }
        //サーバーから返答があったとき
        Msg::ServerResponded(Ok(_)) => {
            *model = Model::Finish;
            clear_file_input();
        }
        Msg::ServerResponded(Err(fetch_error)) => {
            *model = Model::ReadyToSubmit(mem::take(model.form_mut()));
            error!("Request failed!", fetch_error);
        }
    }
}

async fn send_request(form: FormData) -> fetch::Result<String> {
    Request::new(get_request_url())
        .method(fetch::Method::Post)
        .body(JsValue::from(form))
        .fetch()
        .await?
        .text()
        .await
}

#[allow(clippy::option_map_unit_fn)]
fn clear_file_input() {
    seed::document()
        .get_element_by_id("form-file")
        .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|file_input| {
            // Note: `file_input.set_files(None)` doesn't work
            file_input.set_value("");
        });
}

// ------ ------
//     View
// ------ ------

fn view_form_field(mut label: Node<Msg>, control: Node<Msg>) -> Node<Msg> {
    label.add_style("margin-right", unit!(7, px));
    div![
        style! {
          "margin-bottom" => unit!(7, px),
          "display" => "flex",
        },
        label,
        control
    ]
}

pub fn view(model: &Model, intro: impl FnOnce(&str, &str) -> Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    if let Model::Finish = model {
        return nodes![intro(TITLE, THANKYOU)];
    }

    let btn_enabled = matches!(model, Model::ReadyToSubmit(form) if !form.name.is_empty() && !form.contents.is_empty() && !form.id.is_empty());

    let form = form![
        style! {
            St::Display => "flex",
            St::FlexDirection => "column",
        },
        ev(Ev::Submit, move |event| {
            event.prevent_default();
            Msg::FormSubmitted
        }),
        view_form_field(
            label!["氏名:", attrs! {At::For => "user-name" }],
            input![
                input_ev(Ev::Input, Msg::NameChanged),
                attrs! {
                    At::Id => "user-name",
                    At::Value => model.form().name,
                    At::Required => true.as_at_value(),
                    At::Rows => 1,
                }
            ]
        ),
        view_form_field(
            label!["学籍番号:", attrs! {At::For => "user-id" }],
            textarea![
                input_ev(Ev::Input, Msg::IdChanged),
                attrs! {
                    At::Id => "user-id",
                    At::Value => model.form().id,
                    At::Rows => 1,
                },
            ],
        ),
        view_form_field(
            label!["", attrs! {At::For => "user-contents" }],
            textarea![
                input_ev(Ev::Input, Msg::ContentsChanged),
                attrs! {
                    At::Id => "user-contents",
                    At::Value => model.form().contents,
                },
            ],
        ),
        button![
            style! {
                "padding" => format!("{} {}", px(2), px(12)),
                "background-color" => if btn_enabled { CSSValue::from("aquamarine") } else { CSSValue::Ignored },
            },
            attrs! {At::Disabled => not(btn_enabled).as_at_value()},
            "提出✔︎"
        ]
    ];

    nodes![intro(TITLE, DESCRIPTION), form]
}
