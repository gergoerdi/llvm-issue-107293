ARTEFACT	= target/avr-atmega32u4/release/deps/worduino_avr-0fc01cc05b3aa374
OBJ 		:= obj-${shell $(CC) -dumpmachine}

all: bad.log good.log

%.log: %.elf
	make -C sim
	sim/$(OBJ)/Main.elf $< > $@

worduino-avr/$(ARTEFACT).%:
	cd worduino-avr && cargo build --release

bad.%: worduino-avr/$(ARTEFACT).%
	cp -f $< $@

