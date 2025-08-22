use crate::models::player::*;

pub enum ActionType {
  Fold,
  Check,
  Call,
  Bet(usize),
  Raise(usize),
  AllIn,
  Win,
}

pub struct Action {
  player_id: u64,
  action_type: ActionType,
  amount: usize,
  resulting_pot: usize,
}

impl Action {
  pub fn fold(view: &PlayerView) -> Self {
    Action {
      player_id: view.get_id(),
      action_type: ActionType::Fold,
      amount: 0,
      resulting_pot: view.get_pot_size(),
    }
  }
}

pub struct ValidAction {
  action_type: ActionType,
  min_amount: Option<usize>,
  max_amount: Option<usize>,
}
