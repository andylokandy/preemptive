target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
break unhandled_interrupt
break hard_fault_handler
break rust_begin_unwind

# stop at the user entry point
break main

load

# start the process but immediately halt the processor
stepi
