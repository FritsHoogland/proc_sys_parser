/*!
This crate provides routines for parsing linux `/proc` files into Rust structs.

There are multiple other crates doing this, but these either do not choose to process the statistics
in way to make them directly usable, or generalize the statistics and loose the detail.

# Usage
In order to use this crate with your own repository, either add the `proc_sys_parser` crate to your
`Cargo.toml`, or run `cargo add proc_sys_parser`.

Currently, only two `/proc` files are processed:

# `/proc/stat`
The processor of `/proc/stat` reads the `CLK_TCK` setting and transforms the jiffies of the cpu times
into milliseconds.

Here is an example obtaining the data from `/proc/stat`:
```no_run
use proc_sys_parser::{stat, stat::{ProcStat, CpuStat}};

let proc_stat = stat::read();

println!("{:#?}", proc_stat);
```
Example output:
```text
ProcStat {
    cpu_total: CpuStat { name: "cpu", user: 8570, nice: 0, system: 7530, idle: 1710040, iowait: 2780, irq: 0, softirq: 150, steal: 0, guest: 0, guest_nice: 0 },
    cpu_individual: [CpuStat { name: "cpu0", user: 1800, nice: 0, system: 1450, idle: 283400, iowait: 460, irq: 0, softirq: 120, steal: 0, guest: 0, guest_nice: 0 },
                     CpuStat { name: "cpu1", user: 1720, nice: 0, system: 1320, idle: 284780, iowait: 580, irq: 0, softirq: 0, steal: 0, guest: 0, guest_nice: 0 },
                     CpuStat { name: "cpu2", user: 1060, nice: 0, system: 1220, idle: 285410, iowait: 510, irq: 0, softirq: 0, steal: 0, guest: 0, guest_nice: 0 },
                     CpuStat { name: "cpu3", user: 890, nice: 0, system: 990, idle: 286130, iowait: 450, irq: 0, softirq: 0, steal: 0, guest: 0, guest_nice: 0 },
                     CpuStat { name: "cpu4", user: 1400, nice: 0, system: 1280, idle: 285260, iowait: 310, irq: 0, softirq: 30, steal: 0, guest: 0, guest_nice: 0 },
                     CpuStat { name: "cpu5", user: 1680, nice: 0, system: 1250, idle: 285020, iowait: 450, irq: 0, softirq: 0, steal: 0, guest: 0, guest_nice: 0 }],
    interrupts: [184655, 0, 4500, 60546, 0, 0, 0, 2, 0, 0, 0, 70138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 548, 0, 0, 0, 0, 0, 2, 0, 3410, 2927, 4739, 5542, 1595, 1913, 0, 0, 0, 79, 154, 208, 282, 43, 52, 0, 14842, 11679, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 1437, 0, 0, 0, 0, 0, 0],
    context_switches: 275716,
    boot_time: 1702127060,
    processes: 3472,
    processes_running: 1,
    processes_blocked: 0,
    softirq: [99012, 30, 8368, 2, 24666, 11, 0, 208, 15031, 0, 50696]
}
```
(edited for readability)

# `/proc/schedstat`
The processor of `/proc/schedstat` reads the `CLK_TCK` setting and transforms the jiffies with the
cpu fields, which are fields 7 (time spent running by tasks on this processor) and 8 (time spent waiting
to run by tasks on this processor) to milliseconds. These field numbers are the field numbers of the
statistics in the cpu line of `/proc/schedstat`.

Here is an example obtaining the data from `/proc/schedstat`:
```no_run
use proc_sys_parser::{schedstat, schedstat::ProcSchedStat};

let proc_schedstat = schedstat::read();

println!("{:#?}", proc_schedstat);
```
Example output:
```text
ProcSchedStat {
    version: 15,
    timestamp: 4294964691,
    cpu: [[0, 0, 0, 0, 0, 0, 0, 40178371330, 4778820750, 26299],
          [1, 0, 0, 0, 0, 0, 0, 35526916030, 3606934630, 20919],
          [2, 0, 0, 0, 0, 0, 0, 29224692150, 5614007710, 28163],
          [3, 0, 0, 0, 0, 0, 0, 23848255950, 2265375620, 26240],
          [4, 0, 0, 0, 0, 0, 0, 33846671420, 2990792870, 25605],
          [5, 0, 0, 0, 0, 0, 0, 34565043670, 2885580430, 22629]],
    domain: [[0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
             [0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
             [0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
             [0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
             [0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
             [0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]
}
```
(edited for readability)

## cpu vector
!! Please mind that the vector with cpu statistics takes the cpu number as the first field in the vector.

This means that the fields 7 (running on cpu time) and 8 (waiting on cpu runtime) are fields 8 and 9
in the vector.

## domain vector
!!Please mind that the vector with domain statistics takes the domain number as the first field in the
vector, and the cpumask as the second field.

This means the numbers with the description for the fields
in the kernel documentation <https://www.kernel.org/doc/Documentation/scheduler/sched-stats.txt> has
to be increased by two to get the right statistic number in the vector.

# `/proc/meminfo`
The processor of `/proc/meminfo` reads the values for the memory areas specified in the file.
The values are in kilobytes (kB), just like the values in the original `/proc/meminfo`
file.

Here is an example obtaining the data from `/proc/meminfo`:
```no_run
use proc_sys_parser::{meminfo, meminfo::ProcMemInfo};

let proc_meminfo = meminfo::read();

println!("{:#?}", proc_meminfo);
```
Example output:
```text
ProcMemInfo {
    memtotal: 3997876,
    memfree: 2415136,
    memavailable: 3654096,
    buffers: 37492,
    cached: 1305568,
    swapcached: 0,
    active: 880772,
    inactive: 549432,
    active_anon: 86968,
    inactive_anon: 5196,
    active_file: 793804,
    inactive_file: 544236,
    unevictable: 4000,
    mlocked: 0,
    swaptotal: 0,
    swapfree: 0,
    zswap: 0,
    zswapped: 0,
    dirty: 0,
    writeback: 0,
    anonpages: 91144,
    mapped: 140948,
    shmem: 5020,
    kreclaimable: 56680,
    slab: 93916,
    sreclaimable: 56680,
    sunreclaim: 37236,
    kernelstack: 3256,
    shadowcallstack: 828,
    pagetables: 2884,
    secpagetables: 0,
    nfs_unstable: 0,
    bounce: 0,
    writebacktmp: 0,
    commitlimit: 1998936,
    committed_as: 944240,
    vmalloctotal: 133141626880,
    vmallocused: 14124,
    vmallocchunk: 0,
    percpu: 2280,
    hardwarecorrupted: 0,
    anonhugepages: 4096,
    shmemhugepages: 0,
    shmempmdmapped: 0,
    filehugepages: 0,
    filepmdmapped: 0,
    cmatotal: 32768,
    cmafree: 31232,
    hugepages_total: 0,
    hugepages_free: 0,
    hugepages_rsvd: 0,
    hugepages_surp: 0,
    hugepagesize: 2048,
    hugetlb: 0
}
```
(edited for readability)
 */
pub mod stat;
pub mod schedstat;
pub mod meminfo;

