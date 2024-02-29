use crate::card::*;

use serde::{Serialize, Deserialize};

pub const UPDOWN: u8 = 0;
pub const DOWNUP: u8 = 1;
pub const SHIELD: u8 = 2;
pub const ACORN: u8 = 3;
pub const ROSE: u8 = 4;
pub const BELL: u8 = 5;
pub const SLALOMUPDOWN: u8 = 6;
pub const SLALOMDOWNUP: u8 = 7;
pub const GUSCHTI: u8 = 8;
pub const MARY: u8 = 9;
pub const PASS: u8 = 10;
pub const NONE: u8 = 255;

pub const NUM_PLAYTYPES: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, std::fmt::Debug)]
pub struct Playtype {
    pub playtype: u8,
    pub ruletype: u8,
    pub misere: bool,
    pub passed: bool,
}

impl Playtype {
    pub fn new() -> Self { Self::default() }

    pub fn is_trumpf(&self) -> bool {
        SHIELD <= self.playtype && self.playtype <= BELL
    }

    pub fn get_trumpf_color(&self) -> u8 {
        self.playtype - 2
    }

    pub fn is_color_trumpf(&self, color: u8) -> bool {
        color + 2 == self.playtype
    }

    pub fn is_card_stronger(&self, current: Card, new: Card) -> bool {
        match self.ruletype {
            UPDOWN => { current.color == new.color && current.number < new.number }, // Basic updown
            DOWNUP => { current.color == new.color && current.number > new.number }, // Basic downup
            SHIELD | ACORN | ROSE | BELL => {
                let tcur = self.is_color_trumpf(current.color);
                let tnew = self.is_color_trumpf(new.color);

                if !tcur && !tnew {
                    current.color == new.color && current.number < new.number // Basic updown
                } else if tcur != tnew {
                    tnew // If tnew is trumpf, tcur wouldn't and vice versa
                } else { // both are trumpf!
                    let order: [u8; 9] = [0, 1, 2, 7, 3, 8, 4, 5, 6];
                    order[ current.number as usize ] < order[new.number as usize] // Basic, but with trumpf order!
                }
            },
            _ => false // nothing announced, no rules is there
        }
    }

    pub fn get_card_value(&self, card: Card) -> u16 {
        match self.playtype {
            UPDOWN | SLALOMUPDOWN | GUSCHTI => {
                let values: [u16; 9] = [0, 0, 8, 0, 10, 2, 3, 4, 11];
                values[ card.number as usize ]
            },
            DOWNUP | SLALOMDOWNUP | MARY => {
                let values: [u16; 9] = [11, 0, 8, 0, 10, 2, 3, 4, 0];
                values[ card.number as usize ]
            },
            SHIELD | ACORN | ROSE | BELL => {
                let trumpf = self.is_color_trumpf(card.color) as u16;
                let values: [u16; 9] = [0, 0, 0, 14*trumpf, 10, 2 + 18*trumpf, 3, 4, 11];
                values[ card.number as usize ]

            },
            _ => 0
        }
    }

    pub fn is_show_stronger(&self, current: Show, new: Show) -> bool {
        let pcur = self.get_show_value(current);
        let pnew = self.get_show_value(new);
        if pcur != pnew { return pcur < pnew; }
        // The shows has both the equal points
        if current.row != new.row { return current.row < new.row; } // You don't have to check for the 4-equals seperately
        // The shows are equally long

        if current.number != new.number {
            return match self.playtype {
                DOWNUP | SLALOMDOWNUP | MARY => current.number > new.number,
                _ => current.number < new.number
            };
        }

        // They haev equal points, row, and number, but not color!
        // So if the new show's color is trumpf, it's better!
        return self.is_color_trumpf(new.color);
    }

    pub fn get_show_value(&self, show: Show) -> u16 {
        match show.row {
	    0 => 0,
            1 => {
                match show.number {
                    3 => 150, // 9
                    5 => 200, // boy
                    _ => 100,
                }
            }
            2 | 3 => 20,
            x => 50*(x as u16-3),
        }
    }
}

impl Default for Playtype {
    fn default() -> Self {
        Playtype {
            playtype: NONE,
            ruletype: NONE,
            misere: false,
            passed: false,
        }
    }
}
