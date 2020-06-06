# src-brandfarlig-ng
Software consists of three main units running on a Xilinx Zynq 7Z010 FPGA with a Cortex-A9 processor 

## TACTical - `tactical/`
Runs tactical and command logic

* Platform: linux

## RECOrder - `recorder/`
Records session to a database for replay

* Platform: linux

## NAVIgation - `navigation/`
Handles collection and processing of sensor data for navigation 

* Platform: bare-metal / linux(sim)