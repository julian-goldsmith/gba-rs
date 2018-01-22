LIBC_PATH = ${shell arm-none-eabi-gcc -print-file-name=libc.a}
LIBM_PATH = ${shell arm-none-eabi-gcc -print-file-name=libm.a}
LIBGCC_PATH = ${shell arm-none-eabi-gcc -print-libgcc-file-name}
LIBCORE_PATH = ~/.xargo/lib/rustlib/arm-none-eabi/lib/libcore-69615a36466fc350.rlib	# FIXME
RSFILES = src/lib.rs src/sprite.rs src/gba.rs src/runtime.rs

LIBGBA = target/arm-none-eabi/debug/libgba.rlib

all: cart.gba obj/cart.map

clean:
	rm -rf obj gen cart.gba cart.sav target

cart.gba: obj/cart.elf
	arm-none-eabi-objcopy -Obinary obj/cart.elf cart.gba
	gbafix cart.gba

obj/cart.map: obj/cart.elf
	arm-none-eabi-objdump -t -d obj/cart.elf > obj/cart.map

obj/cart.elf: obj/gba_crt0.o obj/tiles.o ${LIBGBA}
	arm-none-eabi-ld -T gba_cart.ld --gc-sections -o obj/cart.elf obj/gba_crt0.o obj/tiles.o ${LIBGBA} ${LIBCORE_PATH} ${LIBM_PATH} ${LIBC_PATH} ${LIBGCC_PATH}

obj/gba_crt0.o: src/gba_crt0.s | obj
	arm-none-eabi-gcc -c -o obj/gba_crt0.o src/gba_crt0.s

obj/tiles.o: gen/tiles.c | obj
	arm-none-eabi-gcc -c -o obj/tiles.o gen/tiles.c

gen/tiles.c: tiles.png | gen
	grit tiles.png -gB8 -mR8 -mLs -ftch -o gen/tiles.c

${LIBGBA}: build.rs ${RSFILES} gen/tiles.c Cargo.toml
	xargo build

gen:
	mkdir gen

obj:
	mkdir obj
