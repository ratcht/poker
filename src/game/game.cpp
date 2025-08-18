#include "game/game.hpp"

Game::Game(int num_players) {
  if (num_players < 0 || num_players > 9) throw std::invalid_argument("Invalid number of players (0 < p < 9");
  this->num_players = num_players;


}
