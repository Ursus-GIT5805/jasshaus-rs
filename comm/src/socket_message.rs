use serde::{Serialize, Deserialize};
use crate::voting::*;
use jasshaus_game::{
    Game,
    card::*,
    setting::*,
    playtype::*,
};

#[derive(Clone)]
#[derive(PartialEq, std::fmt::Debug, Serialize, Deserialize)]
pub enum SocketMessage {
    // Client <=> Server
    PlayCard(Card), // GAMEPLAY
    Announce(Playtype), // GAMEPLAY

    Vote(Vote, u8),
    NewVote(Votingtype),
    RtcSignaling(String, usize),

    // Client <= Server
    ShowPoints(u16,u8), // GAMEPLAY
    ShowList([[Show; 3]; 4]), // GAMEPLAY
    MarriageWouldWin(u8),

    ID(u8),
    PlayerJoined(u8),
    PlayerDisconnected(u8),
    SetAnnouncePlayer(u8),

    GameState(Game, Cardset),
    GameSetting(Setting),

    StartMating,
    StartGame,
    NewCards(Cardset),

    PlayerOrder([usize; 4]),

    // Client => Server
    PlayShow(Show), // GAMEPLAY
    Mate(u8),
}
