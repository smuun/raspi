# BlogOS

Loosely following https://os.phil-opp.com/.  Adapted for the armv6 CPU in the
raspberry pi 1 model B+.

## Done:
- [X] Freestanding executable running on QEMU.  See cargo config and target json
  for details (cross-compilation, custom linker.ld, etc).  Pieced together from
  the internet and the arm reference manual.
- [X] Bootloader.  See `setup.S` -- no blog-os bootloader crate for ARM, plus it
  was kind of interesting to have to figure it out.
- [X] Basic input and output over the raspi1's UART (configure the UART and read
  and write characters). This is instead of VGA text mode, which doesn't exist on
  ARM (I think).
- [X] Testing.
- [X] Refactor the uart code for singleton example
- [X] Use `readc!` instead of getc
- [X] Exceptions


## Currently working on
- [ ] ?

## TODO (issues)
- [ ] Bug: if you mash tons of keys the input code gets stuck in a spinlock
- [ ] Timer. Two known working examples: `reference/arm-tutorial-rpi/part-4/armc-013`
    and `reference/piaplus/blinker05`.  My implementation from the datasheet was
    basically identical but didn't trigger interrupts on QEMU.  Spent forever
    browsing source files for the two references and then finally tested them on
    qemu -- turns out they don't work either (tried changing their interrupt
    handlers to loop (and check with gdb), didn't work; svc exit qemu, didn't work).
    Next steps: do the LED blink and try on real rpi.

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
The ARMv6 processor in the pi B+ is an **A-profile processor**.  There are tons of
references online to it being M-profile, but it is not. This is important because:
there is no systick exception, the IVT stuff is significantly different, and so are the execution modes.

[RPi boot sequence \(kind of interesting\)](https://raspberrypi.stackexchange.com/questions/10442/what-is-the-boot-sequence/10595#10595).

Some guy doing MINIX on rpi 3:
[here](https://forums.raspberrypi.com/viewtopic.php?t=291366).
