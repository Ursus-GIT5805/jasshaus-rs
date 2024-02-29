use yew::prelude::*;
use std::collections::VecDeque;

use wasm_bindgen::JsCast;

pub const MAX_MESSAGES: usize = 512;

#[derive(PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum ChatMessageType {
    Normal(usize),
    Yours,
    Info,
    System,
    Error,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub history: VecDeque<(String,ChatMessageType)>,
    pub ontext: Callback< String >,
    pub player_names: Vec<String>,
}

pub struct Chat {}

impl Component for Chat {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let call = ctx.props().ontext.clone();
        let onkeydown = Callback::from(move |e: web_sys::KeyboardEvent| {
            if e.key_code() == 13 {
                let ele: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                let s = ele.value();
                call.emit(s);
                ele.set_value("");
            }
        });

	let players = &ctx.props().player_names;

        html! {
            <>
                <div id="ChatWindow">
                    <input id="chatInput" type="text" placeholder="Eingabe" {onkeydown}/>
                    <div>
                    {
                        ctx.props().history.iter().map(|(text, ty)| {
                            let style = match ty {
                                ChatMessageType::Yours => "text-align: right",
                                ChatMessageType::Info => "color: #FFFF00",
                                ChatMessageType::Error => "color: #FF0000",
				ChatMessageType::System => "color: #AAAAAA",
                                _ => "",
                            };



			    let msg = match ty {
				ChatMessageType::Normal(plr) => format!("[{}]: {}", players[*plr].clone(), text),
				_ => text.clone(),
			    };

                            html! {
				<div {style}>{msg}</div>
			    }
                        }).collect::<Vec<Html>>()
                    }
                    </div>
                </div>
            </>
        }
    }
}
