#include "game/card/deck.hpp"
#include "game/constants.hpp"

#include <algorithm>  // for std::shuffle
#include <random>     // for std::mt19937, std::random_device



Deck::Deck() : cards_remaining(52) {
  for (uint8_t i = 0; i < 52; ++i) {
    cards[i] = i;
  }
}

/* deck */
Card Deck::draw_card() {
  if (this->cards_remaining == 0) {
    throw std::runtime_error("No cards left to draw");
  }

  this->cards_remaining--;

  uint8_t card_i = this->cards[this->cards_remaining];

  uint8_t value = card_i % 13;
  uint8_t suit = card_i / 13;

  return Card(value, suit);
}


void Deck::shuffle() {
  std::random_device rd;
  std::mt19937 g(rd());

  // Shuffle the vector
  std::shuffle(this->cards, this->cards+52, g);
}


void Deck::reset(){
  for (uint8_t i = 0; i < 52; ++i) {
    cards[i] = i;
  }
  cards_remaining = 52;
}
