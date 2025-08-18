#ifndef STREET_HPP
#define STREET_HPP

#include "game/engine/table.hpp"

enum State {
  WAITING=-1,
  PREFLOP,
  FLOP,
  TURN,
  RIVER,
  SHOWDOWN
};


class StateCommand {

// api
protected:
Card draw_card_(Table* table) { return table->draw_card_(); }
void update_board_(Table* table, Card card, int position) { table->update_board_(card, position); }

public:
  virtual void execute();
};

class FlopCommand : StateCommand {
  Table* table;
public:
  FlopCommand(Table* table) : table(table) {}
  void execute() override {
    // burn 3 cards
    for (int i = 0; i < 3; ++i) {
      Card _ = draw_card_(table);
    }

    for (int i = 0; i < 3; ++i) {
      Card drawn = draw_card_(table);
      update_board_(table, drawn, i);
    }
  }
};

class TurnCommand : StateCommand {
  Table* table;
public:
  TurnCommand(Table* table) : table(table) {}
  void execute() override {
    Card _ = draw_card_(table); // burn

    Card drawn = draw_card_(table);
    update_board_(table, drawn, 3);
  }
};


class RiverCommand : StateCommand {
  Table* table;
public:
  RiverCommand(Table* table) : table(table) {}
  void execute() override {
    Card _ = draw_card_(table); // burn

    Card drawn = draw_card_(table);
    update_board_(table, drawn, 4);
  }
};


#endif // STREET_HPP
