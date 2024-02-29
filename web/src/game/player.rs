use yew::prelude::*;

//use jasshaus_game::socket_message::*;
use crate::utils::card_path;
use jasshaus_game::card::*;

use gloo::timers::callback::Timeout;

pub struct Player {
    pub name: String,
    pub muted: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: String::from("???"),
            muted: false,
        }
    }
}

// ---

pub enum Msg {
    HideMSG,
    SetMSG(String),
    SetShows([Show; 3]),
    CloseShows,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub pos: u8,
    pub onturn: bool,
    pub name: String,
}

pub struct PlayerComponent {
    pub msg: String,
    pub timout: Option<Timeout>,
    pub shows: Vec<Show>,
}

impl Component for PlayerComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            msg: String::new(),
            timout: None,
            shows: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Msg::HideMSG => {
                self.timout = None;
                self.msg = String::new();
                true
            },
            Msg::SetMSG(msg) => {
                let link = ctx.link().clone();
                let timout = Timeout::new(5000, move || {
                    link.send_message(Msg::HideMSG);
                });
                self.timout = Some(timout);
                self.msg = msg;
                true
            },
            Msg::SetShows(shows) => {
                self.shows = shows.into_iter()
                    .filter(|s| s.color != 4)
                    .collect();
                true
            }
            Msg::CloseShows => {
                self.shows.clear();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let pos = ctx.props().pos;

        let shows = if self.shows.is_empty() {html!{}} else  {
            let style = match pos {
                1 => "bottom:100%;right:0;",
                2 => "top:100%;left:50%;transform:translateX(-50%);",
                3 => "bottom:100%;left:0;",
                _ => "bottom:25vh;left:50%;transform:translateX(-50%);",
            };
            let close = ctx.link().callback(move |_| Msg::CloseShows);

            html! {
                <div class="Shows" {style}>
                {
                    self.shows.iter()
                        .map(|s| html!{
                            <div class="Show">
                            {
                                s.as_cards().iter().map(|c| html! {
                                    <img src={card_path(c)}/>
                                }).collect::<Vec<Html>>()
                            }
                            </div>
                    }).collect::< Vec<Html> >()
                }
                    <div id="closeShows" onclick={close}>{"Schliessen"}</div>
                </div>
            }
        };

        if pos == 0 {
            let msg = {
                if self.msg.is_empty() {
                    html! { <></> }
                } else {
                    let link = ctx.link().clone();
                    let onclick = Callback::from(move |_| link.send_message(Msg::HideMSG));

                    html! {
                        <div class="PlayerMSG CenterX" {onclick} style={"bottom: 25vh;"}>
                            {self.msg.clone()}

                        </div>
                    }
                }
            };

            return html! {
                <>
                    {msg}
                    {shows}
                </>
            };
        }

        let center = if pos & 1 != 0 { "CenterY" } else { "CenterX" };
        let align = match pos {
            1 => "right: 2px",
            2 => "top: 2px",
            3 => "left: 2px",
            _ => "display: none",
        };

        let msg = {
            if self.msg.is_empty() {
                html! { <></> }
            } else {
                let align = match pos {
                    1 => "bottom: 100%; right: 0px;",
                    2 => "top: 100%; left: 50%; transform: translateX(-50%);",
                    3 => "bottom: 100%; left: 0px;",
                    _ => "display: none",
                };

                let link = ctx.link().clone();
                let onclick = Callback::from(move |_| link.send_message(Msg::HideMSG));

                html! { <div class="PlayerMSG" {onclick} style={align}>{self.msg.clone()}</div> }
            }
        };

        let star = if !ctx.props().onturn {html!{}} else {
            let style = format!("position: absolute;height:100%;{}", match pos {
                1|2 => "right: 100%;",
                3 => "left: 100%;",
                _ => "display: none;",
            });

            html! {
                <img {style} src="img/star.svg"/>
            }
        };

        html! {
            <div class={format!("Player {}", center)} style={align}>
                <a class="PlayerName">{ctx.props().name.clone()}</a>
                {star}
                {msg}
                {shows}
            </div>
        }
    }
}
