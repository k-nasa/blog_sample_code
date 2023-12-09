OUTPUT_ARCH( "riscv" )
ENTRY( _entry )

SECTIONS
{
  . = 0x80200000;

  .text : {
    *(.entry)
    *(.text .text.*)
  }

  .rodata : {
    *(.rdata .rodata .rodata.*)
  }

  .data : {
    *(.data .data.*)
  }

  .bss : {
    *(.bss bss.*)
  }
}
