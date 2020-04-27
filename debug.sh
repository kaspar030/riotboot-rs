#!/bin/sh

openocd -f interface/cmsis-dap.cfg -f "target/nrf52.cfg" -c 'init' -c 'targets' -c 'reset halt'
