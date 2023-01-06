.section ".text.ivt.reset"
.globl _reset
.func _reset
_reset:
    b Reset

.section ".ivt.exceptions"
.globl _ivt
_ivt:
    b NMI
    b HardFault
    b MemManage
    b BusFault
    b UsageFault
    b 0
    b 0
    b 0
    b 0
    b SVCall
    b 0
    b 0
    b PendSV
    b SysTick
.section ".text.boot"
.globl _bootldr
.globl Reset

Reset: 
    /* move the reset vector to address 0 */
    ldr r0, =_reset
    mov r2, #0
    ldr r1, [r0, #4]
    str r1, [r2, #4]

    /* move the rest of the IVT */
    ldr r0, =_ivt
    b 2f
    1:
        ldr r1, [r0, #4]
        str r1, [r2, #4]
    2:
        cmp r2, #0x14
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

