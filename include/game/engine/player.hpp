#ifndef PLAYER_HPP
#define PLAYER_HPP

class Player {
  int id;
  int stack; // chips

  Player();
  ~Player();
};

enum PlayerActionType {
  FOLD=-1,
  CHECK,
  CALL,
  RAISE,
  ALL_IN,
};

class PlayerAction {
  Player player;
  PlayerActionType action_type;

  PlayerAction();
  ~PlayerAction();
};

#endif // PLAYER_HPP
