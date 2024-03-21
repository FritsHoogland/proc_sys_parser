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

let proc_net_dev = Builder::new().path("/myproc").read();
```

*/
use std::fs::read_to_string;
use regex::Regex;
use crate::ProcSysParserError;

/// Struct for holding `/proc/net/dev` statistics
#[derive(Debug, PartialEq, Default)]
pub struct ProcNetDev {
    pub interface: Vec<InterfaceStats>
}

/// Builder pattern for [`ProcNetDev`]
#[derive(Default)]
pub struct Builder {
    pub proc_path : String,
    pub proc_file : String,
    pub proc_filter : String,
}

impl Builder {
    pub fn new() -> Builder {
        Builder { 
            proc_path: "/proc".to_string(),
            proc_file: "net/dev".to_string(),
            proc_filter: "^lo".to_string(),
        }
    }

    pub fn path(mut self, proc_path: &str) -> Builder {
        self.proc_path = proc_path.to_string();
        self
    }
    pub fn file(mut self, proc_file: &str) -> Builder {
        self.proc_file = proc_file.to_string();
        self
    }
    pub fn filter(mut self, proc_filter: &str) -> Builder {
        self.proc_filter = proc_filter.to_string();
        self
    }
    pub fn read(self) -> Result<ProcNetDev, ProcSysParserError> {
        ProcNetDev::read_proc_net_dev(format!("{}/{}", &self.proc_path, &self.proc_file).as_str(), self.proc_filter.as_str())
    }
}

/// The main function for building a [`ProcNetDev`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> Result<ProcNetDev, ProcSysParserError> {
   Builder::new().read()
}

/// Struct for holding statistics of individual network interfaces
#[derive(Debug, PartialEq)]
pub struct InterfaceStats {
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
        filter: &str
    ) -> Result<ProcNetDev, ProcSysParserError> {
        let mut procnetdev = ProcNetDev::new();
        let filter_regex = Regex::new(filter)
            .map_err(|_| ProcSysParserError::RegexCompileError { regex: filter.to_string() })?;

        for line in proc_net_dev.lines() {
            match line {
                line if line.starts_with("Inter-|   Receive") => continue,
                line if line.starts_with(" face |bytes") => continue,
                line if !filter_regex.as_str().is_empty() && filter_regex.is_match(line.trim_start()) => continue,
                line => procnetdev.interface.push(ProcNetDev::parse_proc_net_dev_line(line)?),
            }
        }
        Ok(procnetdev)
    }
    fn parse_proc_net_dev_line(proc_net_dev_line: &str) -> Result<InterfaceStats, ProcSysParserError> {
        let mut fields = proc_net_dev_line.split_whitespace();

        Ok(InterfaceStats {
            name: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev name".to_string() })?
                .trim_end_matches(':')
                .to_string(),
            receive_bytes: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev receive_bytes".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            receive_packets: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev receive_packets".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            receive_errors: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev receive_errors".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            receive_drop: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev receive_drop".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            receive_fifo: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev receive_fifo".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            receive_frame: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev receive_frame".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            receive_compressed: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev receive_compressed".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            receive_multicast: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev receive_multicast".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            transmit_bytes: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev transmit_bytes".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            transmit_packets: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev transmit_packets".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            transmit_errors: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev transmit_errors".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            transmit_drop: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev transmit_drop".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            transmit_fifo: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev transmit_fifo".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            transmit_collisions: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev transmit_collisions".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            transmit_carrier: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev transmit_carrier".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
            transmit_compressed: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "net_dev transmit_compressed".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?,
        })
    }
    pub fn read_proc_net_dev(proc_net_dev_file: &str, proc_net_dev_filter: &str) -> Result<ProcNetDev, ProcSysParserError> {
        let proc_net_dev_output = read_to_string(proc_net_dev_file)
            .map_err(|error| ProcSysParserError::FileReadError { file: proc_net_dev_file.to_string(), error })?;
        ProcNetDev::parse_proc_net_dev(&proc_net_dev_output, proc_net_dev_filter)
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
        let result = ProcNetDev::parse_proc_net_dev_line(&netdev_line).unwrap();
        assert_eq!(result, InterfaceStats {
            name: "eth0".to_string(), receive_bytes: 151012532, receive_packets: 16720, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 816228, transmit_packets: 12257, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 }
        );
    }
    #[test]
    fn parse_proc_netdev_invalid_line() {
        let netdev_line = "Inter-|   Receive                                                |  Transmit";
        let result = ProcNetDev::parse_proc_net_dev(&netdev_line, "").unwrap();
        assert_eq!(result, ProcNetDev { interface: vec![] });
    }

    #[test]
    fn parse_full_proc_net_dev_file() {
        let proc_netdev = "Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
    lo:       0       0    0    0    0     0          0         0        0       0    0    0    0     0       0          0
  eth0: 151013652   16736    0    0    0     0          0         0   816228   12257    0    0    0     0       0          0";
        let result = ProcNetDev::parse_proc_net_dev(proc_netdev, "").unwrap();
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
        // please mind filter("") is used to remove the filter for "^lo", which would remove
        // lo/localhost. This also removes the 'loop' interfaces that are seen with docker.
        let result = Builder::new().filter("").path(&test_path).read().unwrap();
        remove_dir_all(test_path).unwrap();

        assert_eq!(result, ProcNetDev { interface:
        vec![InterfaceStats { name: "lo".to_string(), receive_bytes: 0, receive_packets: 0, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 0, transmit_packets: 0, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 },
             InterfaceStats { name: "eth0".to_string(), receive_bytes: 151013652, receive_packets: 16736, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 816228, transmit_packets: 12257, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 }
        ] } );
    }
}


