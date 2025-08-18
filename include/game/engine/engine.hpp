#ifndef GAME_HPP
#define GAME_HPP

#include "game/engine/table.hpp"
#include "game/engine/state.hpp"

class Game {
  friend class StateCommand;

  Table *table;
  State state;

  Game(int num_players);
  ~Game();

protected:
  void set_state(State new_state) {
    state = new_state;
  }

  void reset_state() {
    state = State::WAITING;
  }

public:
  State get_state() {
    return state;
  }

  int submit_action(PlayerAction *action); // validate action
};

#endif // GAME_HPP
