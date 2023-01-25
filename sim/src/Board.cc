#include "Board.hh"
#include "Util.hh"

#include "avr_spi.h"
#include "avr_ioport.h"

#include <iostream>

Board::Board(avr_t *avr_):
    avr(avr_)
{
    avr_irq_register_fun(
        avr_io_getirq(avr, AVR_IOCTL_SPI_GETIRQ(0), SPI_IRQ_OUTPUT),
        [this](avr_irq_t* irq, uint32_t value) {
            this->mosi(value);
        });
}

void Board::run()
{
    avr_run(avr);
}

void Board::miso(uint8_t value)
{
    avr_raise_irq(
        avr_io_getirq(avr, AVR_IOCTL_SPI_GETIRQ(0), SPI_IRQ_INPUT),
        value);
}

void Board::mosi(uint8_t value)
{
    std::cout << "MOSI " << std::hex << int(value) << std::endl;
}
