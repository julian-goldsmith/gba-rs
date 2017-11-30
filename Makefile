LIBC_PATH = ${shell arm-none-eabi-gcc -print-file-name=libc.a}
LIBGCC_PATH = ${shell arm-none-eabi-gcc -print-libgcc-file-name}
RSFILES = src/lib.rs src/sprite.rs src/gba.rs src/runtime.rs

LIBGBA = target/arm-none-eabi/debug/libgba.rlib

all: cart.gba obj/cart.map

clean:
	rm -f obj/cart.elf obj/*.o obj/cart.map tiles.s cart.gba

clean-lib:
	rm -f obj/lib/*.rlib obj/lib/*.a

cart.gba: obj/cart.elf
	arm-none-eabi-objcopy -Obinary obj/cart.elf cart.gba
	gbafix cart.gba

obj/cart.map: obj/cart.elf
	arm-none-eabi-objdump -t -d obj/cart.elf > obj/cart.map

obj/cart.elf: obj/gba_crt0.o obj/tiles.o ${LIBGBA}
	arm-none-eabi-ld -T gba_cart.ld -o obj/cart.elf obj/gba_crt0.o obj/tiles.o ${LIBGBA} ${LIBC_PATH}

obj/gba_crt0.o: src/gba_crt0.s | obj
	arm-none-eabi-gcc --specs gba.specs -c -o obj/gba_crt0.o src/gba_crt0.s

obj/tiles.o: gen/tiles.c | obj
	arm-none-eabi-gcc --specs gba.specs -c -o obj/tiles.o gen/tiles.c

gen/tiles.c: tiles.png | gen
	grit tiles.png -gB8 -mR8 -mLs -ftch -o gen/tiles.c

${LIBGBA}: ${RSFILES} gen/tiles.c
	xargo build

gen:
	mkdir gen

obj:
	mkdir obj
