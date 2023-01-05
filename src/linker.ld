/* ENTRY(_reset) */
EXTERN(RESET_VECTOR)
EXTERN(EXCEPTIONS)
 
SECTIONS
{
    /* Initial stack pointer value; GPU starts execution here.*/
    . = 0x8000;
    __reset = .;
    .vector_table : 
    {
        /* Start execution after reboot or reset */
        *(.ivt.reset)
        /* Define 14 exception vectors */
        *(.ivt.exceptions)
    }
    __text_start = .;
    .text :
    {
        *(.text.boot)
        *(.text)
    }
    . = ALIGN(4096); /* align to page size */
    __text_end = .;
 
    __rodata_start = .;
    .rodata :
    {
        *(.rodata)
    }
    . = ALIGN(4096); /* align to page size */
    __rodata_end = .;
 
    __data_start = .;
    .data :
    {
        *(.data)
    }
    . = ALIGN(4096); /* align to page size */
    __data_end = .;
 
    __bss_start = .;
    .bss : {
        bss = .;
        *(.bss)
    }
    /* at some point, we might need to FILL() the intervening sections.  See
     * https://github.com/RusPiRo/ruspiro-boot/blob/v0.5.4/link64.ld */

    . = ALIGN(4096); /* align to page size */ 
    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
    __end = .;
}

PROVIDE(NMI = DefaultExceptionHandler);
PROVIDE(HardFault = DefaultExceptionHandler);
PROVIDE(MemManage = DefaultExceptionHandler);
PROVIDE(BusFault = DefaultExceptionHandler);
PROVIDE(UsageFault = DefaultExceptionHandler);
PROVIDE(SVCall = DefaultExceptionHandler);
PROVIDE(PendSV = DefaultExceptionHandler);
PROVIDE(SysTick = DefaultExceptionHandler);