#ifndef GAME_CONSTANTS_HPP
#define GAME_CONSTANTS_HPP

#include <cstdint>
#include <string>

/* deck constants */
#define NUM_CARDS_IN_DECK 52
#define NUM_SUITS 4

/* suits */
#define CLUBS 0
#define SPADES 1
#define HEARTS 2
#define DIAMONDS 3
const uint8_t suits[4] = {CLUBS, SPADES, HEARTS, DIAMONDS};
const std::string suits_symbol[4] = {"♣", "♠", "♥", "♦"};
const std::string suits_letter[4] = {"C", "S", "H", "D"};


#endif
