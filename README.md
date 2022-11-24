# Blog OS / WIP

Loosely following https://os.phil-opp.com/.  But this is on arm, and I am also
writing (some of) my own code.

Progress:
- [X] Freestanding executable running on QEMU.  See cargo config and target json
  for details (cross-compilation, custom linker.ld, etc).  Pieced together from
  the internet and the arm reference manual.
- [X] Bootloader.  The bootloader crate doesn't work on arm, so instead of that
  I'm using a (slightly adapted) bootloader example from the osdev reference.
  It basically just defines a stack, zeroes out bss, and hands off execution to
  kernel_main.
- [X] Basic input and output over the raspi1's UART (configure the UART and read
  and write characters). This is instead of VGA text mode, which doesn't exist on
  ARM (I think).
- [ ] Working testing in Rust.
  currently stuck on the following: there's a bug where it won't compile with
  panic=abort, but I can't figure out how to define the normal panic stuff


# QEMU

[QEMU](https://www.qemu.org/docs/master/system/arm/raspi.html).
We want target `raspi1ap`.  Not implemented in QEMU: SPI, ADC, PWM; don't try to
use these.

Command: my best guess is `qemu-system-arm -machine raspi1ap -kernel target/arm-none-eabihf/debug/raspi`.

# Rust

No built-in target triple.  Resources:

- [Rust users](https://users.rust-lang.org/t/how-to-compile-freestanding-binary-for-armv6/50980/7)
- [OSDev](https://wiki.osdev.org/Raspberry_Pi_Bare_Bones_Rust).

# ARM assembler for the raspberry pi 

Resources: 
- [Intro tutorial](https://thinkingeek.com/2013/01/09/arm-assembler-raspberry-pi-chapter-1/).
- [Whirlwind tour](https://www.coranac.com/tonc/text/asm.htm).

# Notes
[RPi boot sequence](https://raspberrypi.stackexchange.com/questions/10442/what-is-the-boot-sequence/10595#10595).

Some guy doing MINIX on rpi 3:
[here](https://forums.raspberrypi.com/viewtopic.php?t=291366).
