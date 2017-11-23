all:
	arm-none-eabi-gcc --specs gba.specs -c -o obj/gba_crt0.o src/gba_crt0.s
	rustc -O -Z no-landing-pads --target arm-none-eabi -g --emit obj -L libcore-arm -o obj/main.o src/main.rs
	arm-none-eabi-ld -T gba_cart.ld -o obj/cart.elf obj/gba_crt0.o obj/main.o libcore-arm/libcore.rlib
	arm-none-eabi-objcopy -Obinary obj/cart.elf cart.gba
	gbafix cart.gba

core:
	rustc -O --crate-type=lib -Z no-landing-pads --target arm-none-eabi -g rust/src/libcore/lib.rs --out-dir libcore-arm --crate-name core
