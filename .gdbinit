set confirm off
file ./target/thumbv7m-none-eabi/release/preemptive
target remote :3333
monitor reset halt
load

display /3i $pc

define reset
    monitor reset halt
end
