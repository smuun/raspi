#!/bin/bash
set -x
tmux new-session \
    "qemu-system-arm -s -S -serial stdio -M raspi1ap -kernel target/arm-none-eabihf/debug/raspi" \; \
    split-window \
    "gdb target/arm-none-eabihf/debug/raspi -ex \"set arch arm\" -ex \"target remote localhost:1234\""
