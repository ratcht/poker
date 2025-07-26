#ifndef CARD_HPP
#define CARD_HPP

#include "constants.hpp"

#include <cstdint>
#include <string>

/* card class */
class Card {
private:
  uint8_t value;              // 0-12 (23456789JQKA)
  uint8_t suit;       // 0-3

public:
  Card(uint8_t v, uint8_t s);

  uint8_t get_value() const{
    return value;
  }
  uint8_t get_suit() const{
    return suit;
  }

  std::string to_string() const;
};


#endif
