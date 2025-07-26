#ifndef GAME_HPP
#define GAME_HPP

#include "deck.hpp"

class Game {
  Deck* deck;
  Card board[4];

  uint8_t num_players;

  Game(uint8_t num_players);
  ~Game();

public:

};

#endif // GAME_HPP
