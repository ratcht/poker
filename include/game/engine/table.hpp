#ifndef TABLE_HPP
#define TABLE_HPP

#include "game/cards/deck.hpp"
#include "game/engine/player.hpp"

class Table {
  friend class StateCommand;

  Deck* deck;
  Card board[5];
  Player* players;

  int button;
  int num_players;

  Table(int num_players);
  ~Table();

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

  Card get_from_board(int position) {
    if (position < 0 || position > 4) throw std::invalid_argument("Invalid board position");
    return board[position];
  }
};

#endif // TABLE_HPP
