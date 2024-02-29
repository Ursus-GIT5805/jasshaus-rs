use serde::{Serialize, Deserialize};

//const NUM_COLORS: usize = 4;
//const NUM_NUMBERS: usize = 9;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, std::fmt::Debug)]
pub struct Card {
    pub color: u8,
    pub number: u8
}

impl Card {
    pub fn new(color: u8, number: u8) -> Self { Card { color, number } }
    pub fn from_id(card_id: u8) -> Self { Card { color: card_id / 9, number: card_id % 9 } }
    pub fn get_id(&self) -> u8 { self.color * 9 + self.number }
}
impl Default for Card {
    fn default() -> Self { Card { color: 4, number: 2 } }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, std::fmt::Debug)]
pub struct Show {
    pub color: u8,
    pub number: u8,
    pub row: u8,
}

impl Show {
    pub fn new(color: u8, number: u8, row: u8) -> Self { Show { color, number, row } }
    pub fn as_cards(&self) -> Vec<Card> {
        match self.row {
            1 => (0..4).map(|i| Card::new(i, self.number)).collect(),
            _ => (0..self.row).map(|i| Card::new(self.color,self.number+i)).collect(),
        }
    }
}
impl Default for Show {
    fn default() -> Self { Show { color: 4, number: 2, row: 0 } }
}


#[derive(std::fmt::Debug)]
pub enum ShowError {
    Illegal,
    DoesNotContain,
    IsSubset,
}

#[derive(Default)]
#[derive(PartialEq, Eq, std::fmt::Debug)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Cardset {
    list: u64,
}

impl Cardset {
    pub fn insert( &mut self, card: Card ) {
        self.list |= 1u64 << card.get_id();
    }

    pub fn erase( &mut self, card: Card ) {
        self.list &= !(1u64 << card.get_id());
    }

    // Inserts if the card doesn't exist, and remove otherwise
    pub fn toggle( &mut self, card: Card ) {
        self.list ^= 1u64 << card.get_id();
    }

    pub fn clear( &mut self ) {
	self.list = 0;
    }

    pub fn contains( &self, card: Card ) -> bool {
        self.list & (1 << card.get_id()) != 0
    }

    pub fn has_color( &self, color: u8 ) -> bool {
        self.list & (0x01FF << (color*9)) != 0
    }

    pub fn only_has_color(&self, color :u8) -> bool {
        self.list & (0x01FF << (color*9)) == self.list
    }

    pub fn as_bytes(&self) -> [u8; 5] {
        let mut out: [u8; 5] = [0; 5];
        for i in 0..5 { out[4-i] = ((self.list >> (i*8)) % 0x100) as u8; }
        out
    }

    pub fn count_color(&self, color: u8) -> u32 {
        (self.list & (0x01FF << (color*9))).count_ones()
    }

    pub fn has_stronger_trumpf(&self, card: Card) -> bool {
        // Make a bitmask where each card with higher number is marked with 1
        let mask = match card.number {
            0 => 0b111111110,
            1 => 0b111111100,
            2 => 0b111111000,
            3 => 0b000100000,
            4 => 0b111101000,
            5 => 0,
            6 => 0b110101000,
            7 => 0b100101000,
            8 => 0b000101000,
            _ => 0,
        } << card.color * 9;

        // If the list contains something of the mask, there is a better trumpf
        self.list & mask != 0
    }

    pub fn has_show(&self, show: Show) -> Result<(),ShowError> {
        // Rows that do not exist are not legal
        if show.row < 1 || 9 < show.row || show.row == 2 { return Err(ShowError::Illegal); }
        if show.row != 1 && show.number + show.row > 9 { return Err(ShowError::Illegal); }


        // Handle the show for 4-equals
        if show.row == 1 {
            let mask: u64 = 0x0008040201 << show.number;
            return if self.list & mask == mask { Ok(())
            } else { Err(ShowError::DoesNotContain) };
        }

        // Let the mask look like this 0000111000...
        let mask: u64 = {
            let tmp: u64 = 1 << (show.color * 9 + show.number);
            (tmp << show.row) - tmp
        };

        if self.list & mask != mask { return Err(ShowError::DoesNotContain); }

        // Everything is fine so far but... He could've shown only a smaller subset of another show!
        if show.number > 0 { // Can you shift right?
            // If this is also possible, then you could show a row of {show.row+1}
            if self.list & (mask >> 1) == (mask >> 1) { return Err(ShowError::IsSubset); }
        }

        if show.number + show.row == 9 { Ok(()) } // Same procedure with shifting left
        else {
            if self.list & (mask << 1) == mask << 1 {
                Err(ShowError::IsSubset)
            } else { Ok(()) }
        }
    }

    pub fn as_vec(&self) -> std::vec::Vec<Card> {
        let mut out = std::vec::Vec::new();
        for i in 0..36 {
            let card = Card::new(i/9, i%9);
            if self.contains( card ) { out.push( card ); }
        }
        out
    }

    pub fn new( init_list: std::vec::Vec<Card> ) -> Self {
        let mut list: u64 = 0;
        for card in init_list { list |= 1u64 << card.get_id(); }
        Cardset { list }
    }
}
