// stack base for exception modes
.set EXBASE,  0x8000
.set IRQLEN,  256
.set ABTLEN, 256
.set UNDLEN,  256
// svc at bottom

// stack base for program modes
// maybe set it at 0xf424000 = 256 mb?
// for now:
.set PGBASE, 0xf0000
// .set SYSLEN,  0x4000

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
    .word 0 //reserved
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
    /* move the IVT */
    /* since the reset vector and the IVT */
    /* are contiguous in memory we don't need to reload r0*/
    ldr r0, =_ivt
    ldr r4, =_ivt_addrs_end
    b 2f
    1:
        // load r1 with the instruction pointed to by r0
        // i.e., the IVT entry; and increment
        ldr r1, [r0]
        add r0, #4
        // and store it at r2 (same)
        str r1, [r2]
        add r2, #4
    2:
        // did our source pointer get to the end of the IVT?
        cmp r0, r4
        blt 1b

    // done with interrupts
    // initialize stack pointers for all modes
    // sp decrements on push (sp = 10; push; sp = 0c)
    // see arm arch ref man page A3-20
    // from ARM DDI 0100l, page A2-3

    /* ARM processor modes:
     *  |-------------|------|---------------|---------------|
     *  | name        | code | number (0b..) | use in:       |
     *  |-------------|------|---------------|---------------|
     *  | User        | usr  | 10000         | usermode      |
     *  |-------------|------|---------------|---------------|
     *  | FIQ         | fiq  | 10001         | fiq handler   |
     *  |-------------|------|---------------|---------------|
     *  | IRQ         | irq  | 10010         | irq handler   |
     *  |-------------|------|---------------|---------------|
     *  | Supervisor  | svc  | 10011         | OS mode       | ('protected') 
     *  |-------------|------|---------------|---------------|
     *  | Abort       | abt  | 10111         | abrt handlers |
     *  |-------------|------|---------------|---------------|
     *  | Undefined   | und  | 11011         | undef handler |
     *  |-------------|------|---------------|---------------|
     *  | System      | sys  | 10001         | privileged OS |
     *  |-------------|------|---------------|---------------|
     *  
     *  so: we should set up separate stack pointers for every mode
     *  and run in either svc or sys mode.  sys mode is basically usr mode with
     *  no restrictions: unlike svc, it avoids using exception mode registers.
     *  I think we start out in svc mode.
     *  see page a2-7
     *
     *  current psr: cpsr; in all modes
     *  saved psr: spsr, in exception modes
     *  
     *  lower 5 bits of the cpsr are the mode bits
    */


    // usr at bottom

    // TODO following modes not used: usr, fiq

    // EXCEPTION MODES
    mov r0, #EXBASE

    // irq
    msr cpsr_c, #0b10010
    mov sp, r0
    subs r0, #IRQLEN

    // abt
    msr cpsr_c, #0b10111
    mov sp, r0
    subs r0, #ABTLEN

    // undef
    msr cpsr_c, #0b11011
    mov sp, r0
    subs r0, #UNDLEN

    // svc
    msr cpsr_c, #0b10011
    mov sp, r0

    // PROGRAM MODES
    mov r0, #PGBASE

    // sys mode
    msr cpsr_c, #0b11111
    mov sp, r0

    // TODO usr mode

    // call the bootloader in svc mode
    msr cpsr_c, #0b10011

    // enable irq
    cpsie i
    // disable fiq
    cpsid f

    // test out the stack
    b _bootldr

_bootldr:
 
    /* clear out bss */
    ldr r4, =__bss_start
    ldr r9, =__bss_end
    mov r5, #0
    mov r6, #0
    mov r7, #0
    mov r8, #0
    b       2f
 
1:
    // stmia: store multiple at
    // (fill with zeros)
    stmia r4!, {r5-r8}
 

2:
    // below bss_end? clear some more
    cmp r4, r9
    blt 1b

    // call the kernel in sys mode
    msr cpsr_c, #0b11111
    // with irq enabled and fiq disabled
    cpsid f
    cpsie i

    b kernel_main

/* interrupts start out with an unintialized stack */
_undefined_instruction:
    push {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r14}
    bl handle_undefined_instruction
    pop {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r14}
    movs pc, r14

_swi:
    push {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r14}
    bl handle_swi
    pop {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r14}
    movs pc, r14

_prefetch_abrt:
    push {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r14}
    bl handle_prefetch_abrt
    pop {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r14}
    // subs pc, r14, #4
    movs pc, r14

_data_abrt:
    push {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r14}
    bl handle_data_abrt
    pop {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r14}
    // if fixing the reason for the abort
    // return and reexecute
    // subs pc, lr, #8
    movs pc, r14

_irq:
    push {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r14}
    bl handle_irq
    pop {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r14}
    subs pc, r14, #4

_fiq:
    push {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r14}
    bl handle_fiq
    pop {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r14}
    subs pc, r14, #4
