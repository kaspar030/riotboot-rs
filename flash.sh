#!/bin/sh

openocd -f interface/cmsis-dap.cfg -f "target/nrf52.cfg" -c 'init' -c 'targets' -c 'reset halt' -c "flash write_image erase \"$1\" 0 elf" -c "verify_image \"$1\" 0 elf" -c 'reset run' -c     'shutdown'
