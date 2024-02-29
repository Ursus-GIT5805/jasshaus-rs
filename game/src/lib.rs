pub mod card;
pub mod playtype;
pub mod setting;

use card::*;
use playtype::*;
use setting::*;

use rand::seq::SliceRandom;

use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;

const MARRIAGE_POINTS: u16 = 20;


#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, std::fmt::Debug, Default)]
#[derive(Serialize, Deserialize)]
pub enum MarriageState {
    #[default]
    None,
    PlayedOne,
    PlayedBoth,
    WouldWin, // Used for setting.point_recv_order, if the order is relevant
}

#[derive(Clone)]
#[derive(PartialEq, Eq, std::fmt::Debug)]
#[derive(Serialize, Deserialize)]
pub struct Game {
    // History
    pub history_index: u8,
    #[serde(with = "BigArray")]
    pub history: [Card; 36], // history[i] = the {i+1}-th card played in the round

    // Needed for calculations
    pub team0_wins: u8, // How many turns team 0 won (used for checking match)
    pub bestcard_index: u8,

    //TODO insert settings
    pub points: [u16; 2],
    pub won_points: [u16; 2],
    pub show_points: [u16; 2],

    pub playtype: Playtype,
    pub announce_player: u8,
    pub current_player: u8,

    pub setting: Setting,

    // Contains private information ---
    #[serde(skip)]
    pub shows: [[Show; 3]; 4],
    #[serde(skip)]
    pub hands: [Cardset; 4],
    #[serde(skip)]
    pub marriage: [MarriageState; 4],
}

impl Game {
    pub fn new() -> Self {
        Game {
            history_index: 0,
            history: [Card::default(); 36],

            team0_wins: 0,
            bestcard_index: 0,

            points: [0; 2],
            won_points: [0; 2],
            show_points: [0; 2],

            playtype: Playtype::default(),
            announce_player: 0,
            current_player: 0,

            setting: Setting::default(),

            shows: [[Show::default(); 3]; 4],
            hands: [Cardset::default(); 4],
            marriage: [MarriageState::None; 4],
        }
    }

    // Events ------

    pub fn generate_cards(&mut self) {
        let mut cards: [Card; 36] = [Card::new(0,0); 36];
        for i in 0..36 { cards[i] = Card::from_id(i as u8); }

        {
            let mut rng = rand::thread_rng();
            cards.shuffle(&mut rng);
        }

	for i in 0..4 { self.hands[i].clear(); }
        for i in 0..36 { self.hands[i/9].insert( cards[i] ); }
    }

    pub fn start_new_round(&mut self) {
        // Reset round variables
        self.team0_wins = 0;
        self.history_index = 0;
        self.marriage = [MarriageState::None; 4];
        for i in 0..36 { self.history[i] = Card::new(4, 0); }
        for i in 0..12 { self.shows[i/3][i%3] = Show::new(4, 0, 0); }
        self.playtype = Playtype::new();

	self.won_points = [0, 0];
	self.show_points = [0, 0];

	self.announce_player = (self.announce_player+1) % 4;
        self.current_player = self.announce_player;
    }

    fn end_round(&mut self) {
        if self.history_index == 36 {
            let last_team = self.current_player as usize & 1; // The team which won the last turn
            self.add_points(last_team, 5); // Last team get's 5 points

            // Checking if the team, who won the last turn, won each other turn so far
            let wins: u8 = match last_team {
                0 => self.team0_wins,
                1 => 9 - self.team0_wins,
                _ => panic!("A team, which shouldn't exist, won a turn!"),
            };
            if wins == 9 { self.add_points(last_team, 100); }
        }
    }

    pub fn play_marriage(&mut self, plr: usize) {
        self.points[plr&1] += MARRIAGE_POINTS;
        self.show_points[plr&1] += MARRIAGE_POINTS;
        self.marriage[plr] = MarriageState::PlayedBoth;
    }

    // Handles marraige
    pub fn handle_marriage(&mut self) {
        if !self.playtype.is_trumpf() { return; }

        for i in 0..4 {
            if self.marriage[i] != MarriageState::WouldWin { continue; }
            self.play_marriage(i);
        }
    }

    // Handles shows and add the points
    pub fn handle_shows(&mut self) {
        let mut bestplr: usize = 0;
        let mut bestshw: usize = 0;
        for plr in 0..4 {
            for i in 0..3 {
                if self.shows[plr][i].color == 4 { break; }
                if self.shows[bestplr][bestshw].color == 4 ||
                    self.playtype.is_show_stronger(self.shows[bestplr][bestshw], self.shows[plr][i])
                {
                    bestplr = plr;
                    bestshw = i;
                }
            }
        }
        if self.shows[bestplr][bestshw].color == 4 { return; }

        let team = bestplr & 1;

        for i in 0..4 {
            for j in 0..3 {
                if i&1 == team {
                    if self.shows[i][j].color == 4 { break; }
                    let sp = std::cmp::min
                        (self.playtype.get_show_value(self.shows[i][j]),
                         self.setting.show_points_maximum);
                    self.points[i&1] += sp;
                    self.show_points[i&1] += sp;

                } else { self.shows[i][j] = Show::new(4,2,0); }
            }
        }
    }

    fn end_turn(&mut self) {
        let mut points: u16 = 0;
        for i in (self.history_index-4)..self.history_index {
            let card = self.history[i as usize];
            points += self.playtype.get_card_value( card );
        }

        let best_player = (self.current_player + 4 - (self.history_index-1-self.bestcard_index)) % 4;
        let best_team: u8 = best_player & 1;

        self.team0_wins += (best_team == 0) as u8;
        self.current_player = best_player;

        if self.history_index == 4 {
            for rule in self.setting.point_recv_order {
                match rule {
                    PointRule::PLAY => self.add_points(best_team as usize, points),
                    PointRule::SHOW => self.handle_shows(),
                    PointRule::MARRIAGE => self.handle_marriage(), // TODO
                }
                if self.should_end() {
                    self.end_round();
                    return;
                }
            }
        } else {
            self.add_points(best_team as usize, points);
            if self.should_end() {
                self.end_round();
                return;
            }
        }

        let flip = match self.playtype.playtype {
            SLALOMUPDOWN | SLALOMDOWNUP => true, // flip every end of turn
            GUSCHTI | MARY => self.history_index == 16, // flip at end of 4th turn
            _ => false,
        };
        if flip { self.playtype.ruletype ^= 1; }

        if self.history_index == 36 { self.end_round(); } // End of last turn
    }

    // Action functions ------

    pub fn play_card(&mut self, card: Card) {
        let bcrd = self.history[ self.bestcard_index as usize ];
        if self.history_index % 4 == 0 || self.playtype.is_card_stronger(bcrd, card) {
            self.bestcard_index = self.history_index;
        }

        self.history[ self.history_index as usize ] = card;
        self.history_index += 1;
        self.hands[ self.current_player as usize ].erase(card);

        if self.playtype.is_trumpf() {
            let trumpf = self.playtype.get_trumpf_color();

            if card.color == trumpf && (card.color == 6 || card.color == 7) {
                let plr = self.current_player as usize;
                match self.marriage[plr] {
                    MarriageState::None => self.marriage[plr] = MarriageState::PlayedOne,
                    MarriageState::PlayedOne => self.play_marriage(plr),
                    _ => (),
                }
            }
        }

        if self.history_index % 4 == 0 {
            self.end_turn();
        } else if self.should_end() {
            //self.end_game();
        } else {
            self.current_player = (self.current_player + 1) % 4;
        }
    }

    pub fn play_show(&mut self, show: Show) -> Result<(),()> {
        for i in 0..3 {
            let rshow = &self.shows[self.current_player as usize][i];
            if *rshow == show { return Err(()); }
            if rshow.color == 4 {
                self.shows[self.current_player as usize][i] = show;
                return Ok(());
            }
        }
        Err(())
    }

    pub fn announce(&mut self, pt: Playtype) {
        if pt.playtype == PASS {
            if !self.setting.allow_pass { return; }
            self.playtype.passed = true;
            self.current_player = self.get_announcing_player();
            return;
        }
        if !self.setting.allow_misere && pt.misere { return }
        if !self.setting.allow_playtype[pt.playtype as usize] { return; }

        if pt.is_trumpf() {
            let trumpf = pt.get_trumpf_color();
            for i in 0..4 {
                if self.points[i&1] + MARRIAGE_POINTS < self.setting.max_points { continue; }
                if self.hands[i].contains(Card::new(trumpf, 6)) && self.hands[i].contains(Card::new(trumpf, 7)) {
                    self.marriage[i] = MarriageState::WouldWin;
                }
            }
        }

        self.playtype = pt;
        self.current_player = self.get_startplayer();
        self.playtype.ruletype = match pt.playtype {
            SLALOMUPDOWN | GUSCHTI => UPDOWN,
            SLALOMDOWNUP | MARY => DOWNUP,
            x => x,
        };
    }

    // Utility functions ---

    // Add points to a given team (or the other on misere)
    fn add_points(&mut self, team: usize, points: u16) {
        let real_team = team ^ self.playtype.misere as usize;
        let p = points * self.setting.playtype_multiplier[self.playtype.playtype as usize] as u16;
        self.points[real_team] += p;
        self.won_points[real_team] += p;
    }

    // The game should end if any team has reached the number of points for winning
    pub fn should_end(&self) -> bool {
        self.points.iter().any(|x| x >= &self.setting.max_points)
    }

    // The beginplayer is the player who began the current turn
    pub fn get_beginplayer(&self) -> u8 {
        (self.current_player + 4 - (self.history_index%4)) % 4
    }

    // The startplayer is the player who started the turn the first turn
    pub fn get_startplayer(&self) -> u8 {
        if  self.playtype.passed &&
            self.setting.passed_player_begins[self.playtype.playtype as usize]
            { (self.announce_player + 2) % 4 }
        else { self.announce_player }
    }

    // The player who should announce now/or has announced. Passing changes the announcing player
    pub fn get_announcing_player(&self) -> u8 {
        (self.announce_player + 2*( self.playtype.passed as u8 )) % 4
    }

    // True when the given card is playable
    pub fn is_legal_card(&self, player_id: u8, card: Card) -> bool {
        // Check for basic cheating
        if self.history_index >= 36 { return false; }
        if self.playtype.playtype == NONE { return false; }
        if !self.hands[player_id as usize].contains(card) { return false; }
        if player_id != self.current_player { return false; }

        // If it's the first card currently played, it's always legal
        let first_card_index = self.history_index as usize / 4 * 4;
        if self.history[first_card_index].color == 4 { return true; }

        let fcrd = self.history[first_card_index]; // first card
        let bcrd = self.history[self.bestcard_index as usize]; // best card
        let hand = &self.hands[player_id as usize];

        // First, check all additional rules from trumpf
        if self.playtype.is_trumpf() {
            let trumpf_first = self.playtype.is_color_trumpf(fcrd.color);
            let trumpf_card = self.playtype.is_color_trumpf(card.color);

            // Rule: you can't play a weaker trumpf than on the board
            // You can play it if you have no other choice
            if !trumpf_first && trumpf_card {
                // If the first card wasn't a trumpf and the new card is a trumpf, it must be stronger
                if self.playtype.is_card_stronger(bcrd, card) { return true; }
                // Since this is not true, there must be a stronger trumpf on the board
                // It's only legal to play if you can't play anything else
                return hand.only_has_color(card.color) && !hand.has_stronger_trumpf(fcrd);
            }

            // Rule: You are NEVER forced to play trumpf-boy
            if trumpf_first && !trumpf_card && hand.has_color(fcrd.color) {
                // You can play ANY card if you only possess the trumpf boy, since in this case you "must" hold trumpf
                return hand.count_color(fcrd.color) == 1 && hand.contains(Card::new(fcrd.color, 5));
            }
        }
        // Basic: You must hold the color if you can
        fcrd.color == card.color || !hand.has_color(fcrd.color)
    }

    pub fn can_show(&self, player_id: u8) -> bool {
        self.current_player == player_id && self.history_index < 4 && self.playtype.playtype != NONE
    }

    // Returns true when the given player can announce
    pub fn can_announce(&self, player_id: u8) -> bool {
        self.playtype.playtype == NONE && self.get_announcing_player() == player_id
    }

    // Return a vector of cards which are on the board
    pub fn get_playedcards(&self) -> Vec<Card> {
        let start = self.history_index / 4 * 4;
        let end = self.history_index;

        (start..end).map(|i| self.history[i as usize]).collect()
    }

    pub fn is_announced(&self) -> bool {
        self.playtype.playtype != NONE
    }

    pub fn get_winner_team(&self) -> u8 {
        if self.points[0] < self.points[1] { 1 }
        else { 0 }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode_decode_game() {
        let before = crate::Game::new();
        let ser = serde_json::to_string(&before).unwrap();
        let after: crate::Game = serde_json::from_str(ser.as_str()).unwrap();
        assert_eq!(before, after);
    }
}
