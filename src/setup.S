.code 32
.balign 4

.section ".text.ivt"
.globl _ivt
.globl _ivt_end

/* https://github.com/BrianSidebotham/arm-tutorial-rpi/blob/master/part-4/readme.md */
/* I was INCORRECTLY using armv6-m; it should have been pure armv6 */
_ivt:
    ldr pc, addr_Reset
    ldr pc, addr_undefined_instruction
    ldr pc, addr_swi
    ldr pc, addr_prefetch_abrt
    ldr pc, addr_data_abrt
    .word 0 @reserved
    ldr pc, addr_irq
    ldr pc, addr_fiq
_ivt_end:
    .word 0xdeadbeef
_ivt_addrs:
    addr_Reset:                 .word _reset
    addr_undefined_instruction: .word _undefined_instruction
    addr_swi:                   .word _swi
    addr_prefetch_abrt:         .word _prefetch_abrt
    addr_data_abrt:             .word _data_abrt
    addr_irq:                   .word _irq
    addr_fiq:                   .word _fiq
_ivt_addrs_end:
    .word 0xdeadbeef

.section ".text.boot"
.balign 4
.globl _bootldr
.globl _reset

_reset: 
    /* enable irq and fiq */
    cpsie i
    cpsie f

    /* move the IVT */
    /* since the reset vector and the IVT */
    /* are contiguous in memory we don't need to reload r0*/
    ldr r0, =_ivt
    ldr r4, =_ivt_addrs_end
    b 2f
    1:
        @ load r1 with the instruction pointed to by r0
        @ i.e., the IVT entry; and increment
        ldr r1, [r0]
        add r0, #4
        @ and store it at r2 (same)
        str r1, [r2]
        add r2, #4
    2:
        @ did our source pointer get to the end of the IVT?
        cmp r0, r4
        blt 1b

    @ done with interrupts
    @ continue on to prepare stack, etc
    b _bootldr

_bootldr:
    @ sp grows downwards from code start
    mov sp, #0x8000
 
    /* clear out bss */
    ldr r4, =__bss_start
    ldr r9, =__bss_end
    mov r5, #0
    mov r6, #0
    mov r7, #0
    mov r8, #0
    b       2f
 
1:
    @ stmia: store multiple at
    @ (fill with zeros)
    stmia r4!, {r5-r8}
 

2:
    @ below bss_end? clear some more
    cmp r4, r9
    blo 1b

    b kernel_main

/* interrupts start out with an unintialized stack */

_undefined_instruction:
    mov sp, #0x6000
    bl handle_undefined_instruction

_swi:
    mov sp, #0x6000
    bl handle_swi

_prefetch_abrt:
    mov sp, #0x6000
    bl handle_prefetch_abrt

_data_abrt:
    mov sp, #0x6000
    bl handle_data_abrt

_irq:
    mov sp, #0x6000
    bl handle_irq

_fiq:
    mov sp, #0x6000
    bl handle_fiq