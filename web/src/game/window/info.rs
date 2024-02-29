use yew::prelude::*;
//use yew::html::Scope;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub info: &'static str,
}

#[function_component]
pub fn Info(props: &Props) -> Html {
    html! {
        <div id="infoWindow">
            <h1><u>{"Info"}</u></h1>
            <a style="padding: 0.5rem;">{props.info.clone()}</a>
            <div id="infoButton">{"Okay"}</div>
        </div>
    }
}
