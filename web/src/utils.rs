use yew::prelude::*;
use wasm_bindgen::JsValue;

use yew_router::Routable;
use crate::pages::Route;

use jasshaus_game::{
    playtype::*,
    card::*,
};

pub const PATH_MISERE: &str = "./img/playtype/misere.png";

pub fn pt_path(pt: u8) -> String {
    let src = match pt {
        UPDOWN => "updown.png",
        DOWNUP => "downup.png",
        SHIELD => "trumpfshield.png",
        ACORN => "trumpfacorn.png",
        ROSE => "trumpfrose.png",
        BELL => "trumpfbell.png",
        SLALOMUPDOWN => "slalomup.png",
        SLALOMDOWNUP => "slalomdown.png",
        GUSCHTI => "guschti.png",
        MARY => "mary.png",
        PASS => "pass.png",
        _ => "",
    };

    format!("./img/playtype/{}", src)
}

pub fn card_path(card: &Card) -> String {
    format!( "./img/de/{}{}.png", card.color, card.number )
}

/*pub fn card_name(card: &Card) -> String {
    let color = match card.color {
        0 => "Schilte",
        1 => "Eichle",
        2 => "Rose",
        3 => "Schelle",
        _ => "",
    };
    let number = match card.number {
        0 => "6",
        1 => "7",
        2 => "8",
        3 => "9",
        4 => "Banner",
        5 => "Under",
        6 => "Ober",
        7 => "König",
        8 => "Ass",
        _ => "",
    };
    format!("{} {}", color, number)
}*/

pub fn pt_name(pt: u8, misere: bool) -> String {
    let plt = match pt {
        UPDOWN => "Obenabe",
        DOWNUP => "Undeufe",
        SHIELD => "Trumpf Schilte",
        ACORN => "Trumpf Eichle",
        ROSE => "Trumpf Rose",
        BELL => "Trumpf Schelle",
        SLALOMUPDOWN => "Slalom Obenabe",
        SLALOMDOWNUP => "Slalom Undeufe",
        GUSCHTI => "Guschti",
        MARY => "Mary",
        PASS => "Schieben",
        _ => "",
    };
    let mis = if misere { "Misère:" } else { "" };
    format!("{} {}", mis, plt)
}

pub fn get_player_pos(host_id: u8, id: u8) -> u8 {
    (id + 4 - host_id) % 4
}

#[allow(dead_code)]
pub fn log<'a>(msg: &'a str){
    #[cfg(debug_assertions)]
    gloo::console::log!(JsValue::from(msg));
}

#[allow(dead_code)]
pub fn error<'a>(msg: &'a str){
    #[cfg(debug_assertions)]
    gloo::console::error!(JsValue::from(msg));
}

pub fn go_to<IN>(route: Route) -> Callback<IN> {
    Callback::from(move |_| {
        let window = web_sys::window().expect("Could not retrieve window");
        window.location().set_href(route.to_path().as_str()).expect("Failed to set location");
    })
}
