# proc_sys_parser

This crate provides routines for parsing linux `/proc` files into Rust structs.

There are multiple other crates doing this, but these either do not choose to process the statistics
in way to make them directly usable, or generalize the statistics and loose the detail.

Currently, only two `/proc` files are processed:

## `/proc/stat`
The processor of `/proc/stat` reads the `CLK_TCK` setting and transforms the jiffies of the cpu times
into milliseconds.

Here is an example using `/proc/stat`:
```rust
use proc_sys_parser::{stat, stat::{ProcStat, CpuStat}};

let proc_stat = stat::read();

assert_eq!(proc_stat,
    ProcStat { cpu_total: CpuStat { name: "cpu".to_string(), user: 1807810, nice: 3570, system: 1993480, idle: 1571076180, iowait: 106510, irq: 0, softirq: 22630, steal: 0, guest: 0, guest_nice: 0, },
               cpu_individual: vec![ CpuStat { name: "cpu0".to_string(), user: 291210, nice: 0, system: 346690, idle: 261693790, iowait: 17710, irq: 0, softirq: 17260, steal: 0, guest: 0, guest_nice: 0, },
                                     CpuStat { name: "cpu1".to_string(), user: 296120, nice: 640, system: 333930, idle: 261871750, iowait: 18610, irq: 0, softirq: 1130, steal: 0, guest: 0, guest_nice: 0, },
                                     CpuStat { name: "cpu2".to_string(), user: 293240, nice: 470, system: 329740, idle: 261885790, iowait: 16920, irq: 0, softirq: 1140, steal: 0, guest: 0, guest_nice: 0, },
                                     CpuStat { name: "cpu3".to_string(), user: 312120, nice: 2450, system: 329940, idle: 261867390, iowait: 18590, irq: 0, softirq: 1210, steal: 0, guest: 0, guest_nice: 0, },
                                     CpuStat { name: "cpu4".to_string(), user: 306320, nice: 0, system: 327210, idle: 261879650, iowait: 17120, irq: 0, softirq: 820, steal: 0, guest: 0, guest_nice: 0, },
                                     CpuStat { name: "cpu5".to_string(), user: 308780, nice: 0, system: 325950, idle: 261877780, iowait: 17540, irq: 0, softirq: 1040, steal: 0, guest: 0, guest_nice: 0, },
                                    ],
               interrupts: vec![ 64226374, 0, 1578053, 20277276, 0, 0, 0, 2, 0, 0, 0, 38423958, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 644, 0, 0, 0, 0, 0, 2, 0, 82584, 85616, 82930, 74304, 74994, 81970, 0, 0, 0, 22407, 22081, 21799, 21485, 22304, 21123, 0, 1186950, 860525, 0, 0, 4295, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1281072, 0, 0, ],
               context_switches: 107129963,
               boot_time: 1701783048,
               processes: 1118791,
               processes_running: 1,
               processes_blocked: 0,
               softirq: vec![ 22446125, 32, 4737167, 213, 2012904, 11, 0, 1450, 7408871, 0, 8285477, ],
             });
```

## `/proc/schedstat`
The processor of `/proc/schedstat` also reads the `CLK_TCK` setting and transforms the jiffies with the
cpu fields, which are fields 7 (time spent running by tasks on this processor) and 8 (time spent waiting
to run by tasks on this processor) to milliseconds.

Here is an example using `/proc/schedstat`:
```rust
use proc_sys_parser::{schedstat, schedstat::ProcSchedStat};

let proc_schedstat = schedstat::read();

assert_eq!(proc_schedstat,
ProcSchedStat { version: 15,
                timestamp: 4318961659,
                cpu: vec![
                   vec![0, 0, 0, 0, 0, 0, 0, 4575719016330, 485940746140, 4348645],
                   vec![1, 0, 0, 0, 0, 0, 0, 4352064330120, 449441457150, 3928368],
                   vec![2, 0, 0, 0, 0, 0, 0, 4296375140810, 435916732570, 3833297],
                   vec![3, 0, 0, 0, 0, 0, 0, 4453083890360, 431027439820, 3851418],
                   vec![4, 0, 0, 0, 0, 0, 0, 4386665545210, 437068452780, 3787400],
                   vec![5, 0, 0, 0, 0, 0, 0, 4447083238720, 428623717880, 3900565]],
                domain: vec![
                   vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                   vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                   vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                   vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                   vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                   vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]
        });
```
### cpu vector
!! Please mind that the vector with cpu statistics takes the cpu number as the first field in the vector.

This means that the fields 7 (running on cpu time) and 8 (waiting on cpu runtime) are fields 8 and 9
in the vector.
### domain vector
!!Please mind that the vector with domain statistics takes the domain number as the first field in the
vector, and the cpumask as the second field.

This means the numbers with the description for the fields
in the kernel documentation <https://www.kernel.org/doc/Documentation/scheduler/sched-stats.txt> has
to be increased by two to get the right statistic number in the vector.

License: Apache-2.0
