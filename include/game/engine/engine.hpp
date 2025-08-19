#ifndef GAME_HPP
#define GAME_HPP

#include "game/engine/table.hpp"
#include "game/engine/state.hpp"

class Engine {
  friend class StateCommand;

  Table *table;
  State state;

  Engine(int num_players);
  ~Engine();

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
  int undo_last_action();

};

#endif // GAME_HPP
