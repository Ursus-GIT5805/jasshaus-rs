use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub connected: bool,
}

impl Player {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            name: String::from("???"),
            connected: false,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum PlayerMSG {
    None,
    Text(String),
    Html(Html),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub player: Player,
    pub style: String,
    pub msg: PlayerMSG,
    pub free: [bool; 4], //top, right, down, left
    pub display_name: bool,
    pub onmsg_click: Callback<web_sys::MouseEvent>,
    //pub stream: web_sys::MediaStream,
}

pub struct Msg {}

pub struct PlayerComponent {}

impl Component for PlayerComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context< Self >) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        fn poke_button(text: &'static str) -> Html {
            let onclick = Callback::from(move |_| {
                crate::utils::log(text);
            });
            html! { <div {onclick}>{text}</div> }
        }

	let onclick = ctx.props().onmsg_click.clone();
        let msg = match &ctx.props().msg {
            PlayerMSG::Text(text) => {
                html! {
                    <div class="PlayerMSG" {onclick}>
                        {text.clone()}
                    </div>
                }
            },
	    PlayerMSG::Html(html) => {
		html! {
		    <div class="PlayerMSG" {onclick}>
		    {html.clone()}
		    </div>
		}
	    }
            _ => html!{},
        };

        let settings = {
            let onclick = Callback::from(move |_| {
                crate::utils::log("Einstellungen");
            });
            html! { <div {onclick}>{"Einstellungen"}</div> }
        };

        let player = if ctx.props().display_name {
            html! {
                <div class="Player">
                    <a class="PlayerName">{ctx.props().player.name.clone()}</a>
                    <div class="PlayerDropdown">
                        {poke_button("Wach auf!")}
                        {poke_button("Gut gespielt!")}
                        {settings}
                    </div>
                </div>
            }
        } else { html! {} };

        let free = &ctx.props().free;
        let mut style = String::new();
        if !free[1] { style.push_str("align-items: start;"); }
        if !free[3] { style.push_str("align-items: end;"); }
        if free[1] && free[3] { style.push_str("align-items: center;"); }
        if !free[2] { style.push_str("flex-direction: reverse-column;") }
        else { style.push_str("flex-direction: column;") }
        style.push_str( &ctx.props().style );

        html! {
            <div class="PlayerContainer" {style}>
                {player}
                {msg}
            </div>
        }
    }
}
