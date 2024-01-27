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
ProcNetDev {
    interface: [
        InterfaceStats { name: "lo".to_string(), receive_bytes: 0, receive_packets: 0, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 0, transmit_packets: 0, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 },
        InterfaceStats { name: "eth0".to_string(), receive_bytes: 151013652, receive_packets: 16736, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 816228, transmit_packets: 12257, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 }
    ]
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
#[derive(Debug, PartialEq, Default)]
pub struct ProcNetDev {
    pub interface: Vec<InterfaceStats>
}

/// Builder pattern for [`ProcNetDev`]
#[derive(Default)]
pub struct Builder
{
    pub proc_path : String,
    pub proc_file : String,
}

impl Builder
{
    pub fn new() -> Builder
    {
        Builder { 
            proc_path: "/proc".to_string(),
            proc_file: "net/dev".to_string(),
        }
    }

    pub fn path(mut self, proc_path: &str) -> Builder
    {
        self.proc_path = proc_path.to_string();
        self
    }
    pub fn file(mut self, proc_file: &str) -> Builder
    {
        self.proc_file = proc_file.to_string();
        self
    }
    pub fn read(self) -> ProcNetDev
    {
        ProcNetDev::read_proc_net_dev(format!("{}/{}", &self.proc_path, &self.proc_file).as_str())
    }
}

/// The main function for building a [`ProcNetDev`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> ProcNetDev
{
   Builder::new().read()
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
        ProcNetDev::default()
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
                    procnetdev.interface.push(ProcNetDev::parse_proc_net_dev_line(line));
                }
            }
        }
        procnetdev
    }
    fn parse_proc_net_dev_line(proc_net_dev_line: &str) -> InterfaceStats
    {
        let mut fields = proc_net_dev_line.split_whitespace();

        InterfaceStats {
            name: fields.next().unwrap().trim_end_matches(':').to_string(),
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
    pub fn read_proc_net_dev(proc_net_dev_file: &str) -> ProcNetDev
    {
        let proc_net_dev_output = read_to_string(proc_net_dev_file).unwrap_or_else(|error| panic!("Error {} reading file: {}", error, proc_net_dev_file));
        ProcNetDev::parse_proc_net_dev(&proc_net_dev_output)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{write, remove_dir_all, create_dir_all};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use super::*;

    #[test]
    fn parse_proc_netdev_valid_line() {
        let netdev_line = "  eth0: 151012532   16720    0    0    0     0          0         0   816228   12257    0    0    0     0       0          0";
        let result = ProcNetDev::parse_proc_net_dev_line(&netdev_line);
        assert_eq!(result, InterfaceStats {
            name: "eth0".to_string(), receive_bytes: 151012532, receive_packets: 16720, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 816228, transmit_packets: 12257, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 }
        );
    }
    #[test]
    fn parse_proc_netdev_invalid_line() {
        let netdev_line = "Inter-|   Receive                                                |  Transmit";
        let result = ProcNetDev::parse_proc_net_dev(&netdev_line);
        assert_eq!(result, ProcNetDev { interface: vec![] });
    }

    #[test]
    fn parse_full_proc_net_dev_file() {
        let proc_netdev = "Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
    lo:       0       0    0    0    0     0          0         0        0       0    0    0    0     0       0          0
  eth0: 151013652   16736    0    0    0     0          0         0   816228   12257    0    0    0     0       0          0";
        let result = ProcNetDev::parse_proc_net_dev(proc_netdev);
        assert_eq!(result, ProcNetDev { interface:
        vec![InterfaceStats { name: "lo".to_string(), receive_bytes: 0, receive_packets: 0, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 0, transmit_packets: 0, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 },
             InterfaceStats { name: "eth0".to_string(), receive_bytes: 151013652, receive_packets: 16736, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 816228, transmit_packets: 12257, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 }
        ] } );
    }

    #[test]
    fn create_proc_net_dev_file_and_read() {
        let proc_netdev = "Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
    lo:       0       0    0    0    0     0          0         0        0       0    0    0    0     0       0          0
  eth0: 151013652   16736    0    0    0     0          0         0   816228   12257    0    0    0     0       0          0";

        let directory_suffix: String = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        create_dir_all(format!("{}/net", test_path)).expect("Error creating mock directory.");

        write(format!("{}/net/dev", test_path), proc_netdev).expect(format!("Error writing to {}/net/dev", test_path).as_str());
        let result = Builder::new().path(&test_path).read();
        remove_dir_all(test_path).unwrap();

        assert_eq!(result, ProcNetDev { interface:
        vec![InterfaceStats { name: "lo".to_string(), receive_bytes: 0, receive_packets: 0, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 0, transmit_packets: 0, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 },
             InterfaceStats { name: "eth0".to_string(), receive_bytes: 151013652, receive_packets: 16736, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 816228, transmit_packets: 12257, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 }
        ] } );
    }
}


