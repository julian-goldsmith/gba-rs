LIB_C_PATH = ${shell arm-none-eabi-gcc -print-file-name=libc.a}
LIB_GCC_PATH = ${shell arm-none-eabi-gcc -print-libgcc-file-name}

all: cart.gba obj/cart.map

clean:
	rm -f obj/cart.elf obj/*.o obj/cart.map tiles.s cart.gba obj/lib/*.rlib obj/lib/*.a

cart.gba: obj/cart.elf
	arm-none-eabi-objcopy -Obinary obj/cart.elf cart.gba
	gbafix cart.gba

obj/cart.elf: obj/lib/libcore.rlib obj/gba_crt0.o obj/tiles.o obj/main.o
	arm-none-eabi-ld -T gba_cart.ld -o obj/cart.elf obj/gba_crt0.o obj/main.o obj/tiles.o obj/lib/libcore.rlib ${LIB_C_PATH} ${LIB_GCC_PATH}

obj/cart.map: obj/cart.elf
	arm-none-eabi-objdump -t -d obj/cart.elf > obj/cart.map

obj/gba_crt0.o:
	arm-none-eabi-gcc --specs gba.specs -c -o obj/gba_crt0.o src/gba_crt0.s

obj/tiles.o:
	grit tiles.png -gB8 -mR8 -mLs -fts
	arm-none-eabi-gcc --specs gba.specs -c -o obj/tiles.o tiles.s

obj/main.o: obj/lib/libcore.rlib
	rustc -O -Z no-landing-pads --target arm-none-eabi -g --emit obj -L obj/lib -o obj/main.o src/main.rs

obj/lib/libcore.rlib: 
	rustc -O --crate-type=lib -Z no-landing-pads --target arm-none-eabi -g rust/src/libcore/lib.rs --out-dir obj/lib --crate-name core
