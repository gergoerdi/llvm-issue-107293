ARTEFACT	= target/avr-atmega32u4/release/deps/worduino_avr-0fc01cc05b3aa374
OBJ 		:= obj-${shell $(CC) -dumpmachine}
#RUSTFLAGS	= -Zrandomize-layout
RUSTFLAGS	=

all: bad.log good.log
	diff -u $?

clean:
	rm -rf worduino-avr/bad worduino-avr/good worduino-avr/target bad.log good.log

%.log: worduino-avr/%/worduino-avr.elf
	make -C sim
	sim/$(OBJ)/Main.elf $< > $@

worduino-avr/good/worduino-avr.elf:
	cd worduino-avr && RUSTFLAGS="$(RUSTFLAGS)" cargo build -Z unstable-options --out-dir good --release

worduino-avr/bad/worduino-avr.elf:
	cd worduino-avr && RUSTFLAGS="$(RUSTFLAGS)" cargo build -Z unstable-options --out-dir bad --release --features score

