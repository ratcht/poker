#ifndef GAME_HPP
#define GAME_HPP

#include "game/cards/deck.hpp"

class Game {
  friend class StateCommand;



  Game(int num_players);
  ~Game();

// protected api
protected:
  Card draw_card_() {
    return deck->draw_card();
  }

  void update_board_(Card card, int position) {
    if (position < 0 || position > 4) throw std::invalid_argument("Invalid board position");
    board[position] = card;
  }

public:
  int get_num_players() {
    return num_players;
  }

  int get_stage() {
    return stage;
  }

  void next_stage() {
    stage++;
  }

  Card get_from_board(int position) {
    if (position < 0 || position > 4) throw std::invalid_argument("Invalid board position");
    return board[position];
  }
};

#endif // GAME_HPP
