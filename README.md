# Blog OS / WIP

Loosely following https://os.phil-opp.com/, but for arm.
Progress:
- [X] get a freestanding executable running on QEMU
- [ ] VGA text mode: doesn't really exist on the raspberry pi.  
  instead: try and print hello world over UART.
  I guess we need a bootloader: https://github.com/s-matyukevich/raspberry-pi-os/blob/master/docs/lesson01/rpi-os.md
  there is already some sort of bootloader (bootcode.bin).


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
