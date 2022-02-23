use wasm_bindgen::JsCast;
use simplex::evaluator::Evaluator;
use yew::prelude::*;
use web_sys::{HtmlTextAreaElement};

pub enum Msg {
    SetInput(String),
    Submit
}

pub struct App {
    input: String,
    output: String,
    evaluator: Evaluator
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { input: "".to_string(), output: "".to_string(), evaluator: Evaluator::new() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetInput(s) => {
                self.input = s;
                true
            },
            Msg::Submit => {
                match self.evaluator.eval(&self.input) {
                    Ok(s) => {
                        self.output = s.to_string();
                    },
                    Err(e) => {
                        self.output = format!("{:#?}", e);
                    }
                }
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "Simplex Playground" }</h1>
                <div>
                    <textarea id="input" value={self.input.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                        let target = e.target().unwrap().dyn_into::<HtmlTextAreaElement>();
                        let value = match target {
                            Ok(text_area) => text_area.value(),
                            Err(_) => panic!(),
                        };
                        Msg::SetInput(value)
                    } )} />
                    <button type="button" onclick={ctx.link().callback(|_| { Msg::Submit })}>{ "Submit" }</button>
                    <textarea id="output" readonly=true value={self.output.clone()} />
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}