use crate::engine::action::*;
use crate::models::player::{PlayerStrategy, PlayerView};

pub struct DefaultStrategy;

// always folds
impl PlayerStrategy for DefaultStrategy{
  fn decide_action(&mut self, view: &PlayerView, time_limit: usize) -> Action {
    Action::fold(view)
  }
}
