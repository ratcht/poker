#ifndef DECK_HPP
#define DECK_HPP

#include <cstdint>

#include "card.hpp"

/* deck class */
class Deck {
private:
  uint8_t cards[52]; // unshuffled is 0-12 for CLUBS, SPADES, HEARTS, DIAMONDS in that order
  uint8_t cards_remaining;
public:
  Deck();

  uint8_t get_cards_remaining() const {
    return cards_remaining;
  }

  Card draw_card();
  void shuffle();
  void reset();
};



#endif //DECK_HPP
