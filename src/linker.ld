ENTRY(_reset);
EXTERN(_reset);
EXTERN(_ivt);
 
SECTIONS
{
    /* Initial stack pointer value; GPU starts execution here.*/
    . = 0x8000;
    __text_start = .;
    .text :
    {
        /* Start executing directly in Reset */
        *(.text.ivt)
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

PROVIDE(handle_undefined_instruction = handle_default);
PROVIDE(handle_swi = handle_default);
PROVIDE(handle_prefetch_abrt = handle_default);
PROVIDE(handle_data_abrt = handle_default);
PROVIDE(handle_irq = handle_default);
PROVIDE(handle_fiq = handle_default);