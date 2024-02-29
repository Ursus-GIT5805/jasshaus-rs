use yew::prelude::*;

use jasshaus_game::{
    Game,
//    card::*,
};

pub mod round_details;
pub mod game_details;
pub mod carpet;
pub mod show;

pub fn get_game(game: &Game, names: Vec<String>, id: u8) -> Html {
    html! {
        <>
            {carpet::get_carpet(&game, id)}
            {round_details::get_round_details(&game.playtype)}
            {game_details::get_game_details(names, &game)}
        </>
    }
}
