use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew_router::prelude::*;

mod pages;
mod utils;

mod default;
mod home;
mod settings;
mod game;

use pages::*;

rust_i18n::i18n!();

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <home::Home/> },
        Route::Game => html! { <game::GameComponent/> },
        Route::Settings => html! { <settings::SettingsPage /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
fn main() {
    rust_i18n::set_locale("de");
    yew::Renderer::<App>::new().render();
}
