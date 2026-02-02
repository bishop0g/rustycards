#[derive(Clone, Copy, Debug)]
pub enum Suits {
    H, // Hearts
    D, // Diamonds
    S, // Spades
    C, // Clubs
    J, // Jokers
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub suit: Suits,
    pub rank: u8,
    pub faceup: bool,
    pub wildcard: bool,
}

impl Card {
    pub fn print_rank_name(&self) -> String {
        // Returns the full name of the Card's Rank
        match self.rank {
            1 | 14 => "Ace".to_string(),
            11 => "Jack".to_string(),
            12 => "Queen".to_string(),
            13 => "King".to_string(),
            _ => self.rank.to_string(),
        }
    }

    pub fn print_rank_char(&self) -> String {
        // Returns the single-character representation of the Card's Rank
        match self.rank {
            1 | 14 => "A".to_string(),
            10 => "T".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            _ => self.rank.to_string(),
        }
    }

    pub fn print_suit_name(&self) -> &str {
        // Returns the full name of the Card's Suit
        match self.suit {
            Suits::H => "Hearts",
            Suits::D => "Diamonds",
            Suits::S => "Spades",
            Suits::C => "Clubs",
            Suits::J => "Joker",
        }
    }

    pub fn print_suit_char(&self) -> &str {
        // Returns the single-character representation of the Card's Suit
        match self.suit {
            Suits::H => "H",
            Suits::D => "D",
            Suits::S => "S",
            Suits::C => "C",
            Suits::J => "J",
        }
    }

    pub fn print_name(&self) {
        // Prints the card's rank and suit, if it is face up.
        if self.faceup {
            let rank = self.print_rank_name();
            let suit = self.print_suit_name();
            println!("{rank} of {suit}");
        }
    }

    pub fn print_id(&self) {
        // Prints the 2-character representation of the card
        if self.faceup {
            let rank = self.print_rank_char();
            let suit = self.print_suit_char();
            println!("{rank}{suit}");
        }
    }

    pub fn set_wild_value(&mut self, new_value: u8) {
        if self.wildcard {
            self.rank = new_value;
        }
    }
}
