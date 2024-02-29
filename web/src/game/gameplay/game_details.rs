use yew::prelude::*;

use jasshaus_game::Game;

pub fn get_game_details(names: Vec<String>, game: &Game) -> Html {
    html! {
        <div id="gameDetails">
            <a><u>{format!("Schieber {}", game.setting.max_points)}</u></a><br/>
            {
                (0..2).map(|i| {
                    let mut name1 = names[i].clone();
                    name1.truncate(3);
                    let mut name2 = names[i+2].clone();
                    name2.truncate(3);
                    let got = game.won_points[i] + game.show_points[i];
                    let bef = game.points[i] - got;

                    html! {
                        <>
                            <a>{format!("{} + {}: {} + {}", name1, name2, bef, got)}</a>
                            <br/>
                        </>
                    }
                }).collect::<Vec<Html>>()
            }
        </div>
    }
}
