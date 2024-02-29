use yew::prelude::*;

use jasshaus_game::Game;
use crate::utils::card_path;
use crate::utils::get_player_pos;

pub fn get_carpet(game: &Game, id: u8) -> Html {
    let start = if game.history_index % 4 == 0 && game.history_index > 0 {
        game.history_index - 4
    } else {
        game.history_index / 4 * 4
    } as usize;

    let start_pos = if game.history_index % 4 == 0 && game.history_index > 0 {
        (get_player_pos(id, game.get_beginplayer()) + 4 - game.bestcard_index % 4) % 4
    } else {
        get_player_pos(id, game.get_beginplayer())
    } as usize;
    let cards = game.history_index as usize - start;

    html! {
        <div id="carpet">
            {
                (start..game.history_index as usize).map(|i| {
                    //let last = i == self.playedcards.len()-1;
                    let pos = (start_pos as usize + i) % 4;
                    let position = match pos {
                        0 => "bottom: 0;",
                        1 => "right: 0;",
                        2 => "top: 0;",
                        3 => "left: 0;",
                        _ => "display: none;"
                    };
                    let best = if i as u8 == game.bestcard_index { "border-style:solid;" } else {""};
                    let filter = if cards == 4 {
                        if i as u8 != game.bestcard_index { "filter: brightness(75%);" } else {""}
                    } else {""};

                    let style = format!("{}{}{}",position,best,filter);

                    let center = if pos & 1 != 0 { "CenterY" } else { "CenterX" };
                    let src = card_path( &game.history[i] );

                    html! {
                        <img class={format!("Playedcard {}", center)} {src} {style} />
                    }
                }).collect::<Vec<Html>>()
            }
        </div>
    }

}
