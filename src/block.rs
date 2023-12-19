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
}
```
(edited for readability)

If you want to change the path and/or file that is read for the blockdevices for [`SysBlock`], which is `/sys/block`
by default, use:
```no_run
use proc_sys_parser::{block, block::{SysBlock, Builder}};

let proc_block = Builder::new().path("/mysys/block").read();
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
pub struct BlockDevice
{
    /// From the `/sys/block/<device>/dev` file: block major number.
    pub dev_block_major: u64,
    /// From the `/sys/block/<device>/dev` file: block major number.
    pub dev_block_minor: u64,
    /// `/sys/block/<device>` name.
    pub device_name: String,
    /// `/sys/block/<device>/discard_alignment`
    /// Devices that support discard functionality may internally allocate space in units that are bigger than the exported
    /// logical block size.
    /// This parameter indicates how many bytes the beginning of the device is offset from the internal allocation unit's
    /// natural assignment.
    pub discard_alignment: u64,
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
    pub cache_type: String,
    /// `/sys/block/<device>/diskseq`
    /// Disk sequence number, which is a monotonically increasing number assigned to every drive.
    pub diskseq: u64,
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
    /// `/sys/block/<device>/range`
    /// ?? No documentation found.
    pub range: u64,
    /// `/sys/block/<device>/removable`
    /// Is the device removable? 0/no, 1/yes.
    pub removable: u64,
    /// `/sys/block/<device>/removable`
    /// Is the device readonly? 0/no, 1/yes.
    pub ro: u64,
    /// `/sys/block/<device>/size`
    /// The size of the block device in sectors.
    /// Sector size is 512 bytes.
    pub size: u64,
    /// `/sys/block/<device>/queue/max_hw_sectors_kb`
    /// The maximum IO size allowed by the driver.
    /// Size is in kilobytes.
    pub queue_max_hw_sectors_kb: u64,
    /// `/sys/block/<device>/queue/max_sectors_kb`
    /// The current set maximum IO size. (limited to max_hw_sectors_kb)
    /// Size is in kilobytes.
    pub queue_max_sectors_kb: u64,
    /// `/sys/block/<device>/queue/max_discard_segments`
    /// The maximum number of DMA scatter/gather entries in a discard request.
    pub queue_max_discard_segments: u64,
    /// `/sys/block/<device>/queue/nr_requests`
    /// The current set maximum queue size independently for reads and writes.
    /// This means the actual queue size can be potentialy nr_requests*2!
    pub queue_nr_requests: u64,
    /// `/sys/block/<device>/queue/nr_zones`
    /// Kernel 4.20+
    /// Total number of zones
    pub queue_nr_zones: Option<u64>,
    /// `/sys/block/<device>/queue/scheduler`
    /// The scheduler file contains all available IO schedulers, and the current set IO scheduler is enclosed in '[]' brackets.
    /// When the file is parsed, it takes the current scheduler enclosed in the brackets.
    pub queue_scheduler: String,
    /// `/sys/block/<device>/queue/rotational`
    /// Is the device of rotating type? 0/no, 1/yes.
    pub queue_rotational: u64,
    /// `/sys/block/<device>/queue/dax`
    /// Does the device support direct access (DAX)? 0/no, 1/yes.
    /// DAX is used by CPU-addressable storage to bypass the pagecache.
    pub queue_dax: u64,
    /// `/sys/block/<device>/queue/add_random`
    /// Disk entropy contribution.
    pub queue_add_random: u64,
    /// `/sys/block/<device>/queue/discard_granularity`
    /// The size of the internal allocation of the device in bytes.
    /// A value of '0' means the device does not support the discard functionality.
    pub queue_discard_granularity: u64,
    /// `/sys/block/<device>/queue/discard_max_hw_bytes`
    /// Devices that have discard functionality may have internal limits on the number of bytes that can be trimmed or unmapped.
    /// This value is set by the driver to indicate the maximum amount that can be discarded in a single operation.
    /// A value of '0' means the device does not support the discard functionality.
    pub queue_discard_max_hw_bytes: u64,
    /// `/sys/block/<device>/queue/discard_max_bytes`
    /// This is the current set maximum bytes as limit for the device.
    /// Some devices might exhibit large latencies when large discards are issued, for which this setting can reduce the amount
    /// of bytes discarded in a single operation, potentially reducing latency.
    pub queue_discard_max_bytes: u64,
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
    /// `/sys/block/<device>/queue/minimum_io_size`
    /// The smallest preferred IO size reported by the device
    pub queue_minimum_io_size: u64,
    /// `/sys/block/<device>/queue/max_integrity_segments`
    /// The maximum number of elements in a DMA scatter/gather list with integrity data that will be submitted
    /// by the block layer core to the associated driver.
    pub queue_max_integrity_segments: u64,
    /// `/sys/block/<device>/queue/max_segments`
    pub queue_max_segments: u64,
    /// `/sys/block/<device>/queue/max_segment_size`
    pub queue_max_segment_size: u64,
    /// `/sys/block/<device>/queue/nomerges`
    /// Setting for disabling the lookup logic involved with IO merging.
    /// Settings:
    /// 0: all merges enabled (default)
    /// 1: only simple one-hit merges will be tried.
    /// 2: no merge algorithms will be tried.
    pub queue_nomerges: u64,
    /// `/sys/block/<device>/queue/physical_block_size`
    /// The physical block size of the device, in bytes.
    pub queue_physical_block_size: u64,
    /// `/sys/block/<device>/queue/optimal_io_size`
    /// The optimal io size reported by the device, in bytes.
    pub queue_optimal_io_size: u64,
    /// `/sys/block/<device>/queue/read_ahead_kb`
    /// The maximum number of kilobytes to read-ahead for filesystems on this block device.
    pub queue_read_ahead_kb: u64,
    /// `/sys/block/<device>/queue/rq_affinity`
    /// - 1: the block layer will migrate req. completions to the cpu group that originally submitted
    ///  the request. Some workloads can reduce cpu cycles due to caching effects.
    /// - 2: force completion to run on the requesting cpu (bypassing the group aggregate function)
    ///  this maximizes distribution.
    pub queue_rq_affinity: u64,
    /// `/sys/block/<device>/queue/write_cache`
    /// Whether the device has:
    /// - "write back": write back caching enabled.
    /// - "write through": no write back caching.
    pub queue_write_cache: String,
    /// `/sys/block/<device>/queue/write_write_same_max_bytes`
    /// The number of bytes the device can write in a single write-same command.
    /// A value of '0' means write-same is not supported by the device.
    pub queue_write_same_max_bytes: u64,
    /// `/sys/block/<device>/queue/chunk_sectors`
    /// Kernel 4.10+
    /// For a RAID device (dm-raid), this is the size in 512 bytes sectors of the RAID volume stripe segment.
    /// For a zoned block device, this is the size in 512 bytes sectors of the zones of the device.
    pub queue_chunk_sectors: Option<u64>,
    /// `/sys/block/<device>/queue/zoned`
    /// Kernel 4.10+
    /// Indicates whether the device is a zoned blockdevice, and the zone model:
    /// - "none": not zoned
    /// - "host-aware"
    /// - "host-managed"
    /// - "drive-managed": shows as "none".
    pub queue_zoned: Option<String>,
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
    fn parse_stat(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let stat_contents = read_to_string(blockdevice_dir.path().join("stat")).unwrap_or_else(|error| panic!("Error {} reading block device stat sysfs entry", error));
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
                    Ok(number) => {
                        Some(number)
                    },
                }
            },
        }
    }
    fn parse_dev(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let dev_contents = read_to_string(blockdevice_dir.path().join("dev")).unwrap_or_else(|error| panic!("Error {} reading block device dev sysfs entry", error));
        let mut fields = dev_contents.split(':');
        blockdevice_data.dev_block_major = fields.next().unwrap().parse::<u64>().unwrap();
        blockdevice_data.dev_block_minor = fields.next().unwrap().parse::<u64>().unwrap();
    }

    fn parse_alignment_offset(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let alignment_offset = read_to_string(blockdevice_dir.path().join("alignment_offset")).unwrap_or_else(|error| panic!("Error {} reading block device alignment_offset sysfs entry", error));
        blockdevice_data.alignment_offset = alignment_offset.parse::<u64>().unwrap();
    }
    fn parse_discard_alignment(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let discard_alignment = read_to_string(blockdevice_dir.path().join("discard_alignment")).unwrap_or_else(|error| panic!("Error {} reading block device discard_alignment sysfs entry", error));
        blockdevice_data.discard_alignment = discard_alignment.parse::<u64>().unwrap();
    }
    fn parse_diskseq(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let diskseq = read_to_string(blockdevice_dir.path().join("diskseq")).unwrap_or_else(|error| panic!("Error {} reading block device diskseq sysfs entry", error));
        blockdevice_data.diskseq = diskseq.parse::<u64>().unwrap();
    }
    fn parse_hidden(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let hidden = read_to_string(blockdevice_dir.path().join("hidden")).unwrap_or_else(|error| panic!("Error {} reading block device hidden sysfs entry", error));
        blockdevice_data.hidden = hidden.parse::<u64>().unwrap();
    }
    fn parse_range(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let range = read_to_string(blockdevice_dir.path().join("range")).unwrap_or_else(|error| panic!("Error {} reading block device range sysfs entry", error));
        blockdevice_data.range = range.parse::<u64>().unwrap();
    }
    fn parse_removable(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let removable = read_to_string(blockdevice_dir.path().join("removable")).unwrap_or_else(|error| panic!("Error {} reading block device removable sysfs entry", error));
        blockdevice_data.removable = removable.parse::<u64>().unwrap();
    }
    fn parse_ro(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let ro = read_to_string(blockdevice_dir.path().join("ro")).unwrap_or_else(|error| panic!("Error {} reading block device ro sysfs entry", error));
        blockdevice_data.ro = ro.parse::<u64>().unwrap();
    }
    fn parse_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let size = read_to_string(blockdevice_dir.path().join("size")).unwrap_or_else(|error| panic!("Error {} reading block device size sysfs entry", error));
        blockdevice_data.size = size.parse::<u64>().unwrap();
    }
    fn parse_inflight(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let inflight_from_file = read_to_string(blockdevice_dir.path().join("inflight")).unwrap_or_else(|error| panic!("Error {} reading block device inflight sysfs entry", error));
        blockdevice_data.inflight_reads = inflight_from_file.split_whitespace().nth(0).unwrap().parse::<u64>().unwrap();
        blockdevice_data.inflight_writes = inflight_from_file.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap();
    }
    fn parse_cache_type(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let cache_type = read_to_string(blockdevice_dir.path().join("cache_type")).unwrap_or_else(|error| panic!("Error {} reading block device cache_type sysfs entry", error));
        blockdevice_data.cache_type = cache_type;
    }
    fn parse_queue_max_hw_sectors_kb(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let max_hw_sectors_kb_contents = read_to_string(blockdevice_dir.path().join("queue").join("max_hw_sectors_kb")).unwrap_or_else(|error| panic!("Error {} reading block device queue/max_hw_sectors_kb sysfs entry", error));
        blockdevice_data.queue_max_hw_sectors_kb = max_hw_sectors_kb_contents.parse::<u64>().unwrap();
    }
    fn parse_queue_max_sectors_kb(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let max_sectors_kb = read_to_string(blockdevice_dir.path().join("queue").join("max_sectors_kb")).unwrap_or_else(|error| panic!("Error {} reading block device queue/max_sectors_kb sysfs entry", error));
        blockdevice_data.queue_max_sectors_kb = max_sectors_kb.parse::<u64>().unwrap();
    }
    fn parse_queue_max_integrity_segments(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let max_integrity_segments = read_to_string(blockdevice_dir.path().join("queue").join("max_integrity_segments")).unwrap_or_else(|error| panic!("Error {} reading block device queue/max_integrity_segments sysfs entry", error));
        blockdevice_data.queue_max_integrity_segments = max_integrity_segments.parse::<u64>().unwrap();
    }
    fn parse_queue_max_segment_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let max_segment_size = read_to_string(blockdevice_dir.path().join("queue").join("max_segment_size")).unwrap_or_else(|error| panic!("Error {} reading block device queue/max_segment_size sysfs entry", error));
        blockdevice_data.queue_max_segment_size = max_segment_size.parse::<u64>().unwrap();
    }
    fn parse_queue_max_segments(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let max_segments = read_to_string(blockdevice_dir.path().join("queue").join("max_segments")).unwrap_or_else(|error| panic!("Error {} reading block device queue/max_segments sysfs entry", error));
        blockdevice_data.queue_max_segments = max_segments.parse::<u64>().unwrap();
    }
    fn parse_queue_minimum_io_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let minimum_io_size = read_to_string(blockdevice_dir.path().join("queue").join("minimum_io_size")).unwrap_or_else(|error| panic!("Error {} reading block device queue/minimum_io_size sysfs entry", error));
        blockdevice_data.queue_minimum_io_size = minimum_io_size.parse::<u64>().unwrap();
    }
    fn parse_queue_nomerges(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let nomerges = read_to_string(blockdevice_dir.path().join("queue").join("nomerges")).unwrap_or_else(|error| panic!("Error {} reading block device queue/nomerges sysfs entry", error));
        blockdevice_data.queue_nomerges = nomerges.parse::<u64>().unwrap();
    }
    fn parse_queue_max_discard_segments(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let max_discard_segments = read_to_string(blockdevice_dir.path().join("queue").join("max_discard_segments")).unwrap_or_else(|error| panic!("Error {} reading block device queue/max_discard_segments sysfs entry", error));
        blockdevice_data.queue_max_discard_segments = max_discard_segments.parse::<u64>().unwrap();
    }
    fn parse_queue_nr_requests(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let nr_requests = read_to_string(blockdevice_dir.path().join("queue").join("nr_requests")).unwrap_or_else(|error| panic!("Error {} reading block device queue/nr_requests sysfs entry", error));
        blockdevice_data.queue_nr_requests = nr_requests.parse::<u64>().unwrap();
    }
    fn parse_queue_optimal_io_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let optimal_io_size = read_to_string(blockdevice_dir.path().join("queue").join("optimal_io_size")).unwrap_or_else(|error| panic!("Error {} reading block device queue/optimal_io_size sysfs entry", error));
        blockdevice_data.queue_optimal_io_size = optimal_io_size.parse::<u64>().unwrap();
    }
    fn parse_queue_physical_block_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let physical_block_size = read_to_string(blockdevice_dir.path().join("queue").join("physical_block_size")).unwrap_or_else(|error| panic!("Error {} reading block device queue/physical_block_size sysfs entry", error));
        blockdevice_data.queue_physical_block_size = physical_block_size.parse::<u64>().unwrap();
    }
    fn parse_queue_read_ahead_kb(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let read_ahead_kb = read_to_string(blockdevice_dir.path().join("queue").join("read_ahead_kb")).unwrap_or_else(|error| panic!("Error {} reading block device queue/read_ahead_kb sysfs entry", error));
        blockdevice_data.queue_read_ahead_kb = read_ahead_kb.parse::<u64>().unwrap();
    }
    fn parse_queue_rotational(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let rotational = read_to_string(blockdevice_dir.path().join("queue").join("rotational")).unwrap_or_else(|error| panic!("Error {} reading block device queue/rotational sysfs entry", error));
        blockdevice_data.queue_rotational = rotational.parse::<u64>().unwrap();
    }
    fn parse_queue_rq_affinity(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let rq_affinity = read_to_string(blockdevice_dir.path().join("queue").join("rq_affinity")).unwrap_or_else(|error| panic!("Error {} reading block device queue/rq_affinity sysfs entry", error));
        blockdevice_data.queue_rq_affinity = rq_affinity.parse::<u64>().unwrap();
    }
    fn parse_queue_dax(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let dax = read_to_string(blockdevice_dir.path().join("queue").join("dax")).unwrap_or_else(|error| panic!("Error {} reading block device queue/dax sysfs entry", error));
        blockdevice_data.queue_dax = dax.parse::<u64>().unwrap();
    }
    fn parse_queue_add_random(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let add_random = read_to_string(blockdevice_dir.path().join("queue").join("add_random")).unwrap_or_else(|error| panic!("Error {} reading block device queue/add_random sysfs entry", error));
        blockdevice_data.queue_add_random = add_random.parse::<u64>().unwrap();
    }
    fn parse_queue_discard_granularity(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let discard_granularity = read_to_string(blockdevice_dir.path().join("queue").join("discard_granularity")).unwrap_or_else(|error| panic!("Error {} reading block device queue/discard_granularity sysfs entry", error));
        blockdevice_data.queue_discard_granularity = discard_granularity.parse::<u64>().unwrap();
    }
    fn parse_queue_hw_sector_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let hw_sector_rize = read_to_string(blockdevice_dir.path().join("queue").join("hw_sector_size")).unwrap_or_else(|error| panic!("Error {} reading block device queue/hw_sector_size sysfs entry", error));
        blockdevice_data.queue_hw_sector_size = hw_sector_rize.parse::<u64>().unwrap();
    }
    fn parse_queue_io_poll(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let io_poll = read_to_string(blockdevice_dir.path().join("queue").join("io_poll")).unwrap_or_else(|error| panic!("Error {} reading block device queue/io_poll sysfs entry", error));
        blockdevice_data.queue_io_poll = io_poll.parse::<u64>().unwrap();
    }
    fn parse_queue_io_poll_delay(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let io_poll_delay = read_to_string(blockdevice_dir.path().join("queue").join("io_poll_delay")).unwrap_or_else(|error| panic!("Error {} reading block device queue/io_poll_delay sysfs entry", error));
        blockdevice_data.queue_io_poll_delay = io_poll_delay.parse::<i64>().unwrap();
    }
    fn parse_queue_logical_block_size(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let logical_block_size = read_to_string(blockdevice_dir.path().join("queue").join("logical_block_size")).unwrap_or_else(|error| panic!("Error {} reading block device queue/logical_block_size sysfs entry", error));
        blockdevice_data.queue_logical_block_size = logical_block_size.parse::<u64>().unwrap();
    }
    fn parse_queue_discard_max_bytes(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let discard_max_bytes = read_to_string(blockdevice_dir.path().join("queue").join("discard_max_bytes")).unwrap_or_else(|error| panic!("Error {} reading block device queue/discard_max_bytes sysfs entry", error));
        blockdevice_data.queue_discard_max_bytes = discard_max_bytes.parse::<u64>().unwrap();
    }
    fn parse_queue_discard_max_hw_bytes(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let discard_max_hw_bytes = read_to_string(blockdevice_dir.path().join("queue").join("discard_max_hw_bytes")).unwrap_or_else(|error| panic!("Error {} reading block device queue/discard_max_hw_bytes sysfs entry", error));
        blockdevice_data.queue_discard_max_hw_bytes = discard_max_hw_bytes.parse::<u64>().unwrap();
    }
    pub fn parse_queue_chunk_sectors(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let chunk_sectors = read_to_string(blockdevice_dir.path().join("queue").join("chunk_sectors"));
        blockdevice_data.queue_chunk_sectors = SysBlock::parse_read_and_conversion_into_option_u64(chunk_sectors);
    }
    pub fn parse_queue_nr_zones(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let nr_zones = read_to_string(blockdevice_dir.path().join("queue").join("nr_zones"));
        blockdevice_data.queue_nr_zones = SysBlock::parse_read_and_conversion_into_option_u64(nr_zones);
    }
    fn parse_read_and_conversion_into_option_u64(result: Result<String, Error>) -> Option<u64>
    {
        match result
        {
            Err(_) => None,
            Ok(value) => {
                match value.parse::<u64>()
                {
                    Err(_) => None,
                    Ok(number) => Some(number),
                }
            },
        }
    }
    fn parse_queue_zoned(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let zoned = read_to_string(blockdevice_dir.path().join("queue").join("zoned"));
        blockdevice_data.queue_chunk_sectors = match zoned
        {
            Err(_) => None,
            Ok(value) => {
                match value.parse::<u64>()
                {
                    Err(_) => None,
                    Ok(number) => Some(number),
                }
            },
        };
    }
    fn parse_queue_scheduler(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    )
    {
        let nr_requests = read_to_string(blockdevice_dir.path().join("queue").join("scheduler")).unwrap_or_else(|error| panic!("Error {} reading block device queue/scheduler sysfs entry", error));
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
    pub fn read_sys_block_devices(sys_block_path: &str) -> SysBlock
    {
        let mut sysblock = SysBlock::new();

        let blockdevice_directories = read_dir(sys_block_path).unwrap_or_else(|error| panic!("Error {} reading sysfs for block devices in path: {}.", error, sys_block_path));

        for blockdevice in blockdevice_directories
        {
            let mut blockdevice_data = BlockDevice::new();
            let directory_entry = blockdevice.unwrap_or_else(|error| panic!("Error {} reading block device sysfs entry", error));
            // device name
            let device_name = directory_entry.file_name().into_string().unwrap();
            blockdevice_data.device_name = device_name.to_string();

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
    use super::*;

    /*
    #[test]
    fn parse_proc_diskstats_line() {
        let diskstats_line = "   7       0 loop0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17";
        let result = ProcDiskStats::parse_proc_diskstats_line(&diskstats_line);
        assert_eq!(result, DiskStats { block_major: 7,
            block_minor: 0,
            device_name: "loop0".to_string(),
            reads_completed_success: 1,
            reads_merged: 2,
            reads_sectors: 3,
            reads_time_spent_ms: 4,
            writes_completed_success: 5,
            writes_merged: 6,
            writes_sectors: 7,
            writes_time_spent_ms: 8,
            ios_in_progress: 9,
            ios_time_spent_ms: 10,
            ios_weighted_time_spent_ms: 11,
            discards_completed_success: 12,
            discards_merged: 13,
            discards_sectors: 14,
            discards_time_spent_ms: 15,
            flush_requests_completed_success: 16,
            flush_requests_time_spent_ms: 17
        });
    }
    #[test]
    fn parse_proc_diskstats_line_before_linux_4_18() {
        let diskstats_line = "   7       0 loop0 1 2 3 4 5 6 7 8 9 10 11";
        let result = ProcDiskStats::parse_proc_diskstats_line(&diskstats_line);
        assert_eq!(result, DiskStats { block_major: 7,
            block_minor: 0,
            device_name: "loop0".to_string(),
            reads_completed_success: 1,
            reads_merged: 2,
            reads_sectors: 3,
            reads_time_spent_ms: 4,
            writes_completed_success: 5,
            writes_merged: 6,
            writes_sectors: 7,
            writes_time_spent_ms: 8,
            ios_in_progress: 9,
            ios_time_spent_ms: 10,
            ios_weighted_time_spent_ms: 11,
            discards_completed_success: 0,
            discards_merged: 0,
            discards_sectors: 0,
            discards_time_spent_ms: 0,
            flush_requests_completed_success: 0,
            flush_requests_time_spent_ms: 0
        });
    }

    #[test]
    fn parse_full_proc_diskstats_file() {
        let proc_diskstats = "   7       0 loop0 11 0 28 0 0 0 0 0 0 4 0 0 0 0 0 0 0
   7       1 loop1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       2 loop2 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       3 loop3 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       4 loop4 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       5 loop5 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       6 loop6 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       7 loop7 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 253       0 vda 13534 4237 1645451 3763 10172 10577 1730555 12701 0 23356 18881 7179 0 89620507 396 3929 2019
 253       1 vda1 13192 2675 1623109 3692 10151 10555 1730312 12688 0 23324 16775 7151 0 87803128 394 0 0
 253      15 vda15 136 1547 9919 20 1 0 1 0 0 52 21 1 0 186691 0 0 0
 259       0 vda16 159 15 10711 31 20 22 242 12 0 108 46 27 0 1630688 1 0 0
  11       0 sr0 291 0 75108 68 0 0 0 0 0 156 68 0 0 0 0 0 0";
        let result = ProcDiskStats::parse_proc_diskstats(proc_diskstats);
        assert_eq!(result, ProcDiskStats {
            disk_stats: vec![
                DiskStats { block_major: 7, block_minor: 0, device_name: "loop0".to_string(), reads_completed_success: 11, reads_merged: 0, reads_sectors: 28, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 4, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 1, device_name: "loop1".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 2, device_name: "loop2".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 3, device_name: "loop3".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 4, device_name: "loop4".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 5, device_name: "loop5".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 6, device_name: "loop6".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 7, device_name: "loop7".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 253, block_minor: 0, device_name: "vda".to_string(), reads_completed_success: 13534, reads_merged: 4237, reads_sectors: 1645451, reads_time_spent_ms: 3763, writes_completed_success: 10172, writes_merged: 10577, writes_sectors: 1730555, writes_time_spent_ms: 12701, ios_in_progress: 0, ios_time_spent_ms: 23356, ios_weighted_time_spent_ms: 18881, discards_completed_success: 7179, discards_merged: 0, discards_sectors: 89620507, discards_time_spent_ms: 396, flush_requests_completed_success: 3929, flush_requests_time_spent_ms: 2019 },
                DiskStats { block_major: 253, block_minor: 1, device_name: "vda1".to_string(), reads_completed_success: 13192, reads_merged: 2675, reads_sectors: 1623109, reads_time_spent_ms: 3692, writes_completed_success: 10151, writes_merged: 10555, writes_sectors: 1730312, writes_time_spent_ms: 12688, ios_in_progress: 0, ios_time_spent_ms: 23324, ios_weighted_time_spent_ms: 16775, discards_completed_success: 7151, discards_merged: 0, discards_sectors: 87803128, discards_time_spent_ms: 394, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 253, block_minor: 15, device_name: "vda15".to_string(), reads_completed_success: 136, reads_merged: 1547, reads_sectors: 9919, reads_time_spent_ms: 20, writes_completed_success: 1, writes_merged: 0, writes_sectors: 1, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 52, ios_weighted_time_spent_ms: 21, discards_completed_success: 1, discards_merged: 0, discards_sectors: 186691, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 259, block_minor: 0, device_name: "vda16".to_string(), reads_completed_success: 159, reads_merged: 15, reads_sectors: 10711, reads_time_spent_ms: 31, writes_completed_success: 20, writes_merged: 22, writes_sectors: 242, writes_time_spent_ms: 12, ios_in_progress: 0, ios_time_spent_ms: 108, ios_weighted_time_spent_ms: 46, discards_completed_success: 27, discards_merged: 0, discards_sectors: 1630688, discards_time_spent_ms: 1, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 11, block_minor: 0, device_name: "sr0".to_string(), reads_completed_success: 291, reads_merged: 0, reads_sectors: 75108, reads_time_spent_ms: 68, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 156, ios_weighted_time_spent_ms: 68, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 }
            ]
        });
    }
   */

    #[test]
    fn create_sys_block_device_stat_file_and_parse() {
        let stat_file_contents = "   13423     4331  1635716     4187    17196    13157  2515099    25033        0    48692    33644     7480        0 89783662      382     7436     4041";
        let dev_file_contents = "253:0";
        let max_hw_sectors_kb_file_contents = "2147483647";
        let max_sectors_kb_file_contents = "1280";
        let nr_requests_contents = "256";
        let scheduler = "[none] mq-deadline";
        let rotational = "1";
        let alignment_offset = "0";
        let cache_type = "write back";
        let inflight = "       1        2";
        let dax = "0";

        create_dir_all("/tmp/_sys/block/sda/queue").expect("Error creating mock sysfs directories.");
        write("/tmp/_sys/block/sda/stat", stat_file_contents).expect("error writing to mock sysfs stat file.");
        write("/tmp/_sys/block/sda/dev", dev_file_contents).expect("error writing to mock sysfs dev file.");
        write("/tmp/_sys/block/sda/queue/max_hw_sectors_kb", max_hw_sectors_kb_file_contents).expect("error writing to mock sysfs queue/max_hw_sectors_kb file.");
        write("/tmp/_sys/block/sda/queue/max_sectors_kb", max_sectors_kb_file_contents).expect("error writing to mock sysfs queue/max_sectors_kb file.");
        write("/tmp/_sys/block/sda/queue/nr_requests", nr_requests_contents).expect("error writing to mock sysfs queue/nr_requests file.");
        write("/tmp/_sys/block/sda/queue/scheduler", scheduler).expect("error writing to mock sysfs queue/scheduler file.");
        write("/tmp/_sys/block/sda/queue/rotational", rotational).expect("error writing to mock sysfs queue/rotational file.");
        write("/tmp/_sys/block/sda/alignment_offset", alignment_offset).expect("error writing to mock sysfs alignment_offset file.");
        write("/tmp/_sys/block/sda/cache_type", cache_type).expect("error writing to mock sysfs cache_type file.");
        write("/tmp/_sys/block/sda/inflight", inflight).expect("error writing to mock sysfs inflight file.");
        write("/tmp/_sys/block/sda/queue/dax", dax).expect("error writing to mock sysfs queue/dax file.");

        let result = Builder::new().path("/tmp/_sys/block").read();

        remove_dir_all("/tmp/_sys").unwrap();

        println!("{:?}", result);
    }
}