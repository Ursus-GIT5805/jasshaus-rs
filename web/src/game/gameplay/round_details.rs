use yew::prelude::*;

use crate::utils::{pt_path, pt_name};
use jasshaus_game::playtype::*;

pub fn get_round_details(pt: &Playtype) -> Html {
    fn visible(statement: bool) -> &'static str {
        if statement { "visibility: visible;" }
        else { "visibility: hidden;" }
    }

    let def: Html = html! { <img src="./img/playtype/updown.png" style="visibility: hidden;"/> };

    let filter = if pt.misere { "filter: invert(100%);" } else {""};
    let playtype = {
        if pt.playtype == NONE { def.clone() }
        else {
            html! { <img src={ pt_path(pt.playtype) } style={filter}/> }
        }
    };

    let ruletype = match pt.playtype {
        SLALOMUPDOWN | SLALOMDOWNUP |
        GUSCHTI | MARY => {
            html! { <img src={ pt_path(pt.ruletype)} style={filter}/> }
        }
        _ => def.clone(),
    };

    html! {
        <div id="roundDetails">
            <div>{pt_name(pt.playtype, pt.misere)}</div>
            <div id="roundSymbols">
                <img src="./img/playtype/misere.png" style={format!("{}{}",visible(pt.misere),filter)}/>
                {playtype}
                <div id="roundSymbolsVertical">
                    {ruletype}
                    <img src="./img/playtype/pass.png" style={format!("{}{}",visible(pt.passed),filter)}/>
                </div>
            </div>
        </div>
    }
}
