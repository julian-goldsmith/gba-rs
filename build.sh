echo "crt0"
arm-none-eabi-gcc --specs gba.specs -c -o obj/gba_crt0.o src/gba_crt0.s

echo "Main"
rustc -O -Z no-landing-pads --target arm-none-eabi -g --emit obj -L libcore-arm -o obj/main.o src/main.rs

echo "Link"
arm-none-eabi-ld -T gba_cart.ld -o a.out obj/gba_crt0.o obj/main.o libcore-arm/libcore.rlib
