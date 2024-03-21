/*!
Read data from `/proc/schedstat` into the struct [`ProcSchedStat`].

The documentation for `/proc/schedstat` is found here: <https://www.kernel.org/doc/Documentation/scheduler/sched-stats.txt>.
This says the cpu scheduler statistics are in jiffies. THIS IS INCORRECT since kernel version 2.6.23/commit 425e0968a25f.
(<https://github.com/torvalds/linux/commit/425e0968a25f>)

The cpu scheduler statistics are now in nanoseconds.

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

If you want to change the path and/or file that is read for [`ProcSchedStat`], which is `/proc/schedstat`
by default, use:
```no_run
use proc_sys_parser::{schedstat, schedstat::{ProcSchedStat, Builder}};

let proc_schedstat = Builder::new().path("/myproc").read();
```

# cpu vector numbers
A `/proc/schedstat` cpu line might look like this:
```text
cpu0 0 0 0 0 0 0 4017837133 477882075 26299
```
Such a line will be transformed to the following (`Vec<u64>`) vector in ProcSchedStat.cpu:
```text
[0, 0, 0, 0, 0, 0, 0, 40178371330, 4778820750, 26299]
```
The first number is the cpu number (`cpu0`), the other numbers for cpu statistics follow.
This has the consequence that the `CPU statistics` field numbers in the description from kernel.org
now have to be increased by one to find the statistic in the vector.

Also mind that:
- The time running on cpu in jiffies, statistic number 7 in the description at kernel.org is changed
  to time in milliseconds in the 8th position in the vector.
- The time waiting to run in jiffies, statistic number 8 in the description at kernel.org is changed
  to time in milliseconds in the 9th position in the vector.

# domain vector numbers
A `/proc/schedstat` domain line might look like this:
```text
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
```
Such a line will be transformed to the following (`Vec<u64>`) vector in ProcSchedStat.domain:
```text
[0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```
The first number is the domain number (`domain0`).
The second number is the decimal representation of the hexadecimal `3f` number representing the cpu mask.
Starting from the third number, the statistics that are explained with the `Domain statistics` are available.
This has the consequence that the field numbers in the kernel.org explanation have to be increased by two
to match the statistic in the vector.

*/
use std::fs::read_to_string;
use crate::ProcSysParserError;
use log::warn;

/// Builder pattern for [`ProcSchedStat`]
#[derive(Default)]
pub struct Builder {
    pub proc_path : String,
    pub proc_file : String,
}

impl Builder {
    pub fn new() -> Builder {
        Builder { 
            proc_path: "/proc".to_string(),
            proc_file: "schedstat".to_string(),
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
    pub fn read(self) -> Result<ProcSchedStat, ProcSysParserError> {
        ProcSchedStat::read_proc_schedstat(format!("{}/{}", &self.proc_path, &self.proc_file).as_str())
    }
}

/// The main function for building a [`ProcSchedStat`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> Result<ProcSchedStat, ProcSysParserError> {
    Builder::new().read()
}

/// Struct for holding `/proc/schedstat` statistics
#[derive(Debug, PartialEq, Default)]
pub struct ProcSchedStat {
    pub version: u64,
    pub timestamp: u64,
    pub cpu: Vec<Vec<u64>>,
    pub domain: Vec<Domain>,
}

#[derive(Debug, PartialEq, Default)]
pub struct Domain {
    pub cpu_nr: u64,
    pub domain_nr: u64,
    pub cpu_masks: Vec<u64>,
    pub statistics: Vec<u64>,
}

impl ProcSchedStat {
    pub fn new() -> ProcSchedStat {
        ProcSchedStat::default() 
    }
    pub fn parse_proc_schedstat_output(proc_schedstat: &str) -> Result<ProcSchedStat, ProcSysParserError> {
        let mut schedstat = ProcSchedStat::new();
        // current_cpu keeps the current cpu number.
        // this is used for populating the domain struct belonging to each cpu
        let mut current_cpu = &0;
        for line in proc_schedstat.lines() {
            match line {
                line if line.starts_with("version ") => {
                    schedstat.version = ProcSchedStat::generate_number_unsigned(line)?;
                },
                line if line.starts_with("timestamp ") => {
                    schedstat.timestamp = ProcSchedStat::generate_number_unsigned(line)?;
                },
                line if line.starts_with("cpu") => {
                    schedstat.cpu.push(ProcSchedStat::generate_number_vector(line)?);
                    current_cpu = schedstat.cpu.last()
                        .unwrap()
                        .iter()
                        .next()
                        .unwrap();
                },
                line if line.starts_with("domain") => {
                    schedstat.domain.push(ProcSchedStat::generate_domain_struct(line, current_cpu)?);
                },
                _  => warn!("schedstat: unknown entry found: {}", line),
            }
        }
        Ok(schedstat)
    }
    fn generate_number_vector(proc_schedstat_line: &str) -> Result<Vec<u64>, ProcSysParserError> {
        let proc_schedstat_line = match proc_schedstat_line {
            line if line.starts_with("cpu") => {
                line.split_whitespace()
                    .map(|cpu| if cpu.starts_with("cpu") { cpu.strip_prefix("cpu").unwrap() } else { cpu } )
                    .map(|row| row.parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?
            },
            line => {
                warn!("Unknown entry found: {}", line);
                Vec::new()
            },
        };
        Ok(proc_schedstat_line)
    }
    fn generate_domain_struct(proc_schedstat_line: &str, current_cpu: &u64) -> Result<Domain, ProcSysParserError> {
        
        /*
        let domain_nr = proc_schedstat_line
            .split_whitespace()
            .map(|line| line.strip_prefix("domain").unwrap_or_default())
            .take(1)
            .map(|cpu_nr| cpu_nr.parse::<u64>().unwrap())
            .next()
            .unwrap();
        */
        let domain_nr = proc_schedstat_line
            .split_whitespace()
            .map(|line| line.strip_prefix("domain").unwrap_or_default())
            .next()
            .ok_or(ProcSysParserError::IteratorItemError {item: "schedstat generate_domain_struct domain_nr".to_string() })?
            .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)?;

        /*
        let cpu_masks: Vec<u64> = proc_schedstat_line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|cpu_mask| u64::from_str_radix(cpu_mask, 16).unwrap())
            .collect();
        */
        let cpu_masks: Vec<u64> = proc_schedstat_line
            .split_whitespace()
            .nth(1)
            .ok_or(ProcSysParserError::IteratorItemError {item: "schedstat generate_domain_struct cpu_masks".to_string() })?
            .split(',')
            .map(|cpu_mask| u64::from_str_radix(cpu_mask, 16).map_err(ProcSysParserError::ParseToIntegerError))
            .collect::<Vec<_>>()
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        /*
        let statistics: Vec<u64> = proc_schedstat_line
            .split_whitespace()
            .skip(2)
            .map(|statistics| statistics.parse::<u64>().unwrap())
            .collect();
        */
        let statistics: Vec<u64> = proc_schedstat_line
            .split_whitespace()
            .skip(2)
            .map(|statistics| statistics.parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError))
            .collect::<Vec<_>>()
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;


        Ok(Domain {
            cpu_nr: *current_cpu,
            domain_nr,
            cpu_masks,
            statistics,
        })
    }
    fn generate_number_unsigned(proc_stat_line: &str) -> Result<u64, ProcSysParserError> {
        proc_stat_line.split_whitespace()
            .nth(1)
            .ok_or(ProcSysParserError::IteratorItemError {item: "schedstat generate_number_unsigned".to_string() })?
            .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)
    }
    pub fn read_proc_schedstat(proc_schedstat_file: &str) -> Result<ProcSchedStat, ProcSysParserError> {
        let proc_schedstat_output = read_to_string(proc_schedstat_file)
            .map_err(|error| ProcSysParserError::FileReadError { file: proc_schedstat_file.to_string(), error })?;
        ProcSchedStat::parse_proc_schedstat_output(&proc_schedstat_output)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{write, remove_dir_all, create_dir_all};
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    use super::*;

    // cpu times are in jiffies, which are clock ticks.
    // clock ticks are defined in the getconf value CLK_TCK.
    // this crate dynamically obtains the CLK_TCK value.
    // the common value of CLK_TCK is 100, which is a hard assumption here.
    #[test]
    fn parse_version_line() {
        let version_line = "version 15";
        let result = ProcSchedStat::generate_number_unsigned(&version_line).unwrap();
        assert_eq!(result, 15);
    }
    #[test]
    fn parse_timestamp_line() {
        let timestamp_line = "timestamp 4318766637";
        let result = ProcSchedStat::generate_number_unsigned(&timestamp_line).unwrap();
        assert_eq!(result, 4318766637);
    }

    #[test]
    fn parse_cpu_line() {
        let cpu_line = "cpu0 0 0 0 0 0 0 455307306435 48519572891 4320349";
        let result = ProcSchedStat::generate_number_vector(&cpu_line).unwrap();
        assert_eq!(result, vec![0, 0, 0, 0, 0, 0, 0, 455307306435, 48519572891, 4320349]);
    }

    #[test]
    fn parse_domain_line() {
        let domain_line = "domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0";
        let result = ProcSchedStat::generate_number_vector(&domain_line).unwrap();
        assert_eq!(result, vec![]);
    }
    #[test]
    fn parse_full_proc_schedstat_file() {
        let proc_schedstat = "version 15
timestamp 4318961659
cpu0 0 0 0 0 0 0 457571901633 48594074614 4348645
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
cpu1 0 0 0 0 0 0 435206433012 44944145715 3928368
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
cpu2 0 0 0 0 0 0 429637514081 43591673257 3833297
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
cpu3 0 0 0 0 0 0 445308389036 43102743982 3851418
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
cpu4 0 0 0 0 0 0 438666554521 43706845278 3787400
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
cpu5 0 0 0 0 0 0 444708323872 42862371788 3900565
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0";
        let result = ProcSchedStat::parse_proc_schedstat_output(proc_schedstat).unwrap();
        assert_eq!(result, ProcSchedStat { version: 15,
            timestamp: 4318961659,
            cpu: vec![
                   vec![0, 0, 0, 0, 0, 0, 0, 457571901633, 48594074614, 4348645],
                   vec![1, 0, 0, 0, 0, 0, 0, 435206433012, 44944145715, 3928368],
                   vec![2, 0, 0, 0, 0, 0, 0, 429637514081, 43591673257, 3833297],
                   vec![3, 0, 0, 0, 0, 0, 0, 445308389036, 43102743982, 3851418],
                   vec![4, 0, 0, 0, 0, 0, 0, 438666554521, 43706845278, 3787400],
                   vec![5, 0, 0, 0, 0, 0, 0, 444708323872, 42862371788, 3900565],
            ],
            domain: vec![
                Domain { cpu_nr: 0, domain_nr: 0, cpu_masks: vec![63], statistics: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, 
                Domain { cpu_nr: 1, domain_nr: 0, cpu_masks: vec![63], statistics: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, 
                Domain { cpu_nr: 2, domain_nr: 0, cpu_masks: vec![63], statistics: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, 
                Domain { cpu_nr: 3, domain_nr: 0, cpu_masks: vec![63], statistics: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, 
                Domain { cpu_nr: 4, domain_nr: 0, cpu_masks: vec![63], statistics: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, 
                Domain { cpu_nr: 5, domain_nr: 0, cpu_masks: vec![63], statistics: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
            ] 
        });
    }
    #[test]
    fn create_proc_schedstat_file_and_read() {
        let proc_schedstat = "version 15
timestamp 4318961659
cpu0 0 0 0 0 0 0 457571901633 48594074614 4348645
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
cpu1 0 0 0 0 0 0 435206433012 44944145715 3928368
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
cpu2 0 0 0 0 0 0 429637514081 43591673257 3833297
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
cpu3 0 0 0 0 0 0 445308389036 43102743982 3851418
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
cpu4 0 0 0 0 0 0 438666554521 43706845278 3787400
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
cpu5 0 0 0 0 0 0 444708323872 42862371788 3900565
domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0";
        let directory_suffix: String = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        create_dir_all(format!("{}/proc", test_path)).expect("Error creating mock directory.");

        write(format!("{}/schedstat", test_path), proc_schedstat).expect(format!("Error writing to {}/schedstat", test_path).as_str());

        let result = Builder::new().path(&test_path).read().unwrap();
        remove_dir_all(test_path).unwrap();

        assert_eq!(result, ProcSchedStat { version: 15,
            timestamp: 4318961659,
            cpu: vec![
                vec![0, 0, 0, 0, 0, 0, 0, 457571901633, 48594074614, 4348645],
                vec![1, 0, 0, 0, 0, 0, 0, 435206433012, 44944145715, 3928368],
                vec![2, 0, 0, 0, 0, 0, 0, 429637514081, 43591673257, 3833297],
                vec![3, 0, 0, 0, 0, 0, 0, 445308389036, 43102743982, 3851418],
                vec![4, 0, 0, 0, 0, 0, 0, 438666554521, 43706845278, 3787400],
                vec![5, 0, 0, 0, 0, 0, 0, 444708323872, 42862371788, 3900565],
            ],
            domain: vec![
                Domain { cpu_nr: 0, domain_nr: 0, cpu_masks: vec![63], statistics: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, 
                Domain { cpu_nr: 1, domain_nr: 0, cpu_masks: vec![63], statistics: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, 
                Domain { cpu_nr: 2, domain_nr: 0, cpu_masks: vec![63], statistics: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, 
                Domain { cpu_nr: 3, domain_nr: 0, cpu_masks: vec![63], statistics: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, 
                Domain { cpu_nr: 4, domain_nr: 0, cpu_masks: vec![63], statistics: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, 
                Domain { cpu_nr: 5, domain_nr: 0, cpu_masks: vec![63], statistics: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
            ] 
        });
    }
    #[test]
    fn create_proc_schedstat_file_and_read_multiple_domains() {
        let proc_schedstat = "version 15
timestamp 7452455604
cpu0 0 0 0 0 0 0 1400897766846392 136499688631908 38913706186
domain0 00000000,00000001,00000000,00000001 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
domain1 00000000,ffffffff,00000000,ffffffff 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
domain2 ffffffff,ffffffff,ffffffff,ffffffff 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
cpu127 0 0 0 0 0 0 932015010103181 156376242256299 10212355591
domain0 80000000,00000000,80000000,00000000 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
domain1 ffffffff,00000000,ffffffff,00000000 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
domain2 ffffffff,ffffffff,ffffffff,ffffffff 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0";
        let directory_suffix: String = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        create_dir_all(format!("{}/proc", test_path)).expect("Error creating mock directory.");

        write(format!("{}/schedstat", test_path), proc_schedstat).expect(format!("Error writing to {}/schedstat", test_path).as_str());

        let result = Builder::new().path(&test_path).read().unwrap();
        remove_dir_all(test_path).unwrap();

        assert_eq!(result, ProcSchedStat { version: 15, 
            timestamp: 7452455604, 
            cpu: vec![
                vec![0, 0, 0, 0, 0, 0, 0, 1400897766846392, 136499688631908, 38913706186], 
                vec![127, 0, 0, 0, 0, 0, 0, 932015010103181, 156376242256299, 10212355591]
            ],
            domain: vec![
                Domain {
                    cpu_nr: 0,
                    domain_nr: 0,
                    cpu_masks: vec![ 0, 1, 0, 1, ],
                    statistics: vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
                },
                Domain {
                    cpu_nr: 0,
                    domain_nr: 1,
                    cpu_masks: vec![ 0, 4294967295, 0, 4294967295, ],
                    statistics: vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
                },
                Domain {
                    cpu_nr: 0,
                    domain_nr: 2,
                    cpu_masks: vec![ 4294967295, 4294967295, 4294967295, 4294967295, ],
                    statistics: vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ], 
                },
                Domain {
                    cpu_nr: 127,
                    domain_nr: 0,
                    cpu_masks: vec![ 2147483648, 0, 2147483648,0 ],
                    statistics: vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
                },
                Domain {
                    cpu_nr: 127,
                    domain_nr: 1,
                    cpu_masks: vec![ 4294967295, 0, 4294967295, 0, ],
                    statistics: vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
                },
                Domain {
                    cpu_nr: 127,
                    domain_nr: 2,
                    cpu_masks: vec![ 4294967295, 4294967295, 4294967295, 4294967295, ],
                    statistics: vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
                },
            ],
        }
        );
   }
}
