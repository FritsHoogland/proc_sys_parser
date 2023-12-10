/*!
Read data from `/proc/net/dev` into the struct [`ProcNetDev`].

The documentation for `/proc/net/dev` is found here: <https://www.kernel.org/doc/Documentation/filesystems/proc.txt>

Here is an example obtaining the data from `/proc/net/dev`:
```no_run
use proc_sys_parser::{net_dev, net_dev::ProcNetDev};

let proc_net_dev = net_dev::read();

println!("{:#?}", proc_net_dev);
```
Example output:
```text
}
```
(edited for readability)

If you want to change the path and/or file that is read for [`ProcNetDev`], which is `/proc/net/dev`
by default, use:
```no_run
use proc_sys_parser::{net_dev, net_dev::{ProcNetDev, Builder}};

let proc_net_dev = Builder::new().file_name("/myproc/net/dev").read();
```

*/
use std::fs::read_to_string;

/// Struct for holding `/proc/net/dev` statistics
#[derive(Debug, PartialEq)]
pub struct ProcNetDev {
    pub interface: Vec<InterfaceStats>
}

/// Builder pattern for [`ProcNetDev`]
pub struct Builder
{
    pub proc_net_dev_file: String
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
        Builder { proc_net_dev_file: "/proc/net/dev".to_string() }
    }

    pub fn file_name(mut self, proc_net_dev_file: &str) -> Builder
    {
        self.proc_net_dev_file = proc_net_dev_file.to_string();
        self
    }
    /*
    pub fn read(self) -> ProcNetDev
    {
        ProcNetDev::read_proc_net_dev(&self.proc_net_dev_file)
    }

     */
}

/// The main function for building a [`ProcNetDev`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
/*
pub fn read() -> ProcNetDev
{
   Builder::new().read()
}

 */
impl Default for ProcNetDev
{
    fn default() -> Self {
        Self::new()
    }
}

/// Struct for holding statistics of individual network interfaces
#[derive(Debug, PartialEq)]
pub struct InterfaceStats
{
    pub name: String,
    pub receive_bytes: u64,
    pub receive_packets: u64,
    pub receive_errors: u64,
    pub receive_drop: u64,
    pub receive_fifo: u64,
    pub receive_frame: u64,
    pub receive_compressed: u64,
    pub receive_multicast: u64,
    pub transmit_bytes: u64,
    pub transmit_packets: u64,
    pub transmit_errors: u64,
    pub transmit_drop: u64,
    pub transmit_fifo: u64,
    pub transmit_collisions: u64,
    pub transmit_carrier: u64,
    pub transmit_compressed: u64,
}

impl ProcNetDev {
    pub fn new() -> ProcNetDev {
        ProcNetDev {
            interface: vec![],
        }
    }
    pub fn parse_proc_net_dev(
        proc_net_dev: &str,
    ) -> ProcNetDev
    {
        let mut procnetdev = ProcNetDev::new();
        for line in proc_net_dev.lines()
        {
            match line
            {
                line if line.starts_with("Inter-|   Receive") => continue,
                line if line.starts_with(" face |bytes") => continue,
                line => {
                    println!("{}", line);
                }

            }
        }
        procnetdev
    }
    fn parse_proc_net_dev_line(proc_net_dev_line: &str) -> InterfaceStats
    {
        let mut fields = proc_net_dev_line.split_whitespace();

        InterfaceStats {
            name: fields.next().unwrap().to_string(),
            receive_bytes: fields.next().unwrap().parse::<u64>().unwrap(),
            receive_packets: fields.next().unwrap().parse::<u64>().unwrap(),
            receive_errors: fields.next().unwrap().parse::<u64>().unwrap(),
            receive_drop: fields.next().unwrap().parse::<u64>().unwrap(),
            receive_fifo: fields.next().unwrap().parse::<u64>().unwrap(),
            receive_frame: fields.next().unwrap().parse::<u64>().unwrap(),
            receive_compressed: fields.next().unwrap().parse::<u64>().unwrap(),
            receive_multicast: fields.next().unwrap().parse::<u64>().unwrap(),
            transmit_bytes: fields.next().unwrap().parse::<u64>().unwrap(),
            transmit_packets: fields.next().unwrap().parse::<u64>().unwrap(),
            transmit_errors: fields.next().unwrap().parse::<u64>().unwrap(),
            transmit_drop: fields.next().unwrap().parse::<u64>().unwrap(),
            transmit_fifo: fields.next().unwrap().parse::<u64>().unwrap(),
            transmit_collisions: fields.next().unwrap().parse::<u64>().unwrap(),
            transmit_carrier: fields.next().unwrap().parse::<u64>().unwrap(),
            transmit_compressed: fields.next().unwrap().parse::<u64>().unwrap(),
        }

    }
    /*
    pub fn read_proc_diskstats(proc_diskstats_file: &str) -> ProcDiskStats
    {
        let proc_diskstats_output = read_to_string(proc_diskstats_file).unwrap_or_else(|error|panic!("Error {} reading file: {}", error, proc_diskstats_file));
        ProcDiskStats::parse_proc_diskstats(&proc_diskstats_output)
    }

     */
}

#[cfg(test)]
mod tests {
    use std::fs::{write, remove_file};
    use super::*;

    #[test]
    fn parse_proc_netdev_invalid_line() {
        let netdev_line = "  eth0: 151012532   16720    0    0    0     0          0         0   816228   12257    0    0    0     0       0          0";
        let result = ProcNetDev::parse_proc_net_dev_line(&netdev_line);
        assert_eq!(result, InterfaceStats {
            name: "eth0:".to_string(), receive_bytes: 151012532, receive_packets: 16720, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 816228, transmit_packets: 12257, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 }
        );
    }
    /*
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

     */
}


