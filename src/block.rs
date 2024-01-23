/*!
Read `/sys/block` for block devices into the struct [`SysBlock`].

The documentation for `/sys/block` is found here:
- `/sys/block`: <https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-block>
- `/sys/block/<disk>/stat`: <https://docs.kernel.org/block/stat.html>
- `/sys/block/<disk>/queue`: <https://www.kernel.org/doc/Documentation/block/queue-sysfs.txt>

The main disk IO information is found in `/sys/block/<dev>/stat`, which is mostly equal to `/proc/diskstats`.

Here is an example obtaining the data from `/proc/sys_block`:
```no_run
use proc_sys_parser::{block, block::SysBlock};

let proc_block = block::read();

println!("{:#?}", proc_block);
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

If you want to change the directory that is read for the blockdevices for [`SysBlock`], which is `/sys/block`
by default, use:
```no_run
use proc_sys_parser::{block, block::{SysBlock, Builder}};

let proc_block = Builder::new().path("/my-sys/block").read();
```
*/
use std::io::Error;
use std::fs::{read_to_string, read_dir, DirEntry};

/// Struct for holding `/sys/block` block device statistics and information
#[derive(Debug, PartialEq)]
pub struct SysBlock {
    pub block_devices: Vec<BlockDevice>
}

/// Builder pattern for [`SysBlock`]
pub struct Builder
{
    pub sys_block_path: String
}

impl Default for Builder
{
    fn default() -> Self
    {
        Self::new()
    }
}

impl Builder
{
    pub fn new() -> Builder
    {
        Builder { sys_block_path: "/sys/block".to_string() }
    }

    pub fn path(mut self, sys_block_path: &str) -> Builder
    {
        self.sys_block_path = sys_block_path.to_string();
        self
    }
    pub fn read(self) -> SysBlock
    {
        SysBlock::read_sys_block_devices(&self.sys_block_path)
    }
}

/// The main function for building a [`SysBlock`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> SysBlock
{
   Builder::new().read()
}
impl Default for SysBlock
{
    fn default() -> Self {
        Self::new()
    }
}

/// Struct for holding `/sys/block/<device>` statistics and information
#[derive(Debug, PartialEq, Default)]
pub struct BlockDevice {
    /// `/sys/block/<device>` name.
    pub device_name: String,
    //----------------------------------------------------------------------------------------------------------------//
    /// `/sys/block/<device>/alignment_offset`
    /// Number of bytes at the beginning of the device is offset from the disks natural alignment.
    pub alignment_offset: u64,
    /// `/sys/block/<device>/cache_type`
    /// | cache_type STRING     | write cache | read cache |
    /// |-----------------------|-------------|------------|
    /// |"write through"        | off         | on         |
    /// |"none"                 | off         | off        |
    /// |"write back"           | on          | on         |
    /// |"write back, no read"  | on          | off        |
    ///
    /// <https://docs.kernel.org/scsi/sd-parameters.html>
    pub cache_type: Option<String>,
    /// From the `/sys/block/<device>/dev` file: block major number.
    pub dev_block_major: u64,
    /// From the `/sys/block/<device>/dev` file: block major number.
    pub dev_block_minor: u64,
    /// `/sys/block/<device>/discard_alignment`
    /// Devices that support discard functionality may internally allocate space in units that are bigger than the exported
    /// logical block size.
    /// This parameter indicates how many bytes the beginning of the device is offset from the internal allocation unit's
    /// natural assignment.
    pub discard_alignment: u64,
    /// `/sys/block/<device>/diskseq`
    /// Disk sequence number, which is a monotonically increasing number assigned to every drive.
    /// This file does not exist on EL7.
    pub diskseq: Option<u64>,
    /// `/sys/block/<device>/hidden`
    /// The block device is hidden. It doesn't produce events, and can't be openend from userspace.
    /// Used for the underlying components of multipath devices.
    pub hidden: u64,
    /// `/sys/block/<device>/inflight`
    /// Reports the number of pending IO requests in a device driver.
    /// The inflight file contains two fields: reads and writes.
    /// Number of read requests.
    pub inflight_reads: u64,
    /// `/sys/block/<device>/inflight`
    /// Reports the number of pending IO requests in a device driver.
    /// The inflight file contains two fields: reads and writes.
    /// Number of write requests.
    pub inflight_writes: u64,
    /// `/sys/block/<device>/queue/add_random`
    /// Disk entropy contribution.
    pub queue_add_random: u64,
    /// `/sys/block/<device>/queue/chunk_sectors`
    /// Kernel 4.10+
    /// For a RAID device (dm-raid), this is the size in 512 bytes sectors of the RAID volume stripe segment.
    /// For a zoned block device, this is the size in 512 bytes sectors of the zones of the device.
    pub queue_chunk_sectors: Option<u64>,
    /// `/sys/block/<device>/queue/dax`
    /// Does the device support direct access (DAX)? 0/no, 1/yes.
    /// DAX is used by CPU-addressable storage to bypass the pagecache.
    pub queue_dax: u64,
    /// `/sys/block/<device>/queue/discard_granularity`
    /// The size of the internal allocation of the device in bytes.
    /// A value of '0' means the device does not support the discard functionality.
    pub queue_discard_granularity: u64,
    /// `/sys/block/<device>/queue/discard_max_bytes`
    /// This is the current set maximum bytes as limit for the device.
    /// Some devices might exhibit large latencies when large discards are issued, for which this setting can reduce the amount
    /// of bytes discarded in a single operation, potentially reducing latency.
    pub queue_discard_max_bytes: u64,
    /// `/sys/block/<device>/queue/discard_max_hw_bytes`
    /// Devices that have discard functionality may have internal limits on the number of bytes that can be trimmed or unmapped.
    /// This value is set by the driver to indicate the maximum amount that can be discarded in a single operation.
    /// A value of '0' means the device does not support the discard functionality.
    pub queue_discard_max_hw_bytes: u64,
    /// `/sys/block/<device>/queue/hw_sector_size`
    /// The hardware sector size of the device, in bytes.
    pub queue_hw_sector_size: u64,
    /// `/sys/block/<device>/queue/io_poll`
    /// Is polling enabled? 0/no, 1/yes.
    pub queue_io_poll: u64,
    /// `/sys/block/<device>/queue/io_poll_delay`
    /// If polling is enabled, this controls what kind of polling will be performed.
    /// The default is -1, classic polling.
    /// Other modes:
    /// 0: hybrid polling: kernel makes an educated guess when the IO will be complete. This might be somewhat
    /// slower than classic polling, but is more efficient.
    /// >0: number of microseconds before classic polling.
    pub queue_io_poll_delay: i64,
    /// `/sys/block/<device>/queue/logical_block_size`
    /// The logical block size of the device, in bytes.
    pub queue_logical_block_size: u64,
    /// `/sys/block/<device>/queue/max_discard_segments`
    /// The maximum number of DMA scatter/gather entries in a discard request.
    pub queue_max_discard_segments: u64,
    /// `/sys/block/<device>/queue/max_hw_sectors_kb`
    /// The maximum IO size allowed by the driver.
    /// Size is in kilobytes.
    pub queue_max_hw_sectors_kb: u64,
    /// `/sys/block/<device>/queue/max_sectors_kb`
    /// The current set maximum IO size. (limited to max_hw_sectors_kb)
    /// Size is in kilobytes.
    pub queue_max_sectors_kb: u64,
    /// `/sys/block/<device>/queue/max_integrity_segments`
    /// The maximum number of elements in a DMA scatter/gather list with integrity data that will be submitted
    /// by the block layer core to the associated driver.
    pub queue_max_integrity_segments: u64,
    /// `/sys/block/<device>/queue/max_segments`
    pub queue_max_segments: u64,
    /// `/sys/block/<device>/queue/max_segment_size`
    pub queue_max_segment_size: u64,
    /// `/sys/block/<device>/queue/minimum_io_size`
    /// The smallest preferred IO size reported by the device
    pub queue_minimum_io_size: u64,
    /// `/sys/block/<device>/queue/nomerges`
    /// Setting for disabling the lookup logic involved with IO merging.
    /// Settings:
    /// 0: all merges enabled (default)
    /// 1: only simple one-hit merges will be tried.
    /// 2: no merge algorithms will be tried.
    pub queue_nomerges: u64,
    /// `/sys/block/<device>/queue/nr_requests`
    /// The current set maximum queue size independently for reads and writes.
    /// This means the actual queue size can be potentialy nr_requests*2!
    pub queue_nr_requests: u64,
    /// `/sys/block/<device>/queue/nr_zones`
    /// Kernel 4.20+
    /// Total number of zones
    pub queue_nr_zones: Option<u64>,
    /// `/sys/block/<device>/queue/optimal_io_size`
    /// The optimal io size reported by the device, in bytes.
    pub queue_optimal_io_size: u64,
    /// `/sys/block/<device>/queue/physical_block_size`
    /// The physical block size of the device, in bytes.
    pub queue_physical_block_size: u64,
    /// `/sys/block/<device>/queue/read_ahead_kb`
    /// The maximum number of kilobytes to read-ahead for filesystems on this block device.
    pub queue_read_ahead_kb: u64,
    /// `/sys/block/<device>/queue/rotational`
    /// Is the device of rotating type? 0/no, 1/yes.
    pub queue_rotational: u64,
    /// `/sys/block/<device>/queue/rq_affinity`
    /// - 1: the block layer will migrate req. completions to the cpu group that originally submitted
    ///  the request. Some workloads can reduce cpu cycles due to caching effects.
    /// - 2: force completion to run on the requesting cpu (bypassing the group aggregate function)
    ///  this maximizes distribution.
    pub queue_rq_affinity: u64,
    /// `/sys/block/<device>/queue/scheduler`
    /// The scheduler file contains all available IO schedulers, and the current set IO scheduler is enclosed in '[]' brackets.
    /// When the file is parsed, it takes the current scheduler enclosed in the brackets.
    pub queue_scheduler: String,
    /// `/sys/block/<device>/queue/write_cache`
    /// Whether the device has:
    /// - "write back": write back caching enabled.
    /// - "write through": no write back caching.
    pub queue_write_cache: String,
    /// `/sys/block/<device>/queue/write_write_same_max_bytes`
    /// The number of bytes the device can write in a single write-same command.
    /// A value of '0' means write-same is not supported by the device.
    pub queue_write_same_max_bytes: u64,
    /// `/sys/block/<device>/queue/zoned`
    /// Kernel 4.10+
    /// Indicates whether the device is a zoned blockdevice, and the zone model:
    /// - "none": not zoned
    /// - "host-aware"
    /// - "host-managed"
    /// - "drive-managed": shows as "none".
    pub queue_zoned: Option<String>,
    /// `/sys/block/<device>/range`
    /// ?? No documentation found.
    pub range: u64,
    /// `/sys/block/<device>/removable`
    /// Is the device removable? 0/no, 1/yes.
    pub removable: u64,
    /// `/sys/block/<device>/ro`
    /// Is the device readonly? 0/no, 1/yes.
    pub ro: u64,
    /// `/sys/block/<device>/size`
    /// The size of the block device in sectors.
    /// Sector size is 512 bytes.
    pub size: u64,
    /// The stat file contents are in order of the fields:
    /// From the `/sys/block/<device>/stat` file: number of read requests IOs processed.
    pub stat_reads_completed_success: u64,
    /// From the `/sys/block/<device>/stat` file: number of read requests IOs merged with in-queue IO.
    pub stat_reads_merged: u64,
    /// From the `/sys/block/<device>/stat` file: number of sectors read.
    pub stat_reads_sectors: u64,
    /// From the `/sys/block/<device>/stat` file: total time waited for read requests.
    /// Sector size is 512 bytes.
    pub stat_reads_time_spent_ms: u64,
    /// From the `/sys/block/<device>/stat` file: number of write requests IOs processed.
    /// Time is in milliseconds.
    pub stat_writes_completed_success: u64,
    /// From the `/sys/block/<device>/stat` file: number of write requests IOs merged with in-queue IO.
    pub stat_writes_merged: u64,
    /// From the `/sys/block/<device>/stat` file: number of sectors written.
    pub stat_writes_sectors: u64,
    /// From the `/sys/block/<device>/stat` file: total time waited for write requests.
    /// Sector size is 512 bytes.
    pub stat_writes_time_spent_ms: u64,
    /// From the `/sys/block/<device>/stat` file: number of current IOs.
    /// Time is in milliseconds.
    pub stat_ios_in_progress: u64,
    /// From the `/sys/block/<device>/stat` file: total time this device has been active.
    pub stat_ios_time_spent_ms: u64,
    /// From the `/sys/block/<device>/stat` file: total wait time for all requests.
    /// Time is in milliseconds.
    pub stat_ios_weighted_time_spent_ms: u64,
    /// kernel 4.18+, returns none if field not found.
    /// From the `/sys/block/<device>/stat` file: number of discard request IOs processed.
    /// Time is in milliseconds.
    pub stat_discards_completed_success: Option<u64>,
    /// kernel 4.18+, returns none if field not found.
    /// From the `/sys/block/<device>/stat` file: number of discard request IOs merged with in-queue IO.
    pub stat_discards_merged: Option<u64>,
    /// kernel 4.18+, returns none if field not found.
    /// From the `/sys/block/<device>/stat` file: number of sectors discarded.
    pub stat_discards_sectors: Option<u64>,
    /// kernel 4.18+, returns none if field not found.
    /// From the `/sys/block/<device>/stat` file: number of sectors discarded.
    /// Sector size is 512 bytes.
    pub stat_discards_time_spent_ms: Option<u64>,
    /// kernel 5.5+, returns none if field not found.
    /// From the `/sys/block/<device>/stat` file: number of flush IOs processed.
    /// The block layer combines flush requests and executes at most one at a time.
    /// Not tracked for partitions. <https://docs.kernel.org/block/stat.html>
    pub stat_flush_requests_completed_success: Option<u64>,
    /// kernel 5.5+, returns none if field not found.
    /// From the `/sys/block/<device>/stat` file: total wit time for flush requests.
    /// Time is in milliseconds.
    pub stat_flush_requests_time_spent_ms: Option<u64>,
}

impl BlockDevice
{
    pub fn new() -> BlockDevice
    {
        BlockDevice::default()
    }
}

impl SysBlock {
    pub fn new() -> SysBlock {
        SysBlock {
           block_devices: vec![],
        }
    }
    fn parse_alignment_offset(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let alignment_offset = read_to_string(blockdevice_dir.path().join("alignment_offset")).unwrap_or_else(|error| panic!("Error {} reading block device alignment_offset sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.alignment_offset = alignment_offset.parse::<u64>().unwrap();
    }
    fn parse_cache_type(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        blockdevice_data.cache_type = match read_to_string(blockdevice_dir.path().join("cache_type")) {
            Ok(result) => Some(result.trim_end_matches('\n').to_string()),
            Err(_) => None,
        };
    }
    fn parse_dev(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let dev_contents = read_to_string(blockdevice_dir.path().join("dev")).unwrap_or_else(|error| panic!("Error {} reading block device dev sysfs entry", error)).trim_end_matches('\n').to_string();
        let mut fields = dev_contents.split(':');
        blockdevice_data.dev_block_major = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.dev_block_minor = fields.next().unwrap().parse::<u64>().unwrap();
    }
    fn parse_discard_alignment(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let discard_alignment = read_to_string(blockdevice_dir.path().join("discard_alignment")).unwrap_or_else(|error| panic!("Error {} reading block device discard_alignment sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.discard_alignment = discard_alignment.parse::<u64>().unwrap();
    }
    fn parse_diskseq(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        blockdevice_data.diskseq = match read_to_string(blockdevice_dir.path().join("diskseq")) {
            Ok(result) => result.trim_end_matches('\n').parse::<u64>().ok(),
            Err(_) => None,
        };
    }
    fn parse_hidden(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let hidden = read_to_string(blockdevice_dir.path().join("hidden")).unwrap_or_else(|error| panic!("Error {} reading block device hidden sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.hidden = hidden.parse::<u64>().unwrap();
    }
    fn parse_inflight(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let inflight_from_file = read_to_string(blockdevice_dir.path().join("inflight")).unwrap_or_else(|error| panic!("Error {} reading block device inflight sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.inflight_reads = inflight_from_file.split_whitespace().nth(0).unwrap().parse::<u64>().unwrap();
        blockdevice_data.inflight_writes = inflight_from_file.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap();
    }
    fn parse_queue_add_random(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let add_random = read_to_string(blockdevice_dir.path().join("queue").join("add_random")).unwrap_or_else(|error| panic!("Error {} reading block device queue/add_random sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_add_random = add_random.parse::<u64>().unwrap();
    }
    pub fn parse_queue_chunk_sectors(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let chunk_sectors = read_to_string(blockdevice_dir.path().join("queue").join("chunk_sectors"));
        blockdevice_data.queue_chunk_sectors = SysBlock::parse_read_and_conversion_into_option_u64(chunk_sectors);
    }
    fn parse_queue_dax(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let dax = read_to_string(blockdevice_dir.path().join("queue").join("dax")).unwrap_or_else(|error| panic!("Error {} reading block device queue/dax sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_dax = dax.parse::<u64>().unwrap();
    }
    fn parse_queue_discard_granularity(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let discard_granularity = read_to_string(blockdevice_dir.path().join("queue").join("discard_granularity")).unwrap_or_else(|error| panic!("Error {} reading block device queue/discard_granularity sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_discard_granularity = discard_granularity.parse::<u64>().unwrap();
    }
    fn parse_queue_discard_max_bytes(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let discard_max_bytes = read_to_string(blockdevice_dir.path().join("queue").join("discard_max_bytes")).unwrap_or_else(|error| panic!("Error {} reading block device queue/discard_max_bytes sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_discard_max_bytes = discard_max_bytes.parse::<u64>().unwrap();
    }
    fn parse_queue_discard_max_hw_bytes(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let discard_max_hw_bytes = read_to_string(blockdevice_dir.path().join("queue").join("discard_max_hw_bytes")).unwrap_or_else(|error| panic!("Error {} reading block device queue/discard_max_hw_bytes sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_discard_max_hw_bytes = discard_max_hw_bytes.parse::<u64>().unwrap();
    }
    fn parse_queue_hw_sector_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let hw_sector_rize = read_to_string(blockdevice_dir.path().join("queue").join("hw_sector_size")).unwrap_or_else(|error| panic!("Error {} reading block device queue/hw_sector_size sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_hw_sector_size = hw_sector_rize.parse::<u64>().unwrap();
    }
    fn parse_queue_io_poll(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let io_poll = read_to_string(blockdevice_dir.path().join("queue").join("io_poll")).unwrap_or_else(|error| panic!("Error {} reading block device queue/io_poll sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_io_poll = io_poll.parse::<u64>().unwrap();
    }
    fn parse_queue_io_poll_delay(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let io_poll_delay = read_to_string(blockdevice_dir.path().join("queue").join("io_poll_delay")).unwrap_or_else(|error| panic!("Error {} reading block device queue/io_poll_delay sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_io_poll_delay = io_poll_delay.parse::<i64>().unwrap();
    }
    fn parse_queue_logical_block_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let logical_block_size = read_to_string(blockdevice_dir.path().join("queue").join("logical_block_size")).unwrap_or_else(|error| panic!("Error {} reading block device queue/logical_block_size sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_logical_block_size = logical_block_size.parse::<u64>().unwrap();
    }
    fn parse_queue_max_discard_segments(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let max_discard_segments = read_to_string(blockdevice_dir.path().join("queue").join("max_discard_segments")).unwrap_or_else(|error| panic!("Error {} reading block device queue/max_discard_segments sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_max_discard_segments = max_discard_segments.parse::<u64>().unwrap();
    }
    fn parse_queue_max_hw_sectors_kb(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let max_hw_sectors_kb_contents = read_to_string(blockdevice_dir.path().join("queue").join("max_hw_sectors_kb")).unwrap_or_else(|error| panic!("Error {} reading block device queue/max_hw_sectors_kb sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_max_hw_sectors_kb = max_hw_sectors_kb_contents.parse::<u64>().unwrap();
    }
    fn parse_queue_max_integrity_segments(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let max_integrity_segments = read_to_string(blockdevice_dir.path().join("queue").join("max_integrity_segments")).unwrap_or_else(|error| panic!("Error {} reading block device queue/max_integrity_segments sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_max_integrity_segments = max_integrity_segments.parse::<u64>().unwrap();
    }
    fn parse_queue_max_sectors_kb(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let max_sectors_kb = read_to_string(blockdevice_dir.path().join("queue").join("max_sectors_kb")).unwrap_or_else(|error| panic!("Error {} reading block device queue/max_sectors_kb sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_max_sectors_kb = max_sectors_kb.parse::<u64>().unwrap();
    }
    fn parse_queue_max_segment_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let max_segment_size = read_to_string(blockdevice_dir.path().join("queue").join("max_segment_size")).unwrap_or_else(|error| panic!("Error {} reading block device queue/max_segment_size sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_max_segment_size = max_segment_size.parse::<u64>().unwrap();
    }
    fn parse_queue_max_segments(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let max_segments = read_to_string(blockdevice_dir.path().join("queue").join("max_segments")).unwrap_or_else(|error| panic!("Error {} reading block device queue/max_segments sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_max_segments = max_segments.parse::<u64>().unwrap();
    }
    fn parse_queue_minimum_io_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let minimum_io_size = read_to_string(blockdevice_dir.path().join("queue").join("minimum_io_size")).unwrap_or_else(|error| panic!("Error {} reading block device queue/minimum_io_size sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_minimum_io_size = minimum_io_size.parse::<u64>().unwrap();
    }
    fn parse_queue_nomerges(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let nomerges = read_to_string(blockdevice_dir.path().join("queue").join("nomerges")).unwrap_or_else(|error| panic!("Error {} reading block device queue/nomerges sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_nomerges = nomerges.parse::<u64>().unwrap();
    }
    fn parse_queue_nr_requests(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let nr_requests = read_to_string(blockdevice_dir.path().join("queue").join("nr_requests")).unwrap_or_else(|error| panic!("Error {} reading block device queue/nr_requests sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_nr_requests = nr_requests.parse::<u64>().unwrap();
    }
    pub fn parse_queue_nr_zones(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let nr_zones = read_to_string(blockdevice_dir.path().join("queue").join("nr_zones"));
        blockdevice_data.queue_nr_zones = SysBlock::parse_read_and_conversion_into_option_u64(nr_zones);
    }
    fn parse_queue_optimal_io_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let optimal_io_size = read_to_string(blockdevice_dir.path().join("queue").join("optimal_io_size")).unwrap_or_else(|error| panic!("Error {} reading block device queue/optimal_io_size sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_optimal_io_size = optimal_io_size.parse::<u64>().unwrap();
    }
    fn parse_queue_physical_block_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let physical_block_size = read_to_string(blockdevice_dir.path().join("queue").join("physical_block_size")).unwrap_or_else(|error| panic!("Error {} reading block device queue/physical_block_size sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_physical_block_size = physical_block_size.parse::<u64>().unwrap();
    }
    fn parse_queue_read_ahead_kb(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let read_ahead_kb = read_to_string(blockdevice_dir.path().join("queue").join("read_ahead_kb")).unwrap_or_else(|error| panic!("Error {} reading block device queue/read_ahead_kb sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_read_ahead_kb = read_ahead_kb.parse::<u64>().unwrap();
    }
    fn parse_queue_rotational(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let rotational = read_to_string(blockdevice_dir.path().join("queue").join("rotational")).unwrap_or_else(|error| panic!("Error {} reading block device queue/rotational sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_rotational = rotational.parse::<u64>().unwrap();
    }
    fn parse_queue_rq_affinity(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let rq_affinity = read_to_string(blockdevice_dir.path().join("queue").join("rq_affinity")).unwrap_or_else(|error| panic!("Error {} reading block device queue/rq_affinity sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_rq_affinity = rq_affinity.parse::<u64>().unwrap();
    }
    fn parse_queue_scheduler(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let nr_requests = read_to_string(blockdevice_dir.path().join("queue").join("scheduler")).unwrap_or_else(|error| panic!("Error {} reading block device queue/scheduler sysfs entry", error)).trim_end_matches('\n').to_string();
        let left_bracket = nr_requests.find('[');
        let right_bracket = nr_requests.find(']');
        if left_bracket.is_some() && right_bracket.is_some()
        {
            blockdevice_data.queue_scheduler = nr_requests[left_bracket.unwrap()+1..right_bracket.unwrap()].to_string();
        }
        else
        {
            blockdevice_data.queue_scheduler = "?".to_string();
        }
    }
    fn parse_queue_write_cache(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let write_cache = read_to_string(blockdevice_dir.path().join("queue").join("write_cache")).unwrap_or_else(|error| panic!("Error {} reading block device queue/write_cache sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_write_cache = write_cache;
    }
    pub fn parse_queue_write_same_max_bytes(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let write_same_max_bytes = read_to_string(blockdevice_dir.path().join("queue").join("write_same_max_bytes")).unwrap_or_else(|error| panic!("Error {} reading block device queue/write_same_max_bytes sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.queue_write_same_max_bytes = write_same_max_bytes.parse::<u64>().unwrap();
    }
    fn parse_queue_zoned(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let zoned = read_to_string(blockdevice_dir.path().join("queue").join("zoned"));
        blockdevice_data.queue_zoned = match zoned
        {
            Err(_) => None,
            Ok(value) => Some(value.trim_end_matches('\n').to_string()),
        };
    }
    fn parse_range(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let range = read_to_string(blockdevice_dir.path().join("range")).unwrap_or_else(|error| panic!("Error {} reading block device range sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.range = range.parse::<u64>().unwrap();
    }
    fn parse_removable(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let removable = read_to_string(blockdevice_dir.path().join("removable")).unwrap_or_else(|error| panic!("Error {} reading block device removable sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.removable = removable.parse::<u64>().unwrap();
    }
    fn parse_ro(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let ro = read_to_string(blockdevice_dir.path().join("ro")).unwrap_or_else(|error| panic!("Error {} reading block device ro sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.ro = ro.parse::<u64>().unwrap();
    }
    fn parse_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let size = read_to_string(blockdevice_dir.path().join("size")).unwrap_or_else(|error| panic!("Error {} reading block device size sysfs entry", error)).trim_end_matches('\n').to_string();
        blockdevice_data.size = size.parse::<u64>().unwrap();
    }
    fn parse_stat(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let stat_contents = read_to_string(blockdevice_dir.path().join("stat")).unwrap_or_else(|error| panic!("Error {} reading block device stat sysfs entry", error)).trim_end_matches('\n').to_string();
        let mut fields = stat_contents.split_whitespace();
        blockdevice_data.stat_reads_completed_success = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.stat_reads_merged = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.stat_reads_sectors = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.stat_reads_time_spent_ms = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.stat_writes_completed_success = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.stat_writes_merged = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.stat_writes_sectors = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.stat_writes_time_spent_ms = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.stat_ios_in_progress = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.stat_ios_time_spent_ms = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.stat_ios_weighted_time_spent_ms = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.stat_discards_completed_success = SysBlock::parse_next_and_conversion_into_option_u64(fields.next());
        blockdevice_data.stat_discards_merged = SysBlock::parse_next_and_conversion_into_option_u64(fields.next());
        blockdevice_data.stat_discards_sectors = SysBlock::parse_next_and_conversion_into_option_u64(fields.next());
        blockdevice_data.stat_discards_time_spent_ms = SysBlock::parse_next_and_conversion_into_option_u64(fields.next());
        blockdevice_data.stat_flush_requests_completed_success = SysBlock::parse_next_and_conversion_into_option_u64(fields.next());
        blockdevice_data.stat_flush_requests_time_spent_ms = SysBlock::parse_next_and_conversion_into_option_u64(fields.next());
    }
    fn parse_next_and_conversion_into_option_u64(result: Option<&str>) -> Option<u64>
    {
        match result
        {
            None => None,
            Some(value) => {
                match value.parse::<u64>()
                {
                    Err(_) => None,
                    Ok(number) => Some(number),
                }
            },
        }
    }
    fn parse_read_and_conversion_into_option_u64(result: Result<String, Error>) -> Option<u64>
    {
        match result
        {
            Err(_) => None,
            Ok(value) => {
                match value.trim_end_matches('\n').parse::<u64>()
                {
                    Err(_) => None,
                    Ok(number) => Some(number),
                }
            },
        }
    }
    pub fn read_sys_block_devices(sys_block_path: &str) -> SysBlock
    {
        let mut sysblock = SysBlock::new();

        let blockdevice_directories = read_dir(sys_block_path).unwrap_or_else(|error| panic!("Error {} reading sysfs for block devices in path: {}.", error, sys_block_path));

        for blockdevice in blockdevice_directories
        {
            let directory_entry = blockdevice.unwrap_or_else(|error| panic!("Error {} reading block device sysfs entry", error));
            let mut blockdevice_data = BlockDevice::new();

            blockdevice_data.device_name = directory_entry.file_name().into_string().unwrap();
            SysBlock::parse_alignment_offset(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_cache_type(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_dev(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_discard_alignment(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_diskseq(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_hidden(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_inflight(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_add_random(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_chunk_sectors(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_dax(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_discard_granularity(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_discard_max_bytes(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_discard_max_hw_bytes(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_hw_sector_size(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_io_poll(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_io_poll_delay(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_logical_block_size(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_max_discard_segments(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_max_hw_sectors_kb(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_max_integrity_segments(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_max_sectors_kb(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_max_segment_size(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_max_segments(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_minimum_io_size(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_nomerges(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_nr_requests(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_nr_zones(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_optimal_io_size(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_physical_block_size(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_read_ahead_kb(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_rotational(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_rq_affinity(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_scheduler(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_write_cache(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_write_same_max_bytes(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_queue_zoned(&mut blockdevice_data, &directory_entry);

            SysBlock::parse_range(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_removable(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_ro(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_size(&mut blockdevice_data, &directory_entry);
            SysBlock::parse_stat(&mut blockdevice_data, &directory_entry);

            sysblock.block_devices.push(blockdevice_data);
        }

        sysblock
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{write, remove_dir_all, create_dir_all};
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    use super::*;

    #[test]
    fn create_sys_block_device_parse_files()
    {
        let alignment_offset = format!("0\n");
        let cache_type = format!("write back\n");
        let dev= format!("253:0\n");
        let discard_alignment = format!("0\n");
        let diskseq = format!("9\n");
        let hidden = format!("0\n");
        let inflight = format!("       1        2\n");
        let queue_add_random= format!("0\n");
        let queue_chunk_sectors = format!("0\n");
        let queue_dax = format!("0\n");
        let queue_discard_granularity = format!("512\n");
        let queue_discard_max_bytes = format!("2147483136\n");
        let queue_discard_max_hw_bytes = format!("2147483136\n");
        let queue_hw_sector_size = format!("512\n");
        let queue_io_poll = format!("0\n");
        let queue_io_poll_delay = format!("-1\n");
        let queue_logical_block_size = format!("512\n");
        let queue_max_discard_segments = format!("1\n");
        let queue_max_hw_sectors_kb = format!("2147483647\n");
        let queue_max_integrity_segments = format!("0\n");
        let queue_max_sectors_kb = format!("1280\n");
        let queue_max_segment_size = format!("4294967295\n");
        let queue_max_segments = format!("254\n");
        let queue_minimum_io_size = format!("512\n");
        let queue_nomerges = format!("0\n");
        let queue_nr_requests = format!("256\n");
        let queue_nr_zones = format!("0\n");
        let queue_optimal_io_size = format!("0\n");
        let queue_physical_block_size = format!("512\n");
        let queue_read_ahead_kb = format!("128\n");
        let queue_rotational = format!("1\n");
        let queue_rq_affinity = format!("1\n");
        let queue_scheduler = format!("[none] mq-deadline\n");
        let queue_write_cache = format!("write back\n");
        let queue_write_same_max_bytes = format!("0\n");
        let queue_zoned = format!("none\n");
        let range = format!("16\n");
        let removable = format!("0\n");
        let ro = format!("0\n");
        let size = format!("125829120\n");
        let stat = format!("    9718     3826  1052371     3026     2856     2331   312397     1947        0     6004     5554     7141        0 88014755      276      591      304\n");

        let directory_suffix: String = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        create_dir_all(format!("{}/block/sda/queue", test_path)).expect("Error creating mock sysfs directories.");
        write(format!("{}/block/sda/alignment_offset", test_path),alignment_offset).expect("error writing to mock sysfs alignment_offset file.");
        write(format!("{}/block/sda/cache_type", test_path),cache_type).expect("error writing to mock sysfs cache_type file.");
        write(format!("{}/block/sda/dev", test_path),dev).expect("error writing to mock sysfs dev file.");
        write(format!("{}/block/sda/discard_alignment", test_path),discard_alignment).expect("error writing to mock sysfs discard_alginment file.");
        write(format!("{}/block/sda/diskseq", test_path),diskseq).expect("error writing to mock sysfs diskseq file.");
        write(format!("{}/block/sda/hidden", test_path),hidden).expect("error writing to mock sysfs hidden file.");
        write(format!("{}/block/sda/inflight", test_path),inflight).expect("error writing to mock sysfs inflight file.");
        write(format!("{}/block/sda/queue/add_random", test_path),queue_add_random).expect("error writing to mock sysfs queue/add_random file.");
        write(format!("{}/block/sda/queue/chunk_sectors", test_path),queue_chunk_sectors).expect("error writing to mock sysfs queue/chunk_sectors file.");
        write(format!("{}/block/sda/queue/dax", test_path),queue_dax).expect("error writing to mock sysfs queue/dax file.");
        write(format!("{}/block/sda/queue/discard_granularity", test_path),queue_discard_granularity).expect("error writing to mock sysfs queue/discard_granularity file.");
        write(format!("{}/block/sda/queue/discard_max_bytes", test_path),queue_discard_max_bytes).expect("error writing to mock sysfs queue/discard_max_bytes file.");
        write(format!("{}/block/sda/queue/discard_max_hw_bytes", test_path),queue_discard_max_hw_bytes).expect("error writing to mock sysfs queue/discard_max_hw_bytes file.");
        write(format!("{}/block/sda/queue/hw_sector_size", test_path),queue_hw_sector_size).expect("error writing to mock sysfs queue/hw_sector_size file.");
        write(format!("{}/block/sda/queue/io_poll", test_path),queue_io_poll).expect("error writing to mock sysfs queue/io_poll file.");
        write(format!("{}/block/sda/queue/io_poll_delay", test_path),queue_io_poll_delay).expect("error writing to mock sysfs queue/io_poll_delay file.");
        write(format!("{}/block/sda/queue/logical_block_size", test_path),queue_logical_block_size).expect("error writing to mock sysfs queue/logical_block_size file.");
        write(format!("{}/block/sda/queue/max_discard_segments", test_path),queue_max_discard_segments).expect("error writing to mock sysfs queue/max_discard_segments file.");
        write(format!("{}/block/sda/queue/max_hw_sectors_kb", test_path),queue_max_hw_sectors_kb).expect("error writing to mock sysfs queue/max_hw_sectors_kb file.");
        write(format!("{}/block/sda/queue/max_integrity_segments", test_path),queue_max_integrity_segments).expect("error writing to mock sysfs queue/max_integrity_segments file.");
        write(format!("{}/block/sda/queue/max_sectors_kb", test_path),queue_max_sectors_kb).expect("error writing to mock sysfs queue/max_sectors_kb file.");
        write(format!("{}/block/sda/queue/max_segment_size", test_path),queue_max_segment_size).expect("error writing to mock sysfs queue/max_segment_size file.");
        write(format!("{}/block/sda/queue/max_segments", test_path),queue_max_segments).expect("error writing to mock sysfs queue/max_segments file.");
        write(format!("{}/block/sda/queue/minimum_io_size", test_path),queue_minimum_io_size).expect("error writing to mock sysfs queue/minimum_io_size file.");
        write(format!("{}/block/sda/queue/nomerges", test_path),queue_nomerges).expect("error writing to mock sysfs queue/nomerges file.");
        write(format!("{}/block/sda/queue/nr_requests", test_path),queue_nr_requests).expect("error writing to mock sysfs queue/nr_requests file.");
        write(format!("{}/block/sda/queue/nr_zones", test_path),queue_nr_zones).expect("error writing to mock sysfs queue/nr_zones file.");
        write(format!("{}/block/sda/queue/optimal_io_size", test_path),queue_optimal_io_size).expect("error writing to mock sysfs queue/optimal_io_size file.");
        write(format!("{}/block/sda/queue/physical_block_size", test_path),queue_physical_block_size).expect("error writing to mock sysfs queue/physical_block_size file.");
        write(format!("{}/block/sda/queue/read_ahead_kb", test_path),queue_read_ahead_kb).expect("error writing to mock sysfs queue/read_ahead_kb file.");
        write(format!("{}/block/sda/queue/rotational", test_path),queue_rotational).expect("error writing to mock sysfs queue/rotational file.");
        write(format!("{}/block/sda/queue/rq_affinity", test_path),queue_rq_affinity).expect("error writing to mock sysfs queue/rq_affinity file.");
        write(format!("{}/block/sda/queue/scheduler", test_path),queue_scheduler).expect("error writing to mock sysfs queue/scheduler file.");
        write(format!("{}/block/sda/queue/write_cache", test_path),queue_write_cache).expect("error writing to mock sysfs queue/write_cache file.");
        write(format!("{}/block/sda/queue/write_same_max_bytes", test_path),queue_write_same_max_bytes).expect("error writing to mock sysfs queue/write_same_max_bytes file.");
        write(format!("{}/block/sda/queue/zoned", test_path),queue_zoned).expect("error writing to mock sysfs queue/zoned file.");
        write(format!("{}/block/sda/range", test_path),range).expect("error writing to mock sysfs range file.");
        write(format!("{}/block/sda/removable", test_path),removable).expect("error writing to mock sysfs removable file.");
        write(format!("{}/block/sda/ro", test_path),ro).expect("error writing to mock sysfs ro file.");
        write(format!("{}/block/sda/size", test_path),size).expect("error writing to mock sysfs size file.");
        write(format!("{}/block/sda/stat", test_path),stat).expect("error writing to mock sysfs stat file.");

        let result = Builder::new().path(format!("{}/block", test_path).as_str()).read();

        remove_dir_all(test_path).unwrap();

        assert_eq!(result, SysBlock {
            block_devices: vec![
                BlockDevice {
                    dev_block_major: 253,
                    dev_block_minor: 0,
                    device_name: "sda".to_string(),
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
                    cache_type: Some("write back".to_string()),
                    diskseq: Some(9),
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
                    queue_scheduler: "none".to_string(),
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
                    queue_write_cache: "write back".to_string(),
                    queue_write_same_max_bytes: 0,
                    queue_chunk_sectors: Some(
                        0,
                    ),
                    queue_zoned: Some(
                        "none".to_string(),
                    ),
                },
            ],
        }
        );
    }
    #[test]
    fn create_sys_block_device_parse_files_non_existent()
    {
        let alignment_offset = "0";
        let cache_type = "write back";
        let dev = "253:0";
        let discard_alignment = "0";
        let diskseq = "9";
        let hidden = "0";
        let inflight = "       1        2";
        let queue_add_random = "0";
        let queue_dax = "0";
        let queue_discard_granularity = "512";
        let queue_discard_max_bytes = "2147483136";
        let queue_discard_max_hw_bytes = "2147483136";
        let queue_hw_sector_size = "512";
        let queue_io_poll = "0";
        let queue_io_poll_delay = "-1";
        let queue_logical_block_size = "512";
        let queue_max_discard_segments = "1";
        let queue_max_hw_sectors_kb = "2147483647";
        let queue_max_integrity_segments = "0";
        let queue_max_sectors_kb = "1280";
        let queue_max_segment_size = "4294967295";
        let queue_max_segments = "254";
        let queue_minimum_io_size = "512";
        let queue_nomerges = "0";
        let queue_nr_requests = "256";
        let queue_optimal_io_size = "0";
        let queue_physical_block_size = "512";
        let queue_read_ahead_kb = "128";
        let queue_rotational = "1";
        let queue_rq_affinity = "1";
        let queue_scheduler = "[none] mq-deadline";
        let queue_write_cache = "write back";
        let queue_write_same_max_bytes = "0";
        let range = "16";
        let removable = "0";
        let ro = "0";
        let size = "125829120";
        let stat = "    9718     3826  1052371     3026     2856     2331   312397     1947        0     6004     5554";

        let directory_suffix: String = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        
        create_dir_all(format!("{}/block/sda/queue", test_path)).expect("Error creating mock sysfs directories.");
        write(format!("{}/block/sda/alignment_offset", test_path), alignment_offset).expect("error writing to mock sysfs alignment_offset file.");
        write(format!("{}/block/sda/cache_type", test_path), cache_type).expect("error writing to mock sysfs cache_type file.");
        write(format!("{}/block/sda/dev", test_path), dev).expect("error writing to mock sysfs dev file.");
        write(format!("{}/block/sda/discard_alignment", test_path), discard_alignment).expect("error writing to mock sysfs discard_alginment file.");
        write(format!("{}/block/sda/diskseq", test_path), diskseq).expect("error writing to mock sysfs diskseq file.");
        write(format!("{}/block/sda/hidden", test_path), hidden).expect("error writing to mock sysfs hidden file.");
        write(format!("{}/block/sda/inflight", test_path), inflight).expect("error writing to mock sysfs inflight file.");
        write(format!("{}/block/sda/queue/add_random", test_path), queue_add_random).expect("error writing to mock sysfs queue/add_random file.");
        write(format!("{}/block/sda/queue/dax", test_path), queue_dax).expect("error writing to mock sysfs queue/dax file.");
        write(format!("{}/block/sda/queue/discard_granularity", test_path), queue_discard_granularity).expect("error writing to mock sysfs queue/discard_granularity file.");
        write(format!("{}/block/sda/queue/discard_max_bytes", test_path), queue_discard_max_bytes).expect("error writing to mock sysfs queue/discard_max_bytes file.");
        write(format!("{}/block/sda/queue/discard_max_hw_bytes", test_path), queue_discard_max_hw_bytes).expect("error writing to mock sysfs queue/discard_max_hw_bytes file.");
        write(format!("{}/block/sda/queue/hw_sector_size", test_path), queue_hw_sector_size).expect("error writing to mock sysfs queue/hw_sector_size file.");
        write(format!("{}/block/sda/queue/io_poll", test_path), queue_io_poll).expect("error writing to mock sysfs queue/io_poll file.");
        write(format!("{}/block/sda/queue/io_poll_delay", test_path), queue_io_poll_delay).expect("error writing to mock sysfs queue/io_poll_delay file.");
        write(format!("{}/block/sda/queue/logical_block_size", test_path), queue_logical_block_size).expect("error writing to mock sysfs queue/logical_block_size file.");
        write(format!("{}/block/sda/queue/max_discard_segments", test_path), queue_max_discard_segments).expect("error writing to mock sysfs queue/max_discard_segments file.");
        write(format!("{}/block/sda/queue/max_hw_sectors_kb", test_path), queue_max_hw_sectors_kb).expect("error writing to mock sysfs queue/max_hw_sectors_kb file.");
        write(format!("{}/block/sda/queue/max_integrity_segments", test_path), queue_max_integrity_segments).expect("error writing to mock sysfs queue/max_integrity_segments file.");
        write(format!("{}/block/sda/queue/max_sectors_kb", test_path), queue_max_sectors_kb).expect("error writing to mock sysfs queue/max_sectors_kb file.");
        write(format!("{}/block/sda/queue/max_segment_size", test_path), queue_max_segment_size).expect("error writing to mock sysfs queue/max_segment_size file.");
        write(format!("{}/block/sda/queue/max_segments", test_path), queue_max_segments).expect("error writing to mock sysfs queue/max_segments file.");
        write(format!("{}/block/sda/queue/minimum_io_size", test_path), queue_minimum_io_size).expect("error writing to mock sysfs queue/minimum_io_size file.");
        write(format!("{}/block/sda/queue/nomerges", test_path), queue_nomerges).expect("error writing to mock sysfs queue/nomerges file.");
        write(format!("{}/block/sda/queue/nr_requests", test_path), queue_nr_requests).expect("error writing to mock sysfs queue/nr_requests file.");
        write(format!("{}/block/sda/queue/optimal_io_size", test_path), queue_optimal_io_size).expect("error writing to mock sysfs queue/optimal_io_size file.");
        write(format!("{}/block/sda/queue/physical_block_size", test_path), queue_physical_block_size).expect("error writing to mock sysfs queue/physical_block_size file.");
        write(format!("{}/block/sda/queue/read_ahead_kb", test_path), queue_read_ahead_kb).expect("error writing to mock sysfs queue/read_ahead_kb file.");
        write(format!("{}/block/sda/queue/rotational", test_path), queue_rotational).expect("error writing to mock sysfs queue/rotational file.");
        write(format!("{}/block/sda/queue/rq_affinity", test_path), queue_rq_affinity).expect("error writing to mock sysfs queue/rq_affinity file.");
        write(format!("{}/block/sda/queue/scheduler", test_path), queue_scheduler).expect("error writing to mock sysfs queue/scheduler file.");
        write(format!("{}/block/sda/queue/write_cache", test_path), queue_write_cache).expect("error writing to mock sysfs queue/write_cache file.");
        write(format!("{}/block/sda/queue/write_same_max_bytes", test_path), queue_write_same_max_bytes).expect("error writing to mock sysfs queue/write_same_max_bytes file.");
        write(format!("{}/block/sda/range", test_path), range).expect("error writing to mock sysfs range file.");
        write(format!("{}/block/sda/removable", test_path), removable).expect("error writing to mock sysfs removable file.");
        write(format!("{}/block/sda/ro", test_path), ro).expect("error writing to mock sysfs ro file.");
        write(format!("{}/block/sda/size", test_path), size).expect("error writing to mock sysfs size file.");
        write(format!("{}/block/sda/stat", test_path), stat).expect("error writing to mock sysfs stat file.");

        let result = Builder::new().path(format!("{}/block", test_path).as_str()).read();

        remove_dir_all(test_path).unwrap();

        assert_eq!(result,
                   SysBlock {
                       block_devices: vec![
                           BlockDevice {
                               dev_block_major: 253,
                               dev_block_minor: 0,
                               device_name: "sda".to_string(),
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
                               stat_discards_completed_success: None,
                               stat_discards_merged: None,
                               stat_discards_sectors: None,
                               stat_discards_time_spent_ms: None,
                               stat_flush_requests_completed_success: None,
                               stat_flush_requests_time_spent_ms: None,
                               alignment_offset: 0,
                               cache_type: Some("write back".to_string()),
                               diskseq: Some(9),
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
                               queue_nr_zones: None,
                               queue_scheduler: "none".to_string(),
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
                               queue_write_cache: "write back".to_string(),
                               queue_write_same_max_bytes: 0,
                               queue_chunk_sectors: None,
                               queue_zoned: None,
                           },
                       ],
                   }
        );
    }
}
