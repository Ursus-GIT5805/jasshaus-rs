use yew::prelude::*;

use rust_i18n::t;

use crate::utils::*;
use crate::pages::Route;
use crate::default::Defaultpage;

#[function_component]
pub fn Home() -> Html {
    let own = html! {
        <div>
            <h1>{t!("welcome")}</h1>
            <div id="buttonlist">
                <button class="Button" onclick={go_to(Route::Game)}>{t!("goto_game")}</button>
                <button class="Button" onclick={go_to(Route::Settings)}>{t!("goto_settings")}</button>
            </div>
        </div>
    };

    html! {
        <Defaultpage html={own}/>
    }
}
