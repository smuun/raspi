#!/bin/bash
set -x
qemu-system-arm -s -S -M raspi1ap -kernel target/arm-none-eabihf/debug/raspi &\
gdb target/arm-none-eabihf/debug/raspi -ex "set arch arm" -ex "target remote localhost:1234"
