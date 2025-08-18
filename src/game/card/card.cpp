#include "game/card/card.hpp"
#include "game/constants.hpp"


/* card */
Card::Card(uint8_t v, uint8_t s) {
  if (v < 0 || v > 12) throw std::invalid_argument("Invalid card value");
  if (s != CLUBS && s != SPADES && s != HEARTS && s != DIAMONDS) throw std::invalid_argument("Invalid suit");
  value = v;
  suit = s;
}

std::string Card::to_pretty_string() const{
  std::string display_value = std::string(1, "23456789TJQKA"[this->value]);
  std::string display_suit = suits_symbol[this->suit];
  return display_value + display_suit;
}

std::string Card::to_string() const{
  std::string display_value = std::string(1, "23456789TJQKA"[this->value]);
  std::string display_suit = suits_letter[this->suit];
  return display_value + display_suit;
}
