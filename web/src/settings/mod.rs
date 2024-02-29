use yew::prelude::*;
use wasm_bindgen::JsCast;

use serde::{Serialize, Deserialize};

use rust_i18n::t;

use jasshaus_traits::*;
use jasshaus_macros::YewSetting;

use crate::utils::*;
use crate::pages::Route;
use crate::default::Defaultpage;

#[derive(Default, PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(YewSetting)]
pub enum CardSkin {
    #[default]
    #[label( t!("german") )]
    German,

    #[label( t!("french") )]
    French,
}

#[derive(Default, PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(YewSetting)]
pub struct ClientSettings {
    #[label( t!("settings/label/name") )]
    pub name: String,

    #[label( t!("settings/label/click_cards") )]
    pub click_cards: bool,

    #[label( t!("settings/label/cardskin") )]
    pub cardskin: CardSkin,

    // #[label( t!("settings/label/disable_animations") )]
    // pub disable_animations: bool,
}

pub enum SettingsMsg {
    Update(ClientSettings)
}

pub struct SettingsPage {
    settings: ClientSettings,
}

impl Component for SettingsPage {
    type Message = SettingsMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
	Self {
	    settings: ClientSettings::default(),
	}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
	match msg {
	    SettingsMsg::Update(s) => {
		self.settings = s;
		false
	    }
	}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
	let link = ctx.link().clone();
	let onchange = Callback::from(move |s| {
	    link.send_message( SettingsMsg::Update(s) );
	});

	let html = html! {
	    <div style={"align-items: left;"}>
		<button class="Button" onclick={go_to(Route::Home)}>{t!("goto_home")}</button>
		<h2>{t!("settings")}</h2>
		<YewSettingsForm<ClientSettings> {onchange}/>
		</div>
	};

	html! {
	    <Defaultpage {html} />
	}
    }
}
