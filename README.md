# 负载均衡 Bug

1. 在 pick_next_task 时，先尝试从局部队列中取出任务，若没有，则从任务负载最重的队列中取出任务
2. 在 schedule 和 wakeup_task 函数中使用全局大锁

## 现象

1. 提示页错误

```shell

       d8888                            .d88888b.   .d8888b.
      d88888                           d88P" "Y88b d88P  Y88b
     d88P888                           888     888 Y88b.
    d88P 888 888d888  .d8888b  .d88b.  888     888  "Y888b.
   d88P  888 888P"   d88P"    d8P  Y8b 888     888     "Y88b.
  d88P   888 888     888      88888888 888     888       "888
 d8888888888 888     Y88b.    Y8b.     Y88b. .d88P Y88b  d88P
d88P     888 888      "Y8888P  "Y8888   "Y88888P"   "Y8888P"

arch = riscv64
platform = riscv64-qemu-virt
target = riscv64gc-unknown-none-elf
smp = 4
build_mode = release
log_level = error

Running testcase: libctest_testcode.sh
========== START entry-static.exe argv ==========
Pass!
========== END entry-static.exe argv ==========
[  0.542911 0:14 axruntime::lang_items:5] panicked at /home/zfl/.cargo/git/checkouts/axtrap-fb36713fee8cc9bf/f2529e6/src/arch/riscv/mod.rs:100:17:
not implemented: S page fault from kernel, addr: 3FFFF6DC, sepc: FFFFFFC08024EBA6
[  0.544401 0:14 axbacktrace:43] Call trace: 
[  0.544662 0:14 axbacktrace::riscv:89] 0xFFFFFFC08025FE8C 
[  0.544946 0:14 axbacktrace::riscv:89] 0xFFFFFFC0802574A8 
[  0.545167 0:14 axbacktrace::riscv:89] 0xFFFFFFC08026185E 
[  0.545389 0:14 axbacktrace::riscv:89] 0xFFFFFFC0802600CE 
[  0.545621 0:14 axbacktrace::riscv:89] 0xFFFFFFC08020206C 
[  0.545838 0:14 axbacktrace::riscv:89] 0xFFFFFFC080270028 
[  0.546058 0:14 axbacktrace::riscv:89] 0xFFFFFFC080276274 
[  0.546276 0:14 axbacktrace::riscv:89] 0xFFFFFFC080260310 
[  0.546496 0:14 axbacktrace::riscv:89] 0xFFFFFFC080202124
```

2. 没有显示，卡死，需要手动结束 qemu

```shell

       d8888                            .d88888b.   .d8888b.
      d88888                           d88P" "Y88b d88P  Y88b
     d88P888                           888     888 Y88b.
    d88P 888 888d888  .d8888b  .d88b.  888     888  "Y888b.
   d88P  888 888P"   d88P"    d8P  Y8b 888     888     "Y88b.
  d88P   888 888     888      88888888 888     888       "888
 d8888888888 888     Y88b.    Y8b.     Y88b. .d88P Y88b  d88P
d88P     888 888      "Y8888P  "Y8888   "Y88888P"   "Y8888P"

arch = riscv64
platform = riscv64-qemu-virt
target = riscv64gc-unknown-none-elf
smp = 4
build_mode = release
log_level = error

Running testcase: libctest_testcode.sh
========== START entry-static.exe argv ==========
Pass!
========== END entry-static.exe argv ==========
......
========== START entry-static.exe fflush_exit ==========

```

qemu 的日志显示出现页错误

```shell
riscv_cpu_do_interrupt: hart:0, async:0, cause:000000000000000d, epc:0xffffffc08025748c, tval:0x0000000000000073, desc=load_page_fault
riscv_cpu_tlb_fill ad 73 rw 0 mmu_idx 2
riscv_cpu_tlb_fill address=73 ret 1 physical 0000000000000000 prot 0
riscv_cpu_do_interrupt: hart:0, async:0, cause:000000000000000d, epc:0xffffffc08025748c, tval:0x0000000000000073, desc=load_page_fault
riscv_cpu_do_interrupt: hart:1, async:0, cause:000000000000000f, epc:0xffffffc0802020be, tval:0xffffffc084abea48, desc=store_page_fault
```