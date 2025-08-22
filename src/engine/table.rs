use crate::models::{deck::Deck, card::Card, player::Player};
use crate::types;

pub struct TableInfo {
  buy_in: usize,
  max_players: usize,
}
pub struct Table {
  info: TableInfo,
  deck: Deck,
  board: [Option<Card>; 5],
  seats: Vec<Option<Player>>,
  button: usize,
}

impl Table {
  pub fn new(max_players: usize, buy_in: usize) -> Result<Self, types::TableError> {
    if max_players < 2 || max_players > 10 {
      return Err(types::TableError("Invalid number of players".to_string()));
    }

    let table_info = TableInfo{
      buy_in,
      max_players,
    };

    Ok(Table {
      info: table_info,
      deck: Deck::new(),
      board: [None; 5],
      seats: Vec::with_capacity(max_players),
      button: 0,
    })
  }

  pub fn reset(&mut self) {
    self.deck.reset();
    self.deck.shuffle();
    self.board = [None; 5];
  }

  pub fn move_button(&mut self) {
    // loop until we find a valid seat
    // if no one to move button to, does nothing
    for _ in 0..self.seats.len() {
      self.button = (self.button + 1) % self.seats.len();
      if self.seats.get(self.button).is_some() {
        break;
      }
    }

  }

  pub fn seat_player(&mut self, player: Player) -> Result<(), types::TableError> {
    for i in 0..self.seats.len() {
      if self.seats.get(i).is_none() {
        self.seats[i] = Some(player);
        return Ok(());
      }
    }
    Err(types::TableError("Table is full!".to_string()))
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
    writeln!(f, "Table with {} seats:", self.seats.len())?;
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
    let table = Table::new(6, 100).unwrap();
    assert_eq!(table.seats.capacity(), 6);
    assert_eq!(table.button, 0);
  }

  #[test]
  fn new_table_invalid_players() {
    assert!(Table::new(1, 100).is_err());
    assert!(Table::new(11, 100).is_err());
  }

  #[test]
  fn board_operations() {
    let mut table = Table::new(2, 100).unwrap();

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
    let mut table = Table::new(3, 100).unwrap();
    for _ in 0..3 {
      assert!(table.seat_player(Player::new_default()).is_ok());
    }
    assert_eq!(table.button, 0);

    table.move_button();
    assert_eq!(table.button, 1);

    table.move_button();
    assert_eq!(table.button, 2);

    table.move_button(); // Should wrap around
    assert_eq!(table.button, 0);
  }

  #[test]
  fn button_movement_empty_table() {
    let mut table = Table::new(4, 100).unwrap();
    assert!(table.seat_player(Player::new_default()).is_ok());

    assert_eq!(table.button, 0);

    table.move_button();
    assert_eq!(table.button, 0);

  }

  #[test]
  fn reset_clears_board() {
    let mut table = Table::new(2, 100).unwrap();

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
    let mut table = Table::new(2, 100).unwrap();

    let card1 = table.draw_card().unwrap();
    let card2 = table.draw_card().unwrap();

    table.update_board(card1, 0).unwrap();

    // Should fail to overwrite
    assert!(table.update_board(card2, 0).is_err());
  }
}
