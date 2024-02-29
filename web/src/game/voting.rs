use yew::prelude::*;

use jasshaus_comm::voting::*;

pub fn get_voting_window( vh: &Votinghandler<Votingtype>, id: usize, onvote: Callback<Vote> ) -> Html {
    if vh.subject == Votingtype::NONE { return html! {}; }

    let total = vh.vote.len();
    let vote = vec![ Vote::AGREE, Vote::NEUTRAL, Vote::DECLINE ];

    html! {
	<div id="votingWindow">
	    <h2>{
		match vh.subject {
		    Votingtype::REVANCHE => "Revanche",
		    _ => "Unbekannt",
		}
	    }</h2>
	    <div id="voteButtons">
	{
	    (0..3).map(|i| {
		let v = vote[i];

		let name = match v {
		    Vote::AGREE => "Ja",
		    Vote::NEUTRAL => "Neutral",
		    Vote::DECLINE => "Nein",
		    _ => panic!("There is no 3rd agree action"),
		};

		let sum: usize = vh.vote.iter().map(|&vt| (vt == v) as usize).sum();
		let text = format!("{} ({}/{})", name, sum, total);

		let class = if vh.vote[id] == v { "VoteButtonChosen" }
		else { "VoteButton" };

		let call = onvote.clone();
		let onclick = Callback::from(move |_: MouseEvent| call.emit(v) );

		html! { <div {class} {onclick}>{text}</div> }
	    }).collect::<Vec<Html>>()
	}
	    </div>
	</div>
    }

}
