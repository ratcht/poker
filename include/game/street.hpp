#ifndef STREET_HPP
#define STREET_HPP

#include "game.hpp"

class Street {
public:
  virtual void deal();
};


class FlopCommand : Street {
  Game* game;
public:
  FlopCommand(Game* game) : game(game) {}
  void deal() override {
    for (int i = 0; i < 3; ++i) {
      // Card drawn = game->deck->draw_card();

    }
  }
};

class TurnCommand : Street {
  Game* game;
public:
  TurnCommand(Game* game) : game(game) {}
  void deal() override {

  }
};


class RiverCommand : Street {
  Game* game;
public:
  RiverCommand(Game* game) : game(game) {}
  void deal() override {

  }
};


#endif // STREET_HPP
