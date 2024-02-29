use yew::prelude::*;

use std::vec::Vec;
use jasshaus_game::card::*;
use crate::utils::card_path;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub names: Vec< String >,
    pub points: [u16; 2],
    pub won_points: [u16; 2],
    pub show_points: [u16; 2],
    pub woncards: [Cardset; 2],
    pub onclose: Callback<MouseEvent>,
}

pub struct RoundSummary {}

impl Component for RoundSummary {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = &ctx.props();

        let teams = (0..2).map(|i| {
            let names = html! {
                <div>
                    <div style="font-size: 2rem; white-space: nowrap;">
                        <span class="Summaryname">{props.names[i].clone()}</span>
                        <a>{" & "}</a>
                        <span class="Summaryname">{props.names[i+2].clone()}</span>
                    </div>
                    <div>
                        <a>{"Beginn"}</a><br/>
                        <a>{"Weis"}</a><br/>
                        <a>{"Stich"}</a><br/>
                        <a>{"--------"}</a><br/>
                        <a style="font-size: 1.5rem;">{"Endstand"}</a>
                    </div>
                </div>
            };

            let bef = props.points[i] - props.won_points[i] - props.show_points[i];

            let result = html! {
                <div style="text-align: right;">
                    <div style="font-size: 2rem; white-space: nowrap;"><br/></div>
                    <a>{bef}</a><br/>
                    <a>{format!("+{}", props.show_points[i])}</a><br/>
                    <a>{format!("+{}", props.won_points[i])}</a><br/>
                    <a>{"--------"}</a><br/>
                    <a style="font-size: 1.5rem;">{props.points[i]}</a>
                </div>
            };

            let cards = html! {
                <div class="SummaryCards">
                    {
                        (0..9).map(|n| {
                            let column = (0..4).map(|c| {
                                let card = Card::new(c, n);
                                let style = if props.woncards[i].contains(card) {""}
                                else {"filter:brightness(50%);"};

                                html! { <img src={card_path(&card)} {style}/> }
                            }).collect::<Vec<Html>>();

                            html! { <div class="SummaryCardsColumn">{column}</div> }
                        }).collect::<Vec<Html>>()
                    }
                </div>
            };

            html! {
                <div class="SummaryTeam">
                    {names}
                    {result}
                    <div style="width: 2rem;"></div>
                    {cards}
                </div>
            }
        }).collect::<Vec<Html>>();

        html! {
            <div id="roundSummary">
                {teams}
                <div id="closeSummary" onclick={ctx.props().onclose.clone()}>{"Weiter"}</div>
            </div>
        }
    }
}
