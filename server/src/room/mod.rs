mod client;

use jasshaus_game::{
    Game,
    MarriageState,
    card::*,
    setting::*,
    playtype::*,
};

use rand::Rng;

use jasshaus_comm::{
    voting::*,
    socket_message::SocketMessage,
};
use client::*;

const NUM_PLAYERS: usize = 4;

#[derive(PartialEq, Eq)]
pub enum RoomState {
    ENTERING = 0,
    TEAMING = 1,
    PLAYING = 2,
    ENDING = 3,
}

pub struct Room {
    pub clients: [Client; NUM_PLAYERS],
    pub game: Game,
    pub mate: [u8; NUM_PLAYERS],
    pub state: RoomState,
    pub voting: Votinghandler<Votingtype>,

    pub sent_marriage: [bool; NUM_PLAYERS],
}

impl Room {
    pub fn new() -> Self {
        let clients: [Client; NUM_PLAYERS] = [Client::new(0), Client::new(1), Client::new(2), Client::new(3)];

        Room {
            clients,
            game: Game::new(),
            mate: [0xFF; NUM_PLAYERS],
            state: RoomState::ENTERING,
            voting: Votinghandler::new(NUM_PLAYERS),

            sent_marriage: [false; NUM_PLAYERS],
        }
    }

    // Registering ---
    pub async fn register(&mut self, ws_tx: WsWriter) -> Option<u8> {
        for i in 0..4 {
            if self.clients[i].connected { continue; }

            self.clients[i].connect(ws_tx);

            let plr = self.clients[i].player_id;
            self.send_to_all_except( i as u8, SocketMessage::PlayerJoined(plr) ).await;
            self.send_to( i as u8, SocketMessage::ID(plr) ).await;
            self.send_to(i as u8, SocketMessage::GameSetting(self.game.setting)).await;


            let num_connected: usize = self.clients.iter().map(|x| x.connected as usize).sum();
            if num_connected == 4 {
                if self.state == RoomState::ENTERING {
		    self.new_vote( Votingtype::NONE ).await;
                    self.state = RoomState::TEAMING;
                    self.send_to_all( SocketMessage::StartMating ).await;
                } else {
                    let game = self.game.clone();
                    let hand = self.game.hands[i];
                    self.send_to(i as u8, SocketMessage::GameState(game, hand)).await;
                }
            }

            return Some(i as u8);
        }
        return None;
    }

    pub async fn unregister(&mut self, client_id: u8) {
        let plr = self.clients[client_id as usize].player_id;
        self.clients[client_id as usize].disconnect();
        //self.send_to_all( DataPacket::packet_agreementinfo(self.voting) ).await;
        self.send_to_all( SocketMessage::PlayerDisconnected(plr) ).await;
    }

    // Communication functions ---

    async fn send_to(&mut self, client_id: u8, data: SocketMessage) {
        self.clients[client_id as usize].send( data ).await;
    }

    async fn send_to_all_except(&mut self, client_id: u8, data: SocketMessage) {
        for i in 1u8..4u8 {
            self.send_to((client_id + i) % 4, data.clone()).await;
        }
    }

    async fn send_to_all(&mut self, data: SocketMessage) {
        for i in 0u8..4u8 {
            self.send_to(i, data.clone()).await;
        }
    }

    fn get_client(&self, player_id: u8) -> u8 {
        for i in 0..4 {
            if self.clients[i].player_id == player_id { return i as u8; }
        }
        0u8
    }

    // Utility functions ---

    async fn new_vote(&mut self, vt: Votingtype) {
	self.voting.set_subject(vt);
	self.send_to_all( SocketMessage::NewVote(vt) ).await;
    }

    // Gameplay functions ---

    async fn end_game(&mut self) {
        debug!("End game");
	self.new_vote( Votingtype::REVANCHE ).await;
        self.state = RoomState::ENDING;
    }

    /*async fn handle_marriage(&mut self) -> bool {
    if !self.game.playtype.is_trumpf() { return false; }
    todo!("Implement this");
}*/

    fn get_first_announceplayer(&self) -> u8 {
        match self.game.setting.startcondition {
            StartingCondition::CARD(card) => {
                (0..NUM_PLAYERS)
                    .find(|&i| self.game.hands[i].contains(card))
                    .unwrap_or(0) as u8
            },
            StartingCondition::RANDOM => {
                let mut rng = rand::thread_rng();
                rng.gen::<u8>() % NUM_PLAYERS as u8
            },
            StartingCondition::PLAYER(plr) => plr,
        }
    }

    async fn start_round(&mut self) {
        debug!("Start new round");
        self.game.start_new_round();
        self.game.generate_cards();

        for i in 0..4 {
            let plr = self.clients[i].player_id;
            self.send_to(i as u8, SocketMessage::NewCards(self.game.hands[plr as usize]) ).await;
        }
    }

    async fn start_game(&mut self, revanche: bool) {
        debug!("Start game!");
        self.game.points = [0, 0];
        self.game.start_new_round();
        self.state = RoomState::PLAYING;
	self.new_vote( Votingtype::NONE ).await;

        self.start_round().await;

        if !revanche || self.game.setting.apply_startcondition_on_revanche {
            self.game.announce_player = self.get_first_announceplayer();
            debug!("Starting player is {}", self.game.announce_player);
        }
        self.send_to_all( SocketMessage::SetAnnouncePlayer(self.game.announce_player) ).await;
    }

    async fn play_card( &mut self, card: Card, client_id: u8 ) {
        let plr = self.clients[ client_id as usize ].player_id;
        if !self.game.is_legal_card(plr, card) {
            error!("It's illegal to play this card!");
            return;
        }
        self.game.play_card(card);

        match self.game.history_index {
            4 => self.send_to_all( SocketMessage::ShowList(self.game.shows) ).await,
            36 => {
                if self.game.should_end() { self.end_game().await; }
                self.start_round().await;
            },
            _ => {},
        }

	if self.game.should_end() {
	    self.end_game().await;
	}

        self.send_to_all( SocketMessage::PlayCard(card) ).await;
    }

    async fn announce( &mut self, pt: Playtype, client_id: u8 ) {
        let plr = self.clients[client_id as usize].player_id;
        if !self.game.can_announce(plr) {
            error!("Player mustn't at this time!");
            return;
        }
        if pt.playtype == PASS && self.game.playtype.passed {
            error!("It's already passd!");
            return;
        }

        self.game.announce(pt);
        self.send_to_all( SocketMessage::Announce(pt) ).await;
        for i in 0..NUM_PLAYERS {
            if self.game.marriage[i] == MarriageState::WouldWin {
                self.send_to_all( SocketMessage::MarriageWouldWin(plr) ).await;
            }
        }
    }

    async fn play_show( &mut self, show: Show, client_id: u8 ) {
        let plr = self.clients[client_id as usize].player_id;
        if !self.game.can_show(plr) {
            error!("Player mustn't show at this time!");
            return;
        }
        if let Err(e) = self.game.hands[plr as usize].has_show(show) {
            error!("Player can't show it: {:?}", e);
            return;
        }
        if self.game.play_show(show).is_err() {
            error!("Error while showing!");
            return;
        }

	let points = self.game.playtype.get_show_value(show);
        self.send_to_all( SocketMessage::ShowPoints(points, plr) ).await;
    }

    async fn handle_voting( &mut self, vote: Vote, client_id: u8 ) {
        if self.voting.subject == Votingtype::NONE { return; }

	let plr = self.clients[ client_id as usize ].player_id;
	self.voting.set_vote( plr as usize, vote );
	self.send_to_all( SocketMessage::Vote(vote, plr) ).await;

        if !self.voting.all_ready() { return; }
        if self.voting.is_accepted() {
            match self.voting.subject {
		Votingtype::REVANCHE => self.start_game(true).await,
		_ => {}
            }

	    self.voting.set_subject( Votingtype::NONE );
	}
    }

    async fn handle_team_choosing( &mut self, mate: u8, client_id: u8 ) {
        if self.state != RoomState::TEAMING { return; }
        self.mate[client_id as usize] = mate;
        if self.mate.contains(&0xFF){ return; } // Not all players have chosen

        let mut order: [usize; 4] = [0,1,2,3];
        for i in 0..4 {
            let m = self.mate[i] as usize;
            if m == i { continue; }
            if self.mate[m] == i as u8 || self.mate[m] == m as u8 {
                debug!("Link Plr[{}] with Plr[{}]", m, i);
                let second = {
                    if (i+1) % 4 == m { (m+1) % 4 }
                    else { (i+1) % 4 }
                };
                let last = 6 - i - m - second;

                order = [i, second, m, last];
                break;
            }
        }
        debug!("Order of the players {:?}", order);

        for i in 0..4 { self.clients[ order[i] ].player_id = i as u8; }
        self.send_to_all( SocketMessage::PlayerOrder(order) ).await;
        self.start_game(false).await;
    }

    pub async fn handle_input( &mut self, input: SocketMessage, client_id: u8 ) {
	let plr = self.clients[ client_id as usize ].player_id;
	debug!("[{}] {:?}", plr, input);

	match input {
	    SocketMessage::PlayCard(card) => self.play_card(card, client_id).await,
	    SocketMessage::Announce(pt) => self.announce(pt, client_id).await,
	    SocketMessage::PlayShow(show) => self.play_show(show, client_id).await,
	    SocketMessage::Vote(vote, _) => self.handle_voting(vote, client_id).await,
	    SocketMessage::Mate(mate) => self.handle_team_choosing(mate, client_id).await,
	    SocketMessage::RtcSignaling(s, recv) => self.send_to( self.get_client(recv as u8),
								  SocketMessage::RtcSignaling(s, plr as usize) ).await,
	    _ => {
		error!("Invalid header!");
	    },
	}
    }
}
