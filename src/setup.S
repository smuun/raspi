.code 32
.section ".text.ivt.reset"
.balign 4
.globl _reset
_reset:
    b Reset

.section ".text.ivt.exceptions"
.balign 4
.globl _ivt
.globl _ivt_end
_ivt:
    ldr pc, =NMI
    ldr pc, =HardFault
    ldr pc, =MemManage
    ldr pc, =BusFault
    ldr pc, =UsageFault
    .word 0
    .word 0
    .word 0
    .word 0
    ldr pc, =SVCall
    ldr pc, =DebugMonitor
    .word 0
    ldr pc, =PendSV
_ivt_end:
    ldr pc, =SysTick

.section ".text.boot"
.balign 4
.globl _bootldr

.globl Reset

Reset: 
    /* load r0 with the address of reset */
    ldr r0, =_reset
    mov r2, #0
    /* load r1 with the instruction at r0 */
    /* which should be, "b HardFault or whatever" */
    ldr r1, [r0]
    /* increment r0 by 4 bytes = 32 bits = 1 word */
    /* obviously there is a built-in way to do this */
    /* but this is very clear */
    add r0, #4
    str r1, [r2]
    add r2, #4

    /* move the rest of the IVT */
    /* since the reset vector and the IVT */
    /* are contiguous in memory we don't need to reload r0*/
    @ ldr r0, =_ivt
    ldr r4, =_ivt_end
    b 2f
    1:
        ldr r1, [r0]
        add r0, #4
        str r1, [r2]
        add r2, #4
    2:
    /* loop if not last entry in IVT */
        cmp r0, r4
        blt 1b
    b _bootldr

_bootldr:
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
    stmia r4!, {r5-r8}
 
    // If we are still below bss_end, loop.
2:
    cmp r4, r9
    blo 1b
 
    // Call kernel_main
    b kernel_main

