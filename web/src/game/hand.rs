use yew::prelude::*;

use std::vec::Vec;

#[derive(PartialEq, Properties)]
pub struct Props<Card>
where Card: 'static + Copy + PartialEq + Eq
{
    pub hand: Vec<(Card,bool)>,
    pub onplay: Callback<Card,bool>,
    pub source: Callback<Card,String>,
}

pub struct Hand<Card>
where Card: 'static + Copy + PartialEq + Eq
{
    _cards: Vec<Card>,
}

impl<Card> Component for Hand<Card>
where Card: 'static + Copy + PartialEq + Eq
{
    type Message = ();
    type Properties = Props<Card>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _cards: vec![],
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = &ctx.props();
        let len = props.hand.len();

        html! {
            <div id="hand">
                {(0..len).map(|i| {
                    let (card, legal) = props.hand[i];

                    let call = props.onplay.clone();
                    let onclick = if legal {
                        Callback::from(move |_| {
                            let _ = call.emit(card);
                        })
                    } else {
                        Callback::from(move |_| {})
                    };

                    let style = if legal { "filter: brightness(100%);" }
                    else { "filter: brightness(75%);" };

                    let source = props.source.emit(card);
                    html! {
                        <img
                            src={source}
                            {onclick}
                            {style}/>
                    }
                }).collect::<Vec<Html>>()}
            </div>
        }
    }
}
