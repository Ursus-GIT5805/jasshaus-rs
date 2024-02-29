use yew::prelude::*;
use jasshaus_comm::socket_message::*;
use crate::game::ws::WebSocket;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: usize,
    pub ws: WebSocket,
}

pub enum Msg {
    SetMate(u8),
}

pub struct Mate {
    chosen_mate: Option<u8>,
}

impl Component for Mate {
    type Message = Msg;
    type Properties = Props;

     fn create(ctx: &Context<Self>) -> Self {
        ctx.props().ws.send( SocketMessage::Mate( ctx.props().id as u8 ) );
        Self { chosen_mate: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetMate(plr) => {
                self.chosen_mate = Some(plr);
                ctx.props().ws.send( SocketMessage::Mate( (ctx.props().id as u8 + plr) % 4 ) );
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut plrs: std::vec::Vec<Html> = vec![];

        for i in 0..4 {
            let c = ctx.clone().link().clone();
            let onclick = Callback::from(move |_| {
                c.send_message(Msg::SetMate(i));
                //TODO Send message
            });

            let color = {
                if self.chosen_mate.is_none() { "" }
                else {
                    if self.chosen_mate.unwrap() == i { "background-color: #00CC00 !important;" }
                    else { "background-color: #CC0000 !important;" }
                }
            };
            let dir = match i {
                0 => "bottom: 0;",
                1 => "right: 0;",
                2 => "top: 0;",
                3 => "left: 0;",
                _ => ""
            };
            let class = {
                let center = {
                    if i & 1 == 0 { "CenterX" }
                    else { "CenterY" }
                };
                format!("MateButton {}", center)
            };

            let style = format!("{}{}",dir,color);
            plrs.push( html! { <div {class} {style} {onclick}>{"???"}</div> } )
        }

        html! {
            <div id="mateWindow" class="CenterXY">
                <h2>{"Wählen Sie ihren Wunschpartner"}</h2>
                <div id="mateChoose">{ plrs }</div>
            </div>
        }
    }
}
