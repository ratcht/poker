#include <iostream>

#include "game/card/deck.hpp"


int main(int argc, char* argv[]) {
  Deck deck = Deck();

  for (int i = 0; i < 5; ++i) {
    Card card = deck.draw_card();
    std::cout << card.to_string() << std::endl;
  }

  std::cout<<"Shuffling..."<<std::endl;

  deck.reset();
  deck.shuffle();

  for (int i = 0; i < 5; ++i) {
    Card card = deck.draw_card();
    std::cout << card.to_string() << std::endl;
  }

  std::cout<<"Shuffling..."<<std::endl;

  deck.reset();
  deck.shuffle();

  for (int i = 0; i < 5; ++i) {
    Card card = deck.draw_card();
    std::cout << card.to_string() << std::endl;
  }

  return 0;
}
