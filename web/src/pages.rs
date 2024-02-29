use yew_router::Routable;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/settings.html")]
    Settings,
    #[at("/game.html")]
    Game,
    #[not_found]
    #[at("/404.html")]
    NotFound,
}
