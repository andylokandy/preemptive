# Preemptive-rs

A minimum preemptive OS on Cortex-M3 (specially on blue-pill board) written in Rust. It is for the purpose of researching and showing how the fundamental runtime of Cortex-M3 works.

## What is preemptive OS

> In computing, preemption is the act of temporarily interrupting a task being carried out by a computer system, without requiring its cooperation, and with the intention of resuming the task at a later time. Such changes of the executed task are known as context switches.   ----  Wikipedia

In breif, the kernel of non-preemptive OS can not interrupt a task, while the kernel of preemptive OS can take the control back without informing the task.

## Prerequisite

- Make sure you have a `blue-pill` board and a serial port reciever.
- Make sure you have `arm-none-eabi` toolchain and `openocd` installed on your platform.
- Install the latest nightly rust toolchain. The compiler version used when this project is beening written is `rustc 1.37.0-nightly (17e62f77f 2019-07-01)`.

## Project Structure

This project is collections of several stages of building a preliminary preemptive OS from sketch. The `Final` folder contains the final version we build up step by step. I'll make sure every code in each chapter can be compiled and run on `blue-pill`.

## Build and Run

- Enter `Final`

```
cd Final
```

- Connect the `blue-pill` to your laptop.

- Connect pin `PA2` to a serial reciever, with 115200 baudrate, 8 data bits, 1 stop bits, no parity and no flow control.

- Run `openocd`:

```text
> openocd
...
Info : using stlink api v2
Info : Target voltage: 3.175214
Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

- Run the application:

```text
> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.93s
    Running `target\thumbv7m-none-eabi\debug\preemptive`
Reading symbols from target\thumbv7m-none-eabi\debug\preemptive...done.
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

```text
Executing task1!
task1: fib(0)=1
task1: fib(1)=1
task1: fib(2)=2

Executing task2!
task2: is_prime(1)=true
task2: is_prime(2)=true
task2: is_prime(3)=true

Executing task1!
task1: fib(3)=3
task1: fib(4)=5
task1: fib(5)=8

Executing task2!
task2: is_prime(4)=false
task2: is_prime(5)=true
task2: is_prime(6)=false
...
```

## Reference

- mini-arm-os (https://github.com/jserv/mini-arm-os)
- tock os (https://github.com/tock/tock)