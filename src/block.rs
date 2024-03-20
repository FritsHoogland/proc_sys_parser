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
use std::fs::{read_to_string, read_dir, DirEntry};
use regex::Regex;
use crate::ProcSysParserError;

/// Struct for holding `/sys/block` block device statistics and information
#[derive(Debug, PartialEq, Default)]
pub struct SysBlock {
    pub block_devices: Vec<BlockDevice>
}

/// Builder pattern for [`SysBlock`]
#[derive(Default)]
pub struct Builder {
    pub sys_path : String,
    pub filter : String,
}

impl Builder {
    pub fn new() -> Builder {
        Builder { 
            sys_path: "/sys".to_string(), 
            filter: "^dm-".to_string(),
        }
    }
    pub fn path(mut self, sys_path: &str) -> Builder {
        self.sys_path = sys_path.to_string();
        self
    }
    pub fn regex(mut self, filter: &str) -> Builder {
        self.filter = filter.to_string();
        self
    }
    pub fn read(self) -> Result<SysBlock, ProcSysParserError> {
        SysBlock::read_sys_block_devices(format!("{}/block", self.sys_path).as_str(), self.filter.as_str())
    }
}

/// The main function for building a [`SysBlock`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> Result<SysBlock, ProcSysParserError> {
   Builder::new().read()
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

impl BlockDevice {
    pub fn new() -> BlockDevice {
        BlockDevice::default()
    }
}

impl SysBlock {
    pub fn new() -> SysBlock {
        SysBlock::default() 
    }
    fn parse_dev(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    ) -> Result<(), ProcSysParserError> {
        //let dev_contents = read_to_string(blockdevice_dir.path().join("dev")).unwrap_or_else(|error| panic!("Error {} reading block device dev sysfs entry", error)).trim_end_matches('\n').to_string();
        let dev_contents = read_to_string(blockdevice_dir.path().join("dev"))
            .map_err(|error| ProcSysParserError::FileReadError { file: blockdevice_dir.path().join("dev").to_string_lossy().to_string(), error})?
            .trim_end_matches('\n').to_string();
        let mut fields = dev_contents.split(':');
        blockdevice_data.dev_block_major = fields.next().ok_or(ProcSysParserError::IteratorItemError { item: "block parse_dev major".to_string() })?
                        .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.dev_block_minor = fields.next().ok_or(ProcSysParserError::IteratorItemError { item: "block parse_dev minor".to_string() })?
                        .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        Ok(())
    }
    fn parse_inflight(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    ) -> Result<(), ProcSysParserError> {
        let inflight_from_file = read_to_string(blockdevice_dir.path().join("inflight"))
            .map_err(|error| ProcSysParserError::FileReadError { file: blockdevice_dir.path().join("inflight").to_string_lossy().to_string(), error})?
            .trim_end_matches('\n').to_string();
        blockdevice_data.inflight_reads = inflight_from_file.split_whitespace().nth(0).ok_or(ProcSysParserError::IteratorItemError { item: "block parse_inflight reads".to_string() })?
                        .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.inflight_writes = inflight_from_file.split_whitespace().nth(1).ok_or(ProcSysParserError::IteratorItemError { item: "block parse_inflight writes".to_string() })?
                        .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        Ok(())
    }
    fn parse_queue_scheduler(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    ) -> Result<(), ProcSysParserError> {
        let nr_requests = read_to_string(blockdevice_dir.path().join("queue").join("scheduler"))
            .map_err(|error| ProcSysParserError::FileReadError { file: blockdevice_dir.path().join("queue").join("scheduler").to_string_lossy().to_string(), error })?
            .trim_end_matches('\n').to_string();
        let left_bracket = nr_requests.find('[');
        let right_bracket = nr_requests.find(']');

        if left_bracket.is_some() && right_bracket.is_some() {
            blockdevice_data.queue_scheduler = nr_requests[left_bracket.ok_or(ProcSysParserError::FindItemError { item: "block parse_queue_scheduler '['".to_string() })?+1..right_bracket.ok_or(ProcSysParserError::FindItemError { item: "block parse_queue_scheduler ']'".to_string() })?].to_string();
        } else {
            blockdevice_data.queue_scheduler = "?".to_string();
        }
        Ok(())
    }
    fn parse_stat(
        blockdevice_data: &mut BlockDevice,
        blockdevice_dir: &DirEntry,
    ) -> Result<(), ProcSysParserError> {
        let parse_next_and_conversion_into_option_u64 = |result: Option<&str>| -> Option<u64> {
            match result {
                None => None,
                Some(value) => {
                    match value.parse::<u64>() {
                        Err(_) => None,
                        Ok(number) => Some(number),
                    }
                },
            }
        };

        let stat_contents = read_to_string(blockdevice_dir.path().join("stat"))
            .map_err(|error| ProcSysParserError::FileReadError { file: blockdevice_dir.path().join("stat").to_string_lossy().to_string(), error })?
            .trim_end_matches('\n')
            .to_string();
        let mut stat_contents_splitted = stat_contents
            .split_whitespace();

        blockdevice_data.stat_reads_completed_success = stat_contents_splitted.next()
            .ok_or(ProcSysParserError::FindItemError { item: "block parse_stat reads_completed_success".to_string() })?
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.stat_reads_merged = stat_contents_splitted.next()
            .ok_or(ProcSysParserError::FindItemError { item: "block parse_stat reads_merged".to_string() })?
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.stat_reads_sectors = stat_contents_splitted.next()
            .ok_or(ProcSysParserError::FindItemError { item: "block parse_stat reads_sectors".to_string() })?
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.stat_reads_time_spent_ms = stat_contents_splitted.next()
            .ok_or(ProcSysParserError::FindItemError { item: "block parse_stat reads_time_spent_ms".to_string() })?
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.stat_writes_completed_success = stat_contents_splitted.next()
            .ok_or(ProcSysParserError::FindItemError { item: "block parse_stat writes_completed_success".to_string() })?
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.stat_writes_merged = stat_contents_splitted.next()
            .ok_or(ProcSysParserError::FindItemError { item: "block parse_stat writes_completed_success".to_string() })?
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.stat_writes_sectors = stat_contents_splitted.next()
            .ok_or(ProcSysParserError::FindItemError { item: "block parse_stat writes_sectors".to_string() })?
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.stat_writes_time_spent_ms = stat_contents_splitted.next()
            .ok_or(ProcSysParserError::FindItemError { item: "block parse_stat writes_time_spent_ms".to_string() })?
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.stat_ios_in_progress = stat_contents_splitted.next()
            .ok_or(ProcSysParserError::FindItemError { item: "block parse_stat ios_in_progress".to_string() })?
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.stat_ios_time_spent_ms = stat_contents_splitted.next()
            .ok_or(ProcSysParserError::FindItemError { item: "block parse_stat ios_time_spent_ms".to_string() })?
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.stat_ios_weighted_time_spent_ms = stat_contents_splitted.next()
            .ok_or(ProcSysParserError::FindItemError { item: "block parse_stat ios_weighted_time_spent_ms".to_string() })?
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?;
        blockdevice_data.stat_discards_completed_success = parse_next_and_conversion_into_option_u64(stat_contents_splitted.next());
        blockdevice_data.stat_discards_merged = parse_next_and_conversion_into_option_u64(stat_contents_splitted.next());
        blockdevice_data.stat_discards_sectors = parse_next_and_conversion_into_option_u64(stat_contents_splitted.next());
        blockdevice_data.stat_discards_time_spent_ms = parse_next_and_conversion_into_option_u64(stat_contents_splitted.next());
        blockdevice_data.stat_flush_requests_completed_success = parse_next_and_conversion_into_option_u64(stat_contents_splitted.next());
        blockdevice_data.stat_flush_requests_time_spent_ms = parse_next_and_conversion_into_option_u64(stat_contents_splitted.next());
        Ok(())
    }
    fn parse_contents_file_u64(
        file: &str,
        blockdevice_dir: &DirEntry,
    ) -> Result<u64, ProcSysParserError> {
        Ok(
            read_to_string(blockdevice_dir.path().join(file))
            .map_err(|error| ProcSysParserError::FileReadError { file: blockdevice_dir.path().join(file).to_string_lossy().to_string(), error })?
            .trim_end_matches('\n')
            .to_string()
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?
        )
    }
    fn parse_contents_file_i64(
        file: &str,
        blockdevice_dir: &DirEntry,
    ) -> Result<i64, ProcSysParserError> {
        Ok(
            read_to_string(blockdevice_dir.path().join(file))
            .map_err(|error| ProcSysParserError::FileReadError { file: blockdevice_dir.path().join(file).to_string_lossy().to_string(), error })?
            .trim_end_matches('\n')
            .to_string()
            .parse::<i64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?
        )
    }
    fn parse_contents_file_option_u64(
        file: &str,
        blockdevice_dir: &DirEntry,
    ) -> Result<Option<u64>, ProcSysParserError>
    {
        match read_to_string(blockdevice_dir.path().join(file)) {
            Ok(result) => {
                Ok(
                    Some(result
                    .trim_end_matches('\n')
                    .to_string()
                    .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?)
                )
            },
            Err(_) => Ok(None),
        }
    }
    fn parse_contents_file_option_string(
        file: &str,
        blockdevice_dir: &DirEntry,
    ) -> Result<Option<String>, ProcSysParserError> {
        Ok(match read_to_string(blockdevice_dir.path().join(file)) {
            Ok(result) => Some(result.trim_end_matches('\n').to_string()),
            Err(_) => None
        })
    }
    fn parse_contents_file_string(
        file: &str,
        blockdevice_dir: &DirEntry,
    ) -> Result <String, ProcSysParserError> {
        Ok(read_to_string(blockdevice_dir.path().join(file))
            .map_err(|error| ProcSysParserError::FileReadError { file: blockdevice_dir.path().join(file).to_string_lossy().to_string(), error })?
            .trim_end_matches('\n')
            .to_string())
    }
    pub fn read_sys_block_devices(
        sys_block_path: &str,
        filter: &str,
    ) -> Result<SysBlock, ProcSysParserError> {
        let mut sysblock = SysBlock::new();

        let blockdevice_directories = read_dir(sys_block_path)
            .map_err(|error| ProcSysParserError::DirectoryReadError { directory: sys_block_path.to_string(), error })?;
        let filter_regex = Regex::new(filter)
            .map_err(|error| ProcSysParserError::RegexCompileError { regex: filter.to_string() })?;

        for blockdevice in blockdevice_directories {
            let directory_entry = blockdevice.unwrap_or_else(|error| panic!("Error {} reading block device sysfs entry", error));
            // apply filter
            if !filter_regex.as_str().is_empty() && filter_regex.is_match(&directory_entry.path().to_string_lossy().to_string()) { continue };

            let mut blockdevice_data = BlockDevice::new();

            blockdevice_data.device_name = directory_entry.file_name().into_string().unwrap();
            blockdevice_data.alignment_offset = SysBlock::parse_contents_file_u64("alignment_offset", &directory_entry)?;
            blockdevice_data.cache_type = SysBlock::parse_contents_file_option_string("cache_type", &directory_entry)?;
            SysBlock::parse_dev(&mut blockdevice_data, &directory_entry)?;
            blockdevice_data.discard_alignment = SysBlock::parse_contents_file_u64("discard_alignment", &directory_entry)?;
            blockdevice_data.diskseq = SysBlock::parse_contents_file_option_u64("diskseq", &directory_entry)?;
            blockdevice_data.hidden = SysBlock::parse_contents_file_u64("hidden", &directory_entry)?;
            SysBlock::parse_inflight(&mut blockdevice_data, &directory_entry)?;
            blockdevice_data.queue_add_random = SysBlock::parse_contents_file_u64("queue/add_random", &directory_entry)?;
            blockdevice_data.queue_chunk_sectors = SysBlock::parse_contents_file_option_u64("queue/chunk_sectors", &directory_entry)?;
            blockdevice_data.queue_dax = SysBlock::parse_contents_file_u64("queue/dax", &directory_entry)?;
            blockdevice_data.queue_discard_granularity = SysBlock::parse_contents_file_u64("queue/discard_granularity", &directory_entry)?;
            blockdevice_data.queue_discard_max_bytes = SysBlock::parse_contents_file_u64("queue/discard_max_bytes", &directory_entry)?;
            blockdevice_data.queue_discard_max_hw_bytes = SysBlock::parse_contents_file_u64("queue/discard_max_hw_bytes", &directory_entry)?;
            blockdevice_data.queue_hw_sector_size = SysBlock::parse_contents_file_u64("queue/hw_sector_size", &directory_entry)?;
            blockdevice_data.queue_io_poll = SysBlock::parse_contents_file_u64("queue/io_poll", &directory_entry)?;
            blockdevice_data.queue_io_poll_delay = SysBlock::parse_contents_file_i64("queue/io_poll_delay", &directory_entry)?;
            blockdevice_data.queue_logical_block_size = SysBlock::parse_contents_file_u64("queue/logical_block_size", &directory_entry)?;
            blockdevice_data.queue_max_discard_segments = SysBlock::parse_contents_file_u64("queue/max_discard_segments", &directory_entry)?;
            blockdevice_data.queue_max_hw_sectors_kb = SysBlock::parse_contents_file_u64("queue/max_hw_sectors_kb", &directory_entry)?;
            blockdevice_data.queue_max_integrity_segments = SysBlock::parse_contents_file_u64("queue/max_integrity_segments", &directory_entry)?;
            blockdevice_data.queue_max_sectors_kb = SysBlock::parse_contents_file_u64("queue/max_sectors_kb", &directory_entry)?;
            blockdevice_data.queue_max_segment_size = SysBlock::parse_contents_file_u64("queue/max_segment_size", &directory_entry)?;
            blockdevice_data.queue_max_segments = SysBlock::parse_contents_file_u64("queue/max_segments", &directory_entry)?;
            blockdevice_data.queue_minimum_io_size = SysBlock::parse_contents_file_u64("queue/minimum_io_size", &directory_entry)?;
            blockdevice_data.queue_nomerges = SysBlock::parse_contents_file_u64("queue/nomerges", &directory_entry)?;
            blockdevice_data.queue_nr_requests = SysBlock::parse_contents_file_u64("queue/nr_requests", &directory_entry)?;
            blockdevice_data.queue_nr_zones = SysBlock::parse_contents_file_option_u64("queue/nr_zones", &directory_entry)?;
            blockdevice_data.queue_optimal_io_size = SysBlock::parse_contents_file_u64("queue/optimal_io_size", &directory_entry)?;
            blockdevice_data.queue_physical_block_size = SysBlock::parse_contents_file_u64("queue/physical_block_size", &directory_entry)?;
            blockdevice_data.queue_read_ahead_kb = SysBlock::parse_contents_file_u64("queue/read_ahead_kb", &directory_entry)?;
            blockdevice_data.queue_rotational = SysBlock::parse_contents_file_u64("queue/rotational", &directory_entry)?;
            blockdevice_data.queue_rq_affinity = SysBlock::parse_contents_file_u64("queue/rq_affinity", &directory_entry)?;
            SysBlock::parse_queue_scheduler(&mut blockdevice_data, &directory_entry)?;
            blockdevice_data.queue_write_cache = SysBlock::parse_contents_file_string("queue/write_cache", &directory_entry)?;
            blockdevice_data.queue_write_same_max_bytes = SysBlock::parse_contents_file_u64("queue/write_same_max_bytes", &directory_entry)?;
            blockdevice_data.queue_zoned = SysBlock::parse_contents_file_option_string("queue/zoned", &directory_entry)?;
            blockdevice_data.range = SysBlock::parse_contents_file_u64("range", &directory_entry)?;
            blockdevice_data.removable = SysBlock::parse_contents_file_u64("removable", &directory_entry)?;
            blockdevice_data.ro = SysBlock::parse_contents_file_u64("ro", &directory_entry)?;
            blockdevice_data.size = SysBlock::parse_contents_file_u64("size", &directory_entry)?;

            SysBlock::parse_stat(&mut blockdevice_data, &directory_entry)?;

            sysblock.block_devices.push(blockdevice_data);
        }

        Ok(sysblock)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{write, remove_dir_all, create_dir_all};
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    use super::*;

    #[test]
    fn create_sys_block_device_parse_files() {
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

        let result = Builder::new().path(&test_path).read().unwrap();

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
    fn create_sys_block_device_parse_files_non_existent() {
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

        let result = Builder::new().path(&test_path).read().unwrap();

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
