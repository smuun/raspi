# Blog OS / WIP

Loosely following https://os.phil-opp.com/.  Adapted for the armv6 CPU in the
raspberry pi 1 model B+.

## Done:
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
- [X] Testing.
- [X] Refactor the uart code for singleton example
- [X] Use `readc!` instead of getc

## Currently working on
- [ ] Exceptions

## TODO (issues)
- [ ] Bug: if you mash tons of keys the input code gets stuck in a spinlock

# Rust target
- [Rust users](https://users.rust-lang.org/t/how-to-compile-freestanding-binary-for-armv6/50980/7)
- [OSDev](https://wiki.osdev.org/Raspberry_Pi_Bare_Bones_Rust).

See `arm-none-eabihf.json`.

# Debugging
Using [QEMU](https://www.qemu.org/docs/master/system/arm/raspi.html).  See `.cargo/config.toml` for the command, and `./debug.sh` for the debug setup.

# ARM assembler for the raspberry pi 
Resources: 
- [Intro tutorial](https://thinkingeek.com/2013/01/09/arm-assembler-raspberry-pi-chapter-1/).
- [Whirlwind tour](https://www.coranac.com/tonc/text/asm.htm).

# Notes
[RPi boot sequence \(kind of interesting\)](https://raspberrypi.stackexchange.com/questions/10442/what-is-the-boot-sequence/10595#10595).

Some guy doing MINIX on rpi 3:
[here](https://forums.raspberrypi.com/viewtopic.php?t=291366).
