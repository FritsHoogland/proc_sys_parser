/*!
Read data from `/proc/diskstats` into the struct [`ProcDiskStats`].

The documentation for `/proc/diskstats` is found here: <https://www.kernel.org/doc/Documentation/iostats.txt>
And here: <https://www.kernel.org/doc/Documentation/ABI/testing/procfs-diskstats>.

Here is an example obtaining the data from `/proc/diskstats`:
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
            DiskStats { block_major: 11, block_minor: 0, device_name: "sr0", reads_completed_success: 291, reads_merged: 0, reads_sectors: 75108, reads_time_spent_ms: 68, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 156, ios_weighted_time_spent_ms: 68, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 }
    ]}
}
```
(edited for readability)

If you want to change the path and/or file that is read for [`ProcDiskStats`], which is `/proc/diskstats`
by default, use:
```no_run
use proc_sys_parser::{diskstats, diskstats::{ProcDiskStats, Builder}};

let proc_diskstats = Builder::new().file_name("/myproc/diskstats").read();
```

*/
use std::fs::read_to_string;

/// Struct for holding `/proc/diskstats` statistics
#[derive(Debug, PartialEq)]
pub struct ProcDiskStats {
    pub disk_stats: Vec<DiskStats>
}

/// Builder pattern for [`ProcDiskStats`]
pub struct Builder
{
    pub proc_diskstats_file: String
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
        Builder { proc_diskstats_file: "/proc/diskstats".to_string() }
    }

    pub fn file_name(mut self, proc_diskstats_file: &str) -> Builder
    {
        self.proc_diskstats_file = proc_diskstats_file.to_string();
        self
    }
    pub fn read(self) -> ProcDiskStats
    {
        ProcDiskStats::read_proc_diskstats(&self.proc_diskstats_file)
    }
}

/// The main function for building a [`ProcDiskStats`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> ProcDiskStats
{
   Builder::new().read()
}
impl Default for ProcDiskStats
{
    fn default() -> Self {
        Self::new()
    }
}

/// Struct for holding `/proc/diskstats` statistics
#[derive(Debug, PartialEq)]
pub struct DiskStats
{
    pub block_major: u64,
    pub block_minor: u64,
    pub device_name: String,
    pub reads_completed_success: u64,
    pub reads_merged: u64,
    pub reads_sectors: u64,
    pub reads_time_spent_ms: u64,
    pub writes_completed_success: u64,
    pub writes_merged: u64,
    pub writes_sectors: u64,
    pub writes_time_spent_ms: u64,
    pub ios_in_progress: u64,
    pub ios_time_spent_ms: u64,
    pub ios_weighted_time_spent_ms: u64,
    /// kernel 4.18+
    pub discards_completed_success: u64,
    /// kernel 4.18+
    pub discards_merged: u64,
    /// kernel 4.18+
    pub discards_sectors: u64,
    /// kernel 4.18+
    pub discards_time_spent_ms: u64,
    /// kernel 5.5+
    pub flush_requests_completed_success: u64,
    /// kernel 5.5+
    pub flush_requests_time_spent_ms: u64,
}

impl ProcDiskStats {
    pub fn new() -> ProcDiskStats {
        ProcDiskStats {
            disk_stats: vec![],
        }
    }
    pub fn parse_proc_diskstats(
        proc_diskstats: &str,
    ) -> ProcDiskStats
    {
        let mut procdiskstats = ProcDiskStats::new();
        for line in proc_diskstats.lines()
        {
            procdiskstats.disk_stats.push(ProcDiskStats::parse_proc_diskstats_line(line));
        }
        procdiskstats
    }
    fn parse_proc_diskstats_line(proc_diskstats_line: &str) -> DiskStats
    {
        let mut fields = proc_diskstats_line.split_whitespace();

        DiskStats {
            block_major: fields.next().unwrap().parse::<u64>().unwrap(),
            block_minor: fields.next().unwrap().parse::<u64>().unwrap(),
            device_name: fields.next().unwrap().to_string(),
            reads_completed_success: fields.next().unwrap().parse::<u64>().unwrap(),
            reads_merged: fields.next().unwrap().parse::<u64>().unwrap(),
            reads_sectors: fields.next().unwrap().parse::<u64>().unwrap(),
            reads_time_spent_ms: fields.next().unwrap().parse::<u64>().unwrap(),
            writes_completed_success: fields.next().unwrap().parse::<u64>().unwrap(),
            writes_merged: fields.next().unwrap().parse::<u64>().unwrap(),
            writes_sectors: fields.next().unwrap().parse::<u64>().unwrap(),
            writes_time_spent_ms: fields.next().unwrap().parse::<u64>().unwrap(),
            ios_in_progress: fields.next().unwrap().parse::<u64>().unwrap(),
            ios_time_spent_ms: fields.next().unwrap().parse::<u64>().unwrap(),
            ios_weighted_time_spent_ms: fields.next().unwrap().parse::<u64>().unwrap(),
            discards_completed_success: fields.next().unwrap_or("0").parse::<u64>().unwrap(),
            discards_merged: fields.next().unwrap_or("0").parse::<u64>().unwrap(),
            discards_sectors: fields.next().unwrap_or("0").parse::<u64>().unwrap(),
            discards_time_spent_ms: fields.next().unwrap_or("0").parse::<u64>().unwrap(),
            flush_requests_completed_success: fields.next().unwrap_or("0").parse::<u64>().unwrap(),
            flush_requests_time_spent_ms: fields.next().unwrap_or("0").parse::<u64>().unwrap(),
        }

    }
    pub fn read_proc_diskstats(proc_diskstats_file: &str) -> ProcDiskStats
    {
        let proc_diskstats_output = read_to_string(proc_diskstats_file).unwrap_or_else(|error|panic!("Error {} reading file: {}", error, proc_diskstats_file));
        ProcDiskStats::parse_proc_diskstats(&proc_diskstats_output)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{write, remove_file};
    use super::*;

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

    #[test]
    fn create_proc_diskstats_file_and_read() {
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
        write("/tmp/_test_proc_diskstats", proc_diskstats).expect("Error writing to /tmp/_test_proc_diskstats");
        let result = Builder::new().file_name("/tmp/_test_proc_diskstats").read();
        remove_file("/tmp/_test_proc_diskstats").unwrap();
        assert_eq!(result, ProcDiskStats { disk_stats: vec![
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
        ]});
    }
}

