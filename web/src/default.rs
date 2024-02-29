use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HtmlProp {
    pub html: Html,
}

#[function_component(Defaultpage)]
pub fn default_template(prop: &HtmlProp) -> Html {
    html! {
        <>
            <link rel="stylesheet" href="./styles/index.css"/>
            <img id="backimg" src="img/background.jpg"/>

            <div id="mainWindow">
                <div id="title">{"Jasshaus"}</div>
                { prop.html.clone() }
            </div>
        </>
    }
}
