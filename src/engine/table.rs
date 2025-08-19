use crate::models::{deck::Deck, card::Card, player::Player};
use crate::types;

pub struct Table {
  deck: Deck,
  board: [Option<Card>; 5],
  players: Vec<Player>,
  button: usize,
}

impl Table {
  pub fn new(num_players: usize) -> Result<Self, types::TableError> {
    if num_players < 2 || num_players > 10 {
      return Err(types::TableError("Invalid number of players".to_string()));
    }

    let mut players = Vec::with_capacity(num_players);
    for i in 0..num_players {
      players.push(Player::new(i));
    }

    Ok(Table {
      deck: Deck::new(),
      board: [None; 5],
      players,
      button: 0,
    })
  }

  pub fn reset(&mut self) {
    self.deck.reset();
    self.deck.shuffle();
    self.board = [None; 5];
  }

  pub fn move_button(&mut self) {
    self.button = (self.button + 1) % self.players.len();
  }

  pub fn draw_card(&mut self) -> Result<Card, types::DeckError> {
    self.deck.draw_card()
  }

  pub fn get_from_board(&self, position: usize) -> Result<Option<Card>, types::TableError> {
    if position >= 5 {
      return Err(types::TableError("Invalid board position".to_string()));
    }
    Ok(self.board[position])
  }

  pub fn update_board(&mut self, card: Card, position: usize) -> Result<(), types::TableError> {
    if position >= 5 {
      return Err(types::TableError("Invalid board position".to_string()));
    }
    if self.board[position].is_some() {
      return Err(types::TableError("Board position already occupied".to_string()));
    }
    self.board[position] = Some(card);
    Ok(())
  }
}

impl std::fmt::Display for Table {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Table with {} players:", self.players.len())?;
    writeln!(f, "Button position: {}", self.button)?;

    write!(f, "Board: ")?;
    for (i, card) in self.board.iter().enumerate() {
      match card {
        Some(c) => write!(f, "{} ", c)?,
        None => write!(f, "__ ")?,
      }
      if i == 2 {
        write!(f, "| ")?; // Separate flop from turn/river
      }
    }
    writeln!(f)?;

    writeln!(f, "Cards remaining in deck: {}", self.deck.get_cards_remaining())?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_table_valid_players() {
    let table = Table::new(6).unwrap();
    assert_eq!(table.players.len(), 6);
    assert_eq!(table.button, 0);
  }

  #[test]
  fn new_table_invalid_players() {
    assert!(Table::new(1).is_err());
    assert!(Table::new(11).is_err());
  }

  #[test]
  fn board_operations() {
    let mut table = Table::new(2).unwrap();

    // Initially empty
    assert_eq!(table.get_from_board(0).unwrap(), None);

    // Draw and place card
    let card = table.draw_card().unwrap();
    table.update_board(card, 0).unwrap();
    assert_eq!(table.get_from_board(0).unwrap(), Some(card));

    // Invalid position
    assert!(table.get_from_board(5).is_err());
    assert!(table.update_board(card, 5).is_err());
  }

  #[test]
  fn button_movement() {
    let mut table = Table::new(3).unwrap();
    assert_eq!(table.button, 0);

    table.move_button();
    assert_eq!(table.button, 1);

    table.move_button();
    assert_eq!(table.button, 2);

    table.move_button(); // Should wrap around
    assert_eq!(table.button, 0);
  }

  #[test]
  fn reset_clears_board() {
    let mut table = Table::new(2).unwrap();

    // Place a card
    let card = table.draw_card().unwrap();
    table.update_board(card, 0).unwrap();
    assert!(table.get_from_board(0).unwrap().is_some());

    // Reset
    table.reset();
    assert_eq!(table.get_from_board(0).unwrap(), None);
    assert_eq!(table.deck.get_cards_remaining(), 52);
  }

  #[test]
  fn cannot_overwrite_board_position() {
    let mut table = Table::new(2).unwrap();

    let card1 = table.draw_card().unwrap();
    let card2 = table.draw_card().unwrap();

    table.update_board(card1, 0).unwrap();

    // Should fail to overwrite
    assert!(table.update_board(card2, 0).is_err());
  }
}
