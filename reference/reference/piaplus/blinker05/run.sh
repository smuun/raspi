#!/bin/bash
# rm blinker05.elf
# arm-none-eabi-gcc -ggdb -nostartfiles -nostdlib -ffreestanding -mcpu=arm1176jzf-s -Tmemmap \
# blinker05.c vectors.s -o blinker05.elf &&\
make blinker05.elf &&\
tmux new-session \
    "qemu-system-arm -semihosting -s -S -serial stdio -M raspi1ap -kernel blinker05.elf" \; \
    split-window \
    "gdb blinker05.elf -ex \"set arch arm\" -ex \"target remote localhost:1234\""
