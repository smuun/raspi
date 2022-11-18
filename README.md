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

[Good intro](https://thinkingeek.com/2013/01/09/arm-assembler-raspberry-pi-chapter-1/).
