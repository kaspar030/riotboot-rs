#openocd -c 'source [find interface/cmsis-dap.cfg]' \
#    -f '/home/kaspar/src/riot/boards/common/nrf52/dist/openocd.cfg' \
#    -c 'init' -c 'targets' -c 'reset halt' -c "flash write_image erase \"$0\" 0 " -c "verify_image \"$0\" 0" -c 'reset run' -c 'shutdown'
#
set -x
openocd -f interface/cmsis-dap.cfg -f "target/nrf52.cfg" -c 'init' -c 'targets' -c 'reset halt'
