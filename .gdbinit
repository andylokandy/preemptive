set confirm off
file ./target/thumbv7m-none-eabi/release/preemptive
target remote :3333
monitor reset halt
load

display /3i $pc

define reset
    monitor reset halt
end

define showasm
end

define pm
    x /20xw
end

define pi
    x /20i
end

define preg
    info all-registers
end
