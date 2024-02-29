use yew::prelude::*;

use jasshaus_game::playtype::*;
use jasshaus_comm::socket_message::SocketMessage;

use crate::game::ws::WebSocket;
use crate::utils::{pt_path, pt_name, PATH_MISERE};

pub enum Msg {
    ToggleMisere,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub passed: bool,
    pub ws: WebSocket,
}

pub struct Announce {
    misere: bool,
}

impl Component for Announce {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            misere: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleMisere => {
                self.misere = !self.misere;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let filter = if self.misere { "filter: invert(100%);" } else { "" };

        let misere = {
            let link = ctx.link().clone();
            let onclick = Callback::from(move |_| {
                link.send_message(Msg::ToggleMisere);
            });

            html! {
                <div class="AnnounceButton" {onclick}>
                    <img src={PATH_MISERE}/>
                    <p>{"Misère"}</p>
                </div>
            }
        };

        let passed = ctx.props().passed;

        html! {
            <div id="announceWindow" style={filter}>
                <h1>{"Sage an!"}</h1>
                <div id="playtypeBoard">
                    <div class="PTRow">
                        {vec![SLALOMUPDOWN, UPDOWN, DOWNUP, SLALOMDOWNUP].iter().map(|pt| {
                            self.pt_button(*pt, false, ctx)
                        }).collect::<Vec<Html>>()}
                    </div>
                    <div class="PTRow">
                        {vec![SHIELD, ACORN, ROSE, BELL].iter().map(|pt| {
                            self.pt_button(*pt, false, ctx)
                        }).collect::<Vec<Html>>()}
                    </div>
                    <div class="PTRow">
                        {misere}
                        {self.pt_button(GUSCHTI, false, ctx)}
                        {self.pt_button(MARY, false, ctx)}
                        {self.pt_button(PASS, passed, ctx)}
                    </div>
                </div>
            </div>
        }
    }
}

impl Announce {
    fn pt_button(&self, pt: u8, hide: bool, ctx: &Context<Self>) -> Html {
        let plt = Playtype {
            playtype: pt,
            ruletype: pt,
            misere: self.misere,
            passed: ctx.props().passed,
        };

        let ws_clone = ctx.props().ws.clone();
        let onclick = Callback::from(move |_| ws_clone.send(SocketMessage::Announce(plt)) );

        let style = if hide { "visibility: hidden;" } else { "" };

        html! {
            <div class="AnnounceButton" {onclick} {style}>
                <img src={pt_path(pt)}/>
                <p>{pt_name(pt, false)}</p>
            </div>
        }
    }
}
