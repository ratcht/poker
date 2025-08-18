#ifndef STREET_HPP
#define STREET_HPP

#include "game/engine/engine.hpp"

enum State {
  WAITING=-1,
  FLOP,
  TURN,
  RIVER,
  SHOWDOWN
};


class StateCommand {

// api
protected:
Card draw_card_(Game* game) { return game->draw_card_(); }
void update_board_(Game* game, Card card, int position) { game->update_board_(card, position); }

public:
  virtual void execute();
};

class FlopCommand : StateCommand {
  Game* game;
public:
  FlopCommand(Game* game) : game(game) {}
  void execute() override {
    // burn 3 cards
    for (int i = 0; i < 3; ++i) {
      Card _ = draw_card_(game);
    }

    for (int i = 0; i < 3; ++i) {
      Card drawn = draw_card_(game);
      update_board_(game, drawn, i);
    }
  }
};

class TurnCommand : StateCommand {
  Game* game;
public:
  TurnCommand(Game* game) : game(game) {}
  void execute() override {
    Card _ = draw_card_(game); // burn

    Card drawn = draw_card_(game);
    update_board_(game, drawn, 3);
  }
};


class RiverCommand : StateCommand {
  Game* game;
public:
  RiverCommand(Game* game) : game(game) {}
  void execute() override {
    Card _ = draw_card_(game); // burn

    Card drawn = draw_card_(game);
    update_board_(game, drawn, 4);
  }
};


#endif // STREET_HPP
