# Preemptive-rs

A minimum preemptive OS on Cortex-M3 (specially on blue-pill board) written in Rust. It is for the purpose of researching and showing how the fundamental runtime of Cortex-M3 works.

## What is preemptive OS

> In computing, preemption is the act of temporarily interrupting a task being carried out by a computer system, without requiring its cooperation, and with the intention of resuming the task at a later time. Such changes of the executed task are known as context switches.   ----  Wikipedia

In breif, the kernel of non-preemptive OS can not interrupt a task, while the kernel of preemptive OS can take the control back without informing the task.

## Build and Run

- Make sure you have `arm-none-eabi` toolchain and `openocd` installed on your platform.

- Connect pin `PA2` to a serial reciever, with 115200 baudrate, 8 data bits, 1 stop bits, no parity and no flow control.

- Compile the application:
```
> cargo build --release
    Finished release [optimized + debuginfo] target(s) in 0.49s
```

- Run `openocd`:

```
> openocd
...
Info : using stlink api v2
Info : Target voltage: 3.175214
Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

- Run `arm-none-eabi-gdb` and continue:

```
> arm-none-eabi-gdb -q
...
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x080008fc msp: 0x20005000
Loading section .isr_vector, size 0x40 lma 0x8000000
Loading section .text, size 0x2a08 lma 0x8000040
Start address 0x80008fc, load size 10824
Transfer rate: 14 KB/sec, 5412 bytes/write.
(gdb) continue
Continuing.
```

As the result, you would see the output from the serial reciever like this:

```
Kernel started!
Executing task1!
Executing task2!
Executing task1!
Executing task2!
...
```

## Reference

- mini-arm-os (https://github.com/jserv/mini-arm-os)
- tock os (https://github.com/tock/tock)