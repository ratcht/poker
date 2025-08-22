use uuid::Uuid;

use super::card::Card;
use crate::engine::action::*;
use crate::engine::strategy::*;
use crate::types;

pub enum PlayerStatus {
  Active,
  Folded,
  AllIn,
  SittingOut,
}

pub struct Player {
  id: u64,
  stack: usize,
  current_bet: isize,
  status: PlayerStatus,  // Active, Folded, AllIn
  strategy: Box<dyn PlayerStrategy>,
  hole_cards: Option<[Card; 2]>,
}

impl Player {
  pub fn new_default() -> Self {
    Player {
      id: 0,
      stack: 0,
      current_bet: -1,
      status: PlayerStatus::SittingOut,
      strategy: Box::new(DefaultStrategy {}),
      hole_cards: None,
    }
  }
  pub fn new(strategy: Box<dyn PlayerStrategy>, stack: usize) -> Result<Self, types::LogicError> {
    Ok(Player {
      id: Uuid::new_v4().as_u64_pair().0, // half of a uuid4
      stack: stack,
      current_bet: -1,
      status: PlayerStatus::SittingOut,
      strategy: strategy,
      hole_cards: None,
    })
  }


  pub fn set_hand(&mut self, hand: [Card; 2]) -> Result<(), types::LogicError> {
    self.hole_cards = Some(hand);

    Ok(())
  }
}


/* player views */
pub struct PublicPlayerView {
  id: u64,
  stack: usize,
  current_bet: usize,
  status: PlayerStatus,  // Active, Folded, AllIn
  position: usize, // Button = 0
}

pub struct PrivatePlayerView {
  id: u64,
  stack: usize,
  status: PlayerStatus,  // Active, Folded, AllIn
  hole_cards: Option<[Card; 2]>,
  position: usize, // Button = 0
}

pub struct PlayerView {
  player: PrivatePlayerView,
  board: Vec<Card>,
  pot_size: usize,
  players_info: Vec<PublicPlayerView>,  // Stack sizes, positions, current bets
  valid_actions: Vec<ValidAction>,      // What actions are legal now
  betting_history: Vec<Action>,   // All actions this hand
}

impl PlayerView {
  pub fn get_id(&self) -> u64 {
    self.player.id
  }

  pub fn get_pot_size(&self) -> usize {
    self.pot_size
  }
}


// players/Ai must implement this trait
pub trait PlayerStrategy: Send + Sync {
  // rcvs game state from player's perspective, returns action
  fn decide_action(&mut self, view: &PlayerView, time_limit: usize) -> Action;

  // callback once hand completed
  fn on_hand_complete(&mut self, result: &Vec<Action>) {}

  // callback once action submitted
  fn on_action_taken(&mut self, player_id: usize, action: &Action) {}
}
