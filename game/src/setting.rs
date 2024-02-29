/*

//TODO some fancy comments EVERYWHERE!
This struct contains all the different settings you can choose before a game.

*/

use crate::card::*;
use crate::playtype::*;

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, std::fmt::Debug)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum StartingCondition {
    RANDOM,
    PLAYER(u8),
    CARD(Card),
}

#[derive(PartialEq, Eq, std::fmt::Debug)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum PointRule {
    PLAY,
    SHOW,
    MARRIAGE,
}

#[derive(PartialEq, Eq, std::fmt::Debug)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Setting {
    pub max_points: u16, // used
    pub point_recv_order: [PointRule; 3], // used
    pub playtype_multiplier: [u8; NUM_PLAYTYPES], // used

    pub allow_playtype: [bool; NUM_PLAYTYPES], // used
    pub allow_misere: bool, // used
    pub allow_pass: bool, // used

    pub automatic_show: bool,
    pub automatic_marriage: bool,

    pub startcondition: StartingCondition, // used
    pub apply_startcondition_on_revanche: bool, // used

    pub react_time: u32,

    pub passed_player_begins: [bool; NUM_PLAYTYPES], // used

    pub show_points_maximum: u16, // used
}

impl Default for Setting {
    fn default() -> Setting {
        let mut beg_passed = [true; NUM_PLAYTYPES];
        beg_passed[SHIELD as usize] = false;
        beg_passed[ACORN as usize] = false;
        beg_passed[ROSE as usize] = false;
        beg_passed[BELL as usize] = false;

        Setting {
            max_points: 1000,
            point_recv_order: [PointRule::MARRIAGE, PointRule::SHOW, PointRule::PLAY],
            playtype_multiplier: [1; NUM_PLAYTYPES],

            allow_playtype: [true; NUM_PLAYTYPES],
            allow_misere: true,
            allow_pass: true,

            automatic_show: false,
            automatic_marriage: true,

            startcondition: StartingCondition::CARD(Card::new(0, 4)),
            apply_startcondition_on_revanche: false,

            react_time: 0,

            passed_player_begins: beg_passed,

            show_points_maximum: 300,
        }
    }
}
