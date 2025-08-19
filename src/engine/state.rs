use super::table::Table;
use crate::models::{card::Card};
use crate::types;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
  Waiting = -1,
  Preflop = 0,
  Flop = 1,
  Turn = 2,
  River = 3,
  Showdown = 4,
}

pub trait StateCommand {
  fn execute(&mut self, table: &mut Table) -> Result<(), types::StateMachineError>;

  fn draw_card(&self, table: &mut Table) -> Result<Card, types::StateMachineError> {
    table.draw_card().map_err(|e| types::StateMachineError(format!("Failed to draw card: {}", e)))
  }

  fn update_board(&self, table: &mut Table, card: Card, position: usize) -> Result<(), types::StateMachineError> {
    table.update_board(card, position).map_err(|e| types::StateMachineError(format!("Failed to update board: {}", e)))
  }
}

pub struct PreflopCommand;

impl StateCommand for PreflopCommand {
  fn execute(&mut self, table: &mut Table) -> Result<(), types::StateMachineError> {
    // deal hole cards
    // TODO: fill this in

    Ok(())
  }
}


pub struct FlopCommand;

impl StateCommand for FlopCommand {
  fn execute(&mut self, table: &mut Table) -> Result<(), types::StateMachineError> {
    // burn 3 cards
    for _ in 0..3 {
      self.draw_card(table)?;
    }

    // deal flop
    for i in 0..3 {
      let card = self.draw_card(table)?;
      self.update_board(table, card, i)?;
    }

    Ok(())
  }
}

pub struct TurnCommand;

impl StateCommand for TurnCommand {
  fn execute(&mut self, table: &mut Table) -> Result<(), types::StateMachineError> {
    // burn 1 card
    self.draw_card(table)?;

    // deal turn
    let card = self.draw_card(table)?;
    self.update_board(table, card, 3)?;

    Ok(())
  }
}

pub struct RiverCommand;

impl StateCommand for RiverCommand {
  fn execute(&mut self, table: &mut Table) -> Result<(), types::StateMachineError> {
    // burn 1 card
    self.draw_card(table)?;

    // deal river
    let card = self.draw_card(table)?;
    self.update_board(table, card, 4)?;

    Ok(())
  }
}

// state machine to manage transitions
pub struct StateMachine {
  current_state: State,
}

impl StateMachine {
  pub fn new() -> Self {
    StateMachine {
      current_state: State::Waiting,
    }
  }

  pub fn current_state(&self) -> State {
    self.current_state
  }

  pub fn advance_street(&mut self, table: &mut Table) -> Result<(), types::StateMachineError> {
    match self.current_state {
      State::Waiting => {
        let mut preflop_command = PreflopCommand;
        preflop_command.execute(table)?;
        self.current_state = State::Preflop;
        Ok(())
      }
      State::Preflop => {
        let mut flop_command = FlopCommand;
        flop_command.execute(table)?;
        self.current_state = State::Flop;
        Ok(())
      }
      State::Flop => {
        let mut turn_command = TurnCommand;
        turn_command.execute(table)?;
        self.current_state = State::Turn;
        Ok(())
      }
      State::Turn => {
        let mut river_command = RiverCommand;
        river_command.execute(table)?;
        self.current_state = State::River;
        Ok(())
      }
      State::River => {
        self.current_state = State::Showdown;
        Ok(())
      }
      State::Showdown => {
        Err(types::StateMachineError("Cannot advance past showdown".to_string()))
      }
    }
  }

  pub fn reset(&mut self) {
    self.current_state = State::Waiting;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn state_enum_values() {
    assert_eq!(State::Waiting as i8, -1);
    assert_eq!(State::Preflop as i8, 0);
    assert_eq!(State::Flop as i8, 1);
  }
}
