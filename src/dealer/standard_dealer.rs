use card::Card;
use dealer::Dealer;
use rand::{self, Rng};

/// A dealer who attempts to make the game as fair as possible.
#[derive(Debug, Default)]
pub struct StandardDealer {
    deck: Vec<Card>,
}

impl Dealer for StandardDealer {
    fn open(&mut self) {
        self.deck = Card::deck(|face| match face {
            55 => 7,
            f if f % 11 == 0 => 5,
            f if f % 10 == 0 => 3,
            f if f % 5 == 0 => 2,
            _ => 1,
        });
        rand::thread_rng().shuffle(&mut self.deck)
    }

    fn deal(&mut self) -> Card {
        self.deck.pop().expect("dealer ran out of cards.")
    }
}
