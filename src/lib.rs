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
    softirq: [99012, 30, 8368, 2, 24666, 11, 0, 208, 15031, 0, 50696],
}
```
(edited for readability)

# `/proc/schedstat`
The statistics in `/proc/schedstat`, which are the cpu line scheduler running and waiting statistics in fields 7 and 8
are shown in jiffies up to kernel 2.6.23/commit 425e0968a25, and in nanoseconds after that.

That means that most current systems should be using a kernel version that shows this time in nanoseconds.

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
             [0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]],
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
    hugetlb: 0,
}
```
(edited for readability)

# `/proc/diskstats`
The processor of `/proc/diskstats` reads the statistics for the block devices. The amount of data is
in sectors, which are documented as hard coded to 512 bytes per sector in the linux kernel.

Here is an example obtaining the disk statistics from `/proc/diskstats`:
```no_run
use proc_sys_parser::{diskstats, diskstats::ProcDiskStats};

let proc_diskstats = diskstats::read();

println!("{:#?}", proc_diskstats);
```
Example output:
```text
ProcDiskStats {
    disk_stats: [
            DiskStats { block_major: 7, block_minor: 0, device_name: "loop0", reads_completed_success: 11, reads_merged: 0, reads_sectors: 28, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 4, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
            DiskStats { block_major: 7, block_minor: 1, device_name: "loop1", reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
            DiskStats { block_major: 7, block_minor: 2, device_name: "loop2", reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
            DiskStats { block_major: 7, block_minor: 3, device_name: "loop3", reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
            DiskStats { block_major: 7, block_minor: 4, device_name: "loop4", reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
            DiskStats { block_major: 7, block_minor: 5, device_name: "loop5", reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
            DiskStats { block_major: 7, block_minor: 6, device_name: "loop6", reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
            DiskStats { block_major: 7, block_minor: 7, device_name: "loop7", reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
            DiskStats { block_major: 253, block_minor: 0, device_name: "vda", reads_completed_success: 13534, reads_merged: 4237, reads_sectors: 1645451, reads_time_spent_ms: 3763, writes_completed_success: 10172, writes_merged: 10577, writes_sectors: 1730555, writes_time_spent_ms: 12701, ios_in_progress: 0, ios_time_spent_ms: 23356, ios_weighted_time_spent_ms: 18881, discards_completed_success: 7179, discards_merged: 0, discards_sectors: 89620507, discards_time_spent_ms: 396, flush_requests_completed_success: 3929, flush_requests_time_spent_ms: 2019 },
            DiskStats { block_major: 253, block_minor: 1, device_name: "vda1", reads_completed_success: 13192, reads_merged: 2675, reads_sectors: 1623109, reads_time_spent_ms: 3692, writes_completed_success: 10151, writes_merged: 10555, writes_sectors: 1730312, writes_time_spent_ms: 12688, ios_in_progress: 0, ios_time_spent_ms: 23324, ios_weighted_time_spent_ms: 16775, discards_completed_success: 7151, discards_merged: 0, discards_sectors: 87803128, discards_time_spent_ms: 394, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
            DiskStats { block_major: 253, block_minor: 15, device_name: "vda15", reads_completed_success: 136, reads_merged: 1547, reads_sectors: 9919, reads_time_spent_ms: 20, writes_completed_success: 1, writes_merged: 0, writes_sectors: 1, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 52, ios_weighted_time_spent_ms: 21, discards_completed_success: 1, discards_merged: 0, discards_sectors: 186691, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
            DiskStats { block_major: 259, block_minor: 0, device_name: "vda16", reads_completed_success: 159, reads_merged: 15, reads_sectors: 10711, reads_time_spent_ms: 31, writes_completed_success: 20, writes_merged: 22, writes_sectors: 242, writes_time_spent_ms: 12, ios_in_progress: 0, ios_time_spent_ms: 108, ios_weighted_time_spent_ms: 46, discards_completed_success: 27, discards_merged: 0, discards_sectors: 1630688, discards_time_spent_ms: 1, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
            DiskStats { block_major: 11, block_minor: 0, device_name: "sr0", reads_completed_success: 291, reads_merged: 0, reads_sectors: 75108, reads_time_spent_ms: 68, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 156, ios_weighted_time_spent_ms: 68, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
    ]
}
```
(edited for readability)

# `/proc/net/dev`
The processor of `/proc/net/dev` reads the statistics for the network devices.

The documentation for `/proc/net/dev` is found here: <https://www.kernel.org/doc/Documentation/filesystems/proc.txt>

Here is an example obtaining the data from `/proc/net/dev`:
```no_run
use proc_sys_parser::{net_dev, net_dev::ProcNetDev};

let proc_net_dev = net_dev::read();

println!("{:#?}", proc_net_dev);
```
Example output:
```text
ProcNetDev {
    interface: [
        InterfaceStats { name: "lo".to_string(), receive_bytes: 0, receive_packets: 0, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 0, transmit_packets: 0, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 },
        InterfaceStats { name: "eth0".to_string(), receive_bytes: 151013652, receive_packets: 16736, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 816228, transmit_packets: 12257, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 }
    ]
}
```
(edited for readability)

# `/sys/block/<device>`
The processor of `/sys/block` reads the block device directories, and parses the statistics in it.

The documentation for `/sys/block` is found here: <https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-block>

Here is an example obtaining the data from `/sys/block`:
```no_run
use proc_sys_parser::block;

let block = block::read();

println!("{:#?}", block);
```
Example output:
```text
SysBlock {
    block_devices: [
        BlockDevice {
            dev_block_major: 253,
            dev_block_minor: 0,
            device_name: "sda",
            discard_alignment: 0,
            stat_reads_completed_success: 9718,
            stat_reads_merged: 3826,
            stat_reads_sectors: 1052371,
            stat_reads_time_spent_ms: 3026,
            stat_writes_completed_success: 2856,
            stat_writes_merged: 2331,
            stat_writes_sectors: 312397,
            stat_writes_time_spent_ms: 1947,
            stat_ios_in_progress: 0,
            stat_ios_time_spent_ms: 6004,
            stat_ios_weighted_time_spent_ms: 5554,
            stat_discards_completed_success: Some(
                7141,
            ),
            stat_discards_merged: Some(
                0,
            ),
            stat_discards_sectors: Some(
                88014755,
            ),
            stat_discards_time_spent_ms: Some(
                276,
            ),
            stat_flush_requests_completed_success: Some(
                591,
            ),
            stat_flush_requests_time_spent_ms: Some(
                304,
            ),
            alignment_offset: 0,
            cache_type: "write back",
            diskseq: 9,
            hidden: 0,
            inflight_reads: 1,
            inflight_writes: 2,
            range: 16,
            removable: 0,
            ro: 0,
            size: 125829120,
            queue_max_hw_sectors_kb: 2147483647,
            queue_max_sectors_kb: 1280,
            queue_max_discard_segments: 1,
            queue_nr_requests: 256,
            queue_nr_zones: Some(
                0,
            ),
            queue_scheduler: "none",
            queue_rotational: 1,
            queue_dax: 0,
            queue_add_random: 0,
            queue_discard_granularity: 512,
            queue_discard_max_hw_bytes: 2147483136,
            queue_discard_max_bytes: 2147483136,
            queue_hw_sector_size: 512,
            queue_io_poll: 0,
            queue_io_poll_delay: -1,
            queue_logical_block_size: 512,
            queue_minimum_io_size: 512,
            queue_max_integrity_segments: 0,
            queue_max_segments: 254,
            queue_max_segment_size: 4294967295,
            queue_nomerges: 0,
            queue_physical_block_size: 512,
            queue_optimal_io_size: 0,
            queue_read_ahead_kb: 128,
            queue_rq_affinity: 1,
            queue_write_cache: "write back",
            queue_write_same_max_bytes: 0,
            queue_chunk_sectors: Some(
                0,
            ),
            queue_zoned: Some(
                "none",
            ),
        },
    ],
}
```
(edited for readability)
 */

use std::num::{ParseFloatError, ParseIntError};

use thiserror::Error;
//use anyhow::Result;
//use color_eyre::eyre::Result;

#[derive(Error, Debug)]
pub enum ProcSysParserError {
    /// This error shouldn't happen, unless the source data has changed 
    #[error("Expected item is not found on iterator: {item}.")]
    IteratorItemError { item: String },
    /// This error shouldn't happen, unless the source data has changed 
    #[error("Expected character cannot be found: {item}.")]
    FindItemError { item: String },
    /// This error means a string that is picked up and expected to be a float cannot be parsed
    /// into a float.
    #[error("Error during parsing string to float")]
    ParseToFloatError(#[from] ParseFloatError),
    /// This error means a string that is picked up and expected to be an integer cannot be parsed
    /// into an integer.
    #[error("Error during parsing string to integer")]
    ParseToIntegerError(#[from] ParseIntError),
    /// This error means the file to be read cannot be found or is unreadable.
    #[error("Error {error} during reading file {file}.")]
    FileReadError { file: String, error: std::io::Error },
    /// This error means the file to be read cannot be found or is unreadable.
    #[error("Error {error} during reading directory {directory}.")]
    DirectoryReadError { directory: String, error: std::io::Error },
    // This error means the regex cannot be compiled.
    #[error("Error during compilation regex: {regex}.")]
    RegexCompileError { regex: String },
    //
}

/*
impl From<ParseFloatError> for ProcSysParserError {
    fn from(e: <ParseFloatError as TryFrom>::Error) -> Self {
        ProcSysParserError::ParseToNumberError
    }
}
*/

pub mod block;
pub mod diskstats;
pub mod loadavg;
pub mod meminfo;
pub mod net_dev;
pub mod pressure;
pub mod schedstat;
pub mod stat;
pub mod vmstat;
