use core::arch::global_asm;

//TODO write this in Rust!  Just for practice.
global_asm!(
    r#"
// AArch32 mode
 
// To keep this in the first portion of the binary.
.section ".text.boot"

.globl _start
.globl _qemu_halt_normal
.globl _qemu_halt_fail

// OBVIOUSLY THE IVT NEEDS TO BE HERE. DUMBASS.
    .org 0x8000
// Entry point for the kernel.
// r15 -> should begin execution at 0x8000.
// r0 -> 0x00000000
// r1 -> 0x00000C42 - machine id
// r2 -> 0x00000100 - start of ATAGS
// preserve these registers as argument for kernel_main

_start:
    // Setup the stack.
    mov sp, #0x8000
 
    // Clear out bss.
    ldr r4, =__bss_start
    ldr r9, =__bss_end
    mov r5, #0
    mov r6, #0
    mov r7, #0
    mov r8, #0
    b       2f
 
1:
    // store multiple at r4.
    stmia r4!, {{r5-r8}}
 
    // If we are still below bss_end, loop.
2:
    cmp r4, r9
    blo 1b
 
    // Call kernel_main
    ldr r3, =kernel_main
    blx r3
 
    // halt
halt:
    b _qemu_halt_normal
    b halt

_qemu_halt_normal:
    mov r1, #0x20000
    add r1, r1, #0x00026
    b qemu_halt

_qemu_halt_fail:
    mov r1, #0x20000
    add r1, r1, #0x00027
    b qemu_halt

qemu_halt:
    //call angel interrupt 
    //at svc 123456 with
    //#0x18 in r0 and
    //exit code in r1.
    //normal exit is 0x20026
    mov r0, #0x18
    svc 0x00123456
"#
);
