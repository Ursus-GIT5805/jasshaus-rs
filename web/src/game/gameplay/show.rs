use yew::prelude::*;

use jasshaus_game::card::*;
use crate::utils::card_path;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub hand: Cardset,
    pub toshow: Cardset,

    pub oncardclick: Callback<Card>,
    pub onconfirm: Callback<Show>,
    pub oncancel: Callback<()>,
}

pub struct ShowWindow {}

impl ShowWindow {
    fn extract_show(&self, ctx: &Context<Self>) -> Result<Show,&str> {
	let cards = ctx.props().toshow.as_vec();

	if cards.len() < 3 { return Err("Zu kurz!"); }

	// The player wants to make a row!
	let res = if cards[0].color == cards[1].color {
	    for i in 0..(cards.len()-1) {
		if cards[i].color != cards[i+1].color || cards[i].number + 1 != cards[i+1].number {
		    return Err("Illegaler Weis!");
		}
	    }
	    let show = Show::new(cards[0].color, cards[0].number, cards.len() as u8);
	    Ok(show)
	} else {
	    if cards.len() != 4 { return Err("Illegaler Weis!"); }
	    for i in 0..3 {
		if cards[i].color == cards[i+1].color || cards[i].number != cards[i+1].number {
		    return Err("Illegaler Weis!");
		}
	    }
	    let show = Show::new(cards[0].color, cards[0].number, 1);
	    Ok(show)
	};

	if let Ok(s) = res {
	    match ctx.props().hand.has_show(s) {
		Ok(_) => Ok(s),
		Err(e) => match e {
		    ShowError::IsSubset => Err("Du kannst mehr weisen ;)"),
		    _ => Err("Illegaler Weis!"),
		}
	    }
	} else {
	    res
	}
    }
}

impl Component for ShowWindow {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cards = ctx.props().toshow.as_vec();
	let show = self.extract_show(ctx);

	let cancel = {
	    let call =  ctx.props().oncancel.clone();
	    Callback::from(move |_| {
		call.emit(());
	    })
	};

	let confirm = if show.is_err() { Callback::from(move |_| {}) } else {
	    if let Ok(s) = show {
		let call = ctx.props().onconfirm.clone();
		Callback::from(move |_| {
		    call.emit( s );
		})
	    } else {
		Callback::from(move |_| {})
	    }
	};

	let info = if let Err(e) = show { e } else { "" };

        html! {
            <div id="showWindow">
                <h1>{"Wähle Karten von deiner Hand aus"}</h1>
                <a>{info}</a>
                <div id="showCards">
                {
                    cards.iter().map(|c| {
			let call = ctx.props().oncardclick.clone();
			let card = *c;
			let onclick = Callback::from(move |_| {
			    call.emit(card);
			});
                        html! {
                            <img {onclick} src={card_path(c)}/>
                        }
                    }).collect::<Vec<Html>>()
                }
                </div>
		<div style="display:flex;justify-conent:space-evenly;">
		<div class="endButton" onclick={cancel}>{"Abbrechen"}</div>
		<div class="endButton" onclick={confirm}>{"Weisen"}</div>
		</div>
	    </div>
        }
    }
}
