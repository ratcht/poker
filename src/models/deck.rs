use rand::seq::SliceRandom;
use rand::thread_rng;
use super::card::Card;
use crate::types;

pub struct Deck {
  cards: [u8; 52], // unshuffled is 0-12 for CLUBS, SPADES, HEARTS, DIAMONDS in that order
  cards_remaining: u8,
}

impl Deck {
  pub fn new() -> Self {
    let mut cards = [0u8; 52];
    for i in 0..52 {
      cards[i] = i as u8;
    }

    Deck {
      cards,
      cards_remaining: 52,
    }
  }

  pub fn get_cards_remaining(&self) -> u8 {
    self.cards_remaining
  }

  pub fn draw_card(&mut self) -> Result<Card, types::DeckError> {
    if self.cards_remaining == 0 {
      return Err(types::DeckError("No cards left to draw".to_string()));
    }
    self.cards_remaining -= 1;
    let card_i = self.cards[self.cards_remaining as usize];
    let value = (card_i % 13) as i8;
    let suit = (card_i / 13) as i8;

    Card::new(value, suit).map_err(|type_err| {
      types::DeckError(format!("Failed to create card: {}", type_err))
    })
  }

  pub fn shuffle(&mut self) {
    let mut rng = thread_rng();
    self.cards.shuffle(&mut rng);
  }

  pub fn reset(&mut self) {
    for i in 0..52 {
      self.cards[i] = i as u8;
    }
    self.cards_remaining = 52;
  }
}
