use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub names: Vec< String >,
    pub points: [u16; 2],
    pub won: bool,
}

pub struct End {}

impl Component for End {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = &ctx.props();

        let title = if props.won { "Gewonnen" } else { "Verloren" };

        let names: Vec<String> = ctx.props().names.iter().map(|s| {
            let mut a = s.clone();
            a.truncate(3);
            a
        }).collect();

        html! {
            <div id="endWindow">
                <div style="font-size: 3rem; font-weight: bolder;">{title}</div>
                <div style="font-size: 2.5rem; padding: 1rem;">
                    {
                        format!("{} + {} | {} - {} | {} + {}",
                            names[0], names[2], ctx.props().points[0],
                            ctx.props().points[1],names[1], names[3])
                    }
                </div>
            </div>
        }
    }
}
