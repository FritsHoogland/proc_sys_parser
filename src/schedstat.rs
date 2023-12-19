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

let proc_schedstat = Builder::new().file_name("/myproc/schedstat").read();
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

/// Builder pattern for [`ProcSchedStat`]
pub struct Builder
{
    pub proc_schedstat_file: String
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
impl Builder
{
    pub fn new() -> Builder
    {
        Builder { proc_schedstat_file: "/proc/schedstat".to_string() }
    }

    pub fn file_name(mut self, proc_schedstat_file: &str) -> Builder
    {
        self.proc_schedstat_file = proc_schedstat_file.to_string();
        self
    }
    pub fn read(self) -> ProcSchedStat
    {
        ProcSchedStat::read_proc_schedstat(&self.proc_schedstat_file)
    }
}

/// The main function for building a [`ProcSchedStat`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> ProcSchedStat
{
    Builder::new().read()
}

impl Default for ProcSchedStat {
    fn default() -> Self {
        Self::new()
    }
}
/// Struct for holding `/proc/schedstat` statistics
#[derive(Debug, PartialEq)]
pub struct ProcSchedStat {
    pub version: u64,
    pub timestamp: u64,
    pub cpu: Vec<Vec<u64>>,
    pub domain: Vec<Vec<u64>>,
}

impl ProcSchedStat {
    pub fn new() -> ProcSchedStat {
        ProcSchedStat {
            version: 0,
            timestamp: 0,
            cpu: vec![],
            domain: vec![],
        }
    }
    pub fn parse_proc_schedstat_output(
        proc_schedstat: &str,
    ) -> ProcSchedStat
    {
        let mut schedstat = ProcSchedStat::new();
        for line in proc_schedstat.lines()
        {
            match line
            {
                line if line.starts_with("version ") => {
                    schedstat.version = ProcSchedStat::generate_number_unsigned(line);
                },
                line if line.starts_with("timestamp ") => {
                    schedstat.timestamp = ProcSchedStat::generate_number_unsigned(line);
                },
                line if line.starts_with("cpu") => {
                    schedstat.cpu.push(ProcSchedStat::generate_number_vector(line));
                },
                line if line.starts_with("domain") => {
                    schedstat.domain.push(ProcSchedStat::generate_number_vector(line));
                },
                _  => {
                    panic!("Unknown line found in schedstat: {}", line);
                },
            }
        }
        schedstat
    }
    fn generate_number_vector(proc_schedstat_line: &str) -> Vec<u64>
    {
        let proc_schedstat_line = match proc_schedstat_line {
            line if line.starts_with("cpu") => {
                line.split_whitespace()
                    .map(|cpu| if cpu.starts_with("cpu") { cpu.strip_prefix("cpu").unwrap() } else { cpu } )
                    .map(|number| number.parse::<u64>().unwrap())
                    .collect()
            },
            line if line.starts_with("domain") => {
                line.split_whitespace()
                    .map(|domain| if domain.starts_with("domain") { domain.strip_prefix("domain").unwrap() } else { domain } )
                    .map(|number| number.parse::<u64>().unwrap_or(u64::from_str_radix(number, 16).unwrap()))
                    .collect()
            },
            line => panic!("Unknown prefix found: {}", line),
        };
        proc_schedstat_line
    }
    fn generate_number_unsigned(proc_stat_line: &str) -> u64
    {
        proc_stat_line.split_whitespace()
            .skip(1)
            .map(|number| number.parse::<u64>().unwrap())
            .next()
            .unwrap()
    }
    pub fn read_proc_schedstat(proc_schedstat_file: &str) -> ProcSchedStat
    {
        let proc_schedstat_output = read_to_string(proc_schedstat_file).unwrap_or_else(|error| panic!("Error {} reading file: {}", error, proc_schedstat_file));
        ProcSchedStat::parse_proc_schedstat_output(&proc_schedstat_output)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{write, remove_file};
    use super::*;

    // cpu times are in jiffies, which are clock ticks.
    // clock ticks are defined in the getconf value CLK_TCK.
    // this crate dynamically obtains the CLK_TCK value.
    // the common value of CLK_TCK is 100, which is a hard assumption here.
    #[test]
    fn parse_version_line() {
        let version_line = "version 15";
        let result = ProcSchedStat::generate_number_unsigned(&version_line);
        assert_eq!(result, 15);
    }
    #[test]
    fn parse_timestamp_line() {
        let timestamp_line = "timestamp 4318766637";
        let result = ProcSchedStat::generate_number_unsigned(&timestamp_line);
        assert_eq!(result, 4318766637);
    }

    #[test]
    fn parse_cpu_line() {
        let cpu_line = "cpu0 0 0 0 0 0 0 455307306435 48519572891 4320349";
        let result = ProcSchedStat::generate_number_vector(&cpu_line);
        assert_eq!(result, vec![0, 0, 0, 0, 0, 0, 0, 455307306435, 48519572891, 4320349]);
    }

    #[test]
    fn parse_domain_line() {
        let domain_line = "domain0 3f 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0";
        let result = ProcSchedStat::generate_number_vector(&domain_line);
        assert_eq!(result, vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
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
        let result = ProcSchedStat::parse_proc_schedstat_output(proc_schedstat);
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
                vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]
        });
    }
    #[test]
    fn create_proc_schedstat_file_and_read()
    {
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
        write("/tmp/_test_proc_schedstat", proc_schedstat).expect("Error writing to /tmp/_test_proc_schedstat");
        let result = Builder::new().file_name("/tmp/_test_proc_schedstat").read();
        remove_file("/tmp/_test_proc_schedstat").unwrap();
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
                vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]
        });
    }
}
