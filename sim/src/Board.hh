#pragma once

class Board;

#include "sim_avr.h"

class Board {
public:
    avr_t *avr;

public:
    Board(avr_t *avr_);

    void run();
    void miso(uint8_t value);
    void mosi(uint8_t value);
};
