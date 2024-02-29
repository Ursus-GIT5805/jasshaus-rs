use yew::prelude::*;

use std::vec::Vec;
use gloo::timers::callback::Timeout;

mod window;
mod comm;
mod gameplay;
mod ws;
mod hand;
mod voting;

use crate::utils::*;

const NUM_PLAYERS: usize = 4;

use jasshaus_game::{
    Game,
    MarriageState,
    card::*,
    playtype::*,
};

use jasshaus_comm::{
    voting::*,
    socket_message::SocketMessage,
};

use comm::player::{Player, PlayerMSG};
use comm::rtc::{
    RtcHandler,
    RtcDataMsg,
    RtcSignaling,
};
use window::Windowtype;

use self::comm::chat::ChatMessageType;

// All the different events that
#[allow(dead_code)]
pub enum Event {
    None,
    BeginRound,

    WsMessage(SocketMessage),
    RtcMessage(RtcDataMsg, usize),
    OnLocalAudioStream( Option<web_sys::MediaStream> ),

    OnLocalChatMessage(String),

    ToggleShowmode,
    AddtoShow(Card),

    HideMsg(usize),

    OpenWindow(Windowtype),
}

// Main Component
pub struct GameComponent {
    pub id: usize,
    pub players: Vec< Player >,

    pub players_msg: Vec< PlayerMSG >,
    pub player_timeout: Vec< Option<Timeout> >,

    pub rtc: RtcHandler,
    pub chat_history: std::collections::VecDeque<(String,comm::chat::ChatMessageType)>,

    pub ws: ws::WebSocket,
    pub game: Game,
    pub woncards: [Cardset; 2],

    pub is_showing: bool,
    pub toshow: Cardset,

    pub votehandler: Votinghandler< Votingtype >,

    pub open_window: Windowtype,
}

impl Component for GameComponent {
    type Message = Event;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
	let link = ctx.link().clone();
	let onmessage = Callback::from(move |s: SocketMessage| link.send_message( Event::WsMessage(s) ));

	let win = web_sys::window().expect("Could not get window of page!");
	let addr = win.location().hostname().expect("Could not get hostname of page!");
	let url = format!("ws://{}:7999/ws", addr);

	let ws = ws::WebSocket::new(&url, onmessage);

	let rtc = {
	    let ws_clone = ws.clone();
	    let onsignal = Callback::from(move |t: (comm::rtc::RtcSignaling, usize)| {
		let ser = serde_json::to_string(&t.0).unwrap();
		ws_clone.send( SocketMessage::RtcSignaling(ser, t.1) );
	    });
	    let link = ctx.link().clone();
	    let onmessage = Callback::from( move |(s, id): (RtcDataMsg, usize)| {
		link.send_message( Event::RtcMessage(s, id) );
	    });

            RtcHandler::new(0, onsignal, onmessage, NUM_PLAYERS)
        };

        Self {
            id: 0,
            players: (0..NUM_PLAYERS).map(|i| Player::new(i)).collect(),

            players_msg: (0..NUM_PLAYERS).map(|_| PlayerMSG::None ).collect(),
	    player_timeout: (0..NUM_PLAYERS).map(|_| None).collect(),

	    rtc,
	    chat_history: std::collections::VecDeque::new(),

            ws,
            game: Game::new(),
            woncards: [Cardset::default(); 2],

	    is_showing: false,
	    toshow: Cardset::default(),

	    votehandler: Votinghandler::<Votingtype>::new(NUM_PLAYERS),

            open_window: Windowtype::Start,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Event::WsMessage(input) => self.handle_ws(input, ctx),
            Event::RtcMessage(input, sender) => self.handle_rtc(input, sender, ctx),
            Event::OpenWindow(win) => {
                self.open_window = win;
                true
            },
            Event::BeginRound => {
                self.game.start_new_round();
                self.woncards = [Cardset::default(); 2];

                let window =
                    if self.game.should_end() { Windowtype::End }
                else if self.game.get_announcing_player() as usize == self.id { Windowtype::Announce }
                else { Windowtype::None };

                ctx.link().send_message(Event::OpenWindow(window));
                false
            },
	    Event::ToggleShowmode =>  {
		self.is_showing = !self.is_showing;
		if !self.is_showing {
		    self.toshow.clear();
		}
		true
	    },
	    Event::AddtoShow(card) => {
		self.toshow.toggle(card);
		true
	    },
	    Event::HideMsg(plr) => {
		self.player_timeout[plr] = None;
		self.players_msg[plr] = PlayerMSG::None;
		true
	    },
	    Event::OnLocalAudioStream(m) => {
		self.rtc.init_rtc(m);
		false
	    },
	    Event::OnLocalChatMessage(s) => {
		let msg = comm::rtc::RtcDataMsg::Message(s.clone());
		self.set_msg( ctx, self.id, s, ChatMessageType::Yours );
		self.rtc.send_to_all(msg);
		true
	    },
	    _ => {
		log("No message matched!");
		false
	    },
	}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let names: Vec<String> = self.players.iter().map(|p| p.name.clone()).collect();
        let hand = self.game.hands[self.id].as_vec().iter().map(|c| {
	    if self.is_showing {
		(*c, true)
	    } else {
		(*c, self.game.is_legal_card(self.id as u8, *c))
	    }
        }).collect::<Vec<(Card,bool)>>();
        let source = Callback::from(move |card: Card| card_path(&card));
        let ws_clone = self.ws.clone();
        let onplay = if self.is_showing {
	    let link = ctx.link().clone();
	    Callback::from(move |card: Card| {
		link.send_message( Event::AddtoShow(card) );
		false
	    })
	} else {
	    Callback::from(move |card: Card| {
		ws_clone.send( SocketMessage::PlayCard(card) );
		true
	    })
	};

	let showbutton = if !self.game.can_show(self.id as u8) { html!{} } else {
	    let onclick = ctx.link().callback(move |_| Event::ToggleShowmode);
	    html! { <div id="showButton" {onclick}>{"Weisen"}</div> }
	};

	let showwindow = if !self.is_showing { html! {} } else {
	    let oncardclick = ctx.link().callback(Event::AddtoShow);

	    let ws_clone = self.ws.clone();
	    let onconfirm = Callback::from(move |show: Show| {
		ws_clone.send( SocketMessage::PlayShow(show) );
	    });

	    let link = ctx.link().clone();
	    let oncancel = Callback::from(move |_| {
		link.send_message( Event::ToggleShowmode );
	    });

	    html! {
		<gameplay::show::ShowWindow
		    hand={self.game.hands[self.id]}
		toshow={self.toshow}
		{oncardclick}
		{onconfirm}
		{oncancel}/>
	    }
	};

	let ws = self.ws.clone();
	let id = self.id as u8;
	let onvote = Callback::from(move |v: Vote| {
	    ws.send( SocketMessage::Vote(v, id) );
	});

	let link = ctx.link().clone();
	let chat_ontext = Callback::from(move |s: String| {
	    link.send_message( Event::OnLocalChatMessage(s) );
	});

	html! {
            <>
                <link rel="stylesheet" href="./styles/game.css"/>
                <link rel="stylesheet" href="./styles/window.css"/>

                <img src="./img/background.jpg" id="background"/>

            {gameplay::get_game(&self.game, names.clone(), self.id as u8)}
            <hand::Hand<Card> {hand} {onplay} {source}/>
            //<gameplay::show::ShowWindow hand={Cardset::default()} toshow={Cardset::default()}/>
	    {
		(0..NUM_PLAYERS).map(|i| {
		    html! {
			<audio id={format!("audio{}", i)} autoplay={true}/>
		    }
		}).collect::<Vec<Html>>()
	    }
            {(0..NUM_PLAYERS).map(|i| {
                let pos = get_player_pos(self.id as u8, i as u8) as usize;
                let style = match pos {
                    0 => "bottom:25vh;left:50%; transform:translateX(-50%);",
                    1 => "right: 2px; top:45%;",
                    2 => "top:   2px; left:50%; transform:translateX(-50%);",
                    3 => "left:  2px; top:45%;",
                    _ => "",
                };
                let mut free = [true; 4];
                free[ (6+pos) % 4 ] = false;

		let onmsg_click = ctx.link().callback(move |_: MouseEvent| Event::HideMsg(i));

                html! {
                    <comm::player::PlayerComponent
                        player={self.players[i].clone()}
                    {style}
                    msg={self.players_msg[i].clone()}
                    {free}
                    display_name={pos != 0}
		    {onmsg_click}/>
                }
            }).collect::<Vec<Html>>()}
            {self.get_window(ctx)}
	    < comm::chat::Chat history={self.chat_history.clone()} ontext={chat_ontext} player_names={names.clone()} />
	    {showbutton}
	    {showwindow}
	    {voting::get_voting_window(&self.votehandler, self.id, onvote)}
            </>
        }
    }
}

impl GameComponent {
    fn set_msg(&mut self, ctx: &Context<Self>, plr: usize, text: String, ty: ChatMessageType) {
	let link = ctx.link().clone();
	self.player_timeout[plr] = Some( Timeout::new(5000, move || {
	    link.send_message(Event::HideMsg(plr));
	}));
	self.chat_history.push_back( (text.clone(), ty) );
	if self.chat_history.len() > comm::chat::MAX_MESSAGES {
	    self.chat_history.pop_front();
	}
        self.players_msg[plr] = PlayerMSG::Text(text);
    }

    fn set_show_msg(&mut self, plr: usize, shows: Vec<Show>) {
	if shows.is_empty() { return; }

	let htmls = shows.iter().map(|show| {
	    let cards = show.as_cards();

	    let card_html = cards.iter().map(|card| {
		let src = card_path(card);

		html! {
		    <img {src}/>
		}
	    }).collect::<Vec<Html>>();

	    html! {
		<div class="Show">
		{card_html}
		</div>
	    }
	}).collect::<Vec<Html>>();

	let html = html! {
	    <div class="Shows">
	    {htmls}
	    </div>
	};

        self.players_msg[plr] = PlayerMSG::Html(html);
    }

    fn handle_ws(&mut self, input: SocketMessage, ctx: &Context<Self>) -> bool {
        match input {
            SocketMessage::PlayCard(card) => {
                self.game.play_card(card); // Play the card
                if self.game.history_index % 4 == 0 {
                    let index = self.game.history_index as usize;
                    for i in index-4..index {
                        self.woncards[ self.game.current_player as usize & 1 ].insert( self.game.history[i] );
                    }

		    if self.game.history_index == 4 {
			for i in 0..NUM_PLAYERS {
			    let mut shows: Vec<Show> = vec![];

			    for show in self.game.shows[i].into_iter() {
				if show.color < 4 {
				    shows.push( show );
				}
			    }

			    self.set_show_msg(i, shows);
			}
		    }
                }
                if self.game.history_index == 36 || self.game.should_end() {
                    ctx.link().send_message(Event::OpenWindow(Windowtype::Roundsummary));
                }
                true
            },
            SocketMessage::RtcSignaling(sig, sender) => {
                let signal: RtcSignaling = serde_json::from_str(&sig).unwrap();
                self.rtc.handle_signaling(signal, sender);
                false
            },
            SocketMessage::Announce(pt) => {
                if self.game.is_announced() {
                    self.game.start_new_round();
                    self.woncards = [Cardset::default(); 2];
                }
                self.game.announce(pt);

                if pt.playtype == PASS {
                    let win = if self.id == self.game.get_announcing_player() as usize { Windowtype::Announce }
                    else { Windowtype::None };
                    ctx.link().send_message( Event::OpenWindow(win) );
                    return true;
                }
                ctx.link().send_message( Event::OpenWindow(Windowtype::None) );

                let annplr = self.game.get_announcing_player() as usize;
                let msg = crate::utils::pt_name(pt.playtype, pt.misere);
                self.set_msg(ctx, annplr, msg, ChatMessageType::Info);
                true
            },
            SocketMessage::ShowPoints(points, plr) => {
                self.set_msg(ctx, plr as usize, points.to_string(), ChatMessageType::Info);
                true
            },
            SocketMessage::ShowList(shows) => {
                self.game.shows = shows;
                true
            },
            SocketMessage::MarriageWouldWin(plr) => {
                self.game.marriage[plr as usize] = MarriageState::WouldWin;
                false
            },
            SocketMessage::ID(id) => {
                self.id = id as usize;
		self.rtc.id = id as usize;
                for i in 0..4 { self.players[i].name = format!("{}-Player", i); }

		let link = ctx.link().clone();
                let onstream = Callback::from(move |m: Option<web_sys::MediaStream>| {
		    link.send_message( Event::OnLocalAudioStream(m) );
		});
		comm::rtc::setup_mic(onstream);

                true
            },
            SocketMessage::PlayerJoined(_plr) => false,
            SocketMessage::SetAnnouncePlayer(plr) => {
                self.game.announce_player = plr;
                self.game.current_player = plr;
                if self.id == plr as usize {
                    ctx.link().send_message( Event::OpenWindow(Windowtype::Announce) );
                }
                true
            },
            SocketMessage::GameState(game, hand) => {
                self.game = game;
                self.game.hands[self.id] = hand;
                ctx.link().send_message( Event::OpenWindow(Windowtype::None) );
                // get the current cards on the board
                //self.info.startpos = get_player_pos(self.info.id, self.game.get_beginplayer());
                true
            },
            SocketMessage::GameSetting(setting) => {
                self.game.setting = setting;
                false
            },
            SocketMessage::StartMating => {
                ctx.link().send_message( Event::OpenWindow( Windowtype::Mate ) );
                true
            },
            SocketMessage::NewCards(cards) => {
                self.game.hands[ self.id ] = cards;
                true
            },
	    SocketMessage::Vote(vote, plr) => {
		self.votehandler.set_vote( plr as usize, vote );

		if self.votehandler.all_ready() && self.votehandler.is_accepted() {
		    match self.votehandler.subject {
			Votingtype::REVANCHE => {
			    self.game.points = [0, 0];
			    self.game.start_new_round();
			    ctx.link().send_message( Event::OpenWindow( Windowtype::None ) );
			},
			_ => {},
		    }
		}

		true
	    },
	    SocketMessage::NewVote(vt) => {
		self.votehandler.set_subject(vt);
		true
	    },
            SocketMessage::PlayerOrder(order) => {
                let mut becomes = [0usize; 4];
                for i in 0..4 { becomes[ order[i] ] = i; }
                self.id = becomes[ self.id ];
                // TODO switch other players
                ctx.link().send_message( Event::OpenWindow(Windowtype::None) );
                true
            },
            _ => {
                log("Invalid header!");
                false
            },
        }
    }


    fn handle_rtc(&mut self, input: RtcDataMsg, plr: usize, ctx: &Context<Self>) -> bool {
        match input {
            RtcDataMsg::Name(name) => {
                self.players[plr].name = name;
            },
            RtcDataMsg::Message(msg) => {
		self.set_msg(ctx, plr, msg.clone(), ChatMessageType::Normal(plr));
            },
            _ => {},
        }

        true
    }

    fn get_window(&self, ctx: &Context<Self>) -> Html {
        match self.open_window {
            Windowtype::Announce => html! { <window::announce::Announce passed={self.game.playtype.passed} ws={self.ws.clone()}/> },
            Windowtype::Start => window::start::window(),
            Windowtype::Mate => html! { <window::mate::Mate id={self.id} ws={self.ws.clone()}/> },
            Windowtype::Roundsummary => {
                let onclose = ctx.link().callback(move |_| Event::BeginRound );
                html! {
                    <window::round_summary::RoundSummary
                        names={self.players.iter().map(|p| p.name.clone()).collect::<Vec<String>>()}
                    points={self.game.points}
                    won_points={self.game.won_points}
                    show_points={self.game.show_points}
                    woncards={self.woncards}
                    {onclose}/>
                }
            },
            Windowtype::End => {
                let names: Vec<String> = self.players.iter().map(|p| p.name.clone()).collect();
                let won = self.id as u8 % 2 == self.game.get_winner_team();
                html! {
                    <window::end::End {won} points={self.game.points} {names} />
                }
            },
            Windowtype::Info(info) => html! { <window::info::Info {info}/> },
            _ => html! {},
        }
    }
}
