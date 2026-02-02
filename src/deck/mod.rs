use rand::seq::SliceRandom;

pub mod card; // Declare the public card module (loads card.rs)
use self::card::{Card, Suits}; // Import the necessary types

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    // Constructs a new deck of cards, with options for aces high and jokers.
    // The deck is shuffled by default.
    pub fn new(ace_high: bool, jokers: bool, wildcard_value: u8) -> Deck {
        let capacity = if jokers { 54 } else { 52 };
        let mut cards = Vec::with_capacity(capacity);

        let (start_rank, end_rank) = if ace_high { (2, 14) } else { (1, 13) };

        let suits = [Suits::H, Suits::D, Suits::S, Suits::C];

        for &suit in &suits {
            for rank in start_rank..=end_rank {
                cards.push(Card {
                    rank,
                    suit,
                    faceup: false,
                    wildcard: false,
                });
            }
        }

        if jokers {
            for _ in 0..2 {
                cards.push(Card {
                    rank: 0,
                    suit: Suits::J,
                    faceup: false,
                    wildcard: false,
                });
            }
        }

        let mut deck = Deck { cards };
        deck.shuffle();
        if wildcard_value == 0 {
            deck.set_wildcards(wildcard_value);
        }
        return deck;
    }

    // Shuffles the cards in the deck in-place.
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn set_wildcards(&mut self, wild_rank: u8) {
        for card in &mut self.cards {
            if card.rank == wild_rank {
                card.wildcard = true;
            }
        }
    }
}
