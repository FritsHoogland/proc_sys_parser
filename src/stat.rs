/*!
Read data from `/proc/stat` into the struct [`ProcStat`].

The Documentation for `/proc/stat` is found here: <https://www.kernel.org/doc/Documentation/filesystems/proc.txt>.

Please mind that the description of "steal" time in the kernel source describes 'involuntary wait time'.
This is true, but involuntary waiting means virtualization makes the kernel (virtual machine) wait.
This is implemented for PowerPC, S390 and X86, and for X86 for paravirtualization.

The stat module converts the jiffies from `/proc/stat` from the cpu_total and cpu_individual [`CpuStat`]
structs into milliseconds. It does that by taking the `CLK-TCK` (clock tick) sysconf variable set by
`CONFIG_HZ`, and calculate the time in milliseconds from the cpu state jiffies value in the following way:

```text
(CPUSTATE_JIFFIES * 1000)        / CLK_TCK
convert seconds to milliseconds    divide by ticks per second
```
Example usage of stat:
```no_run
use proc_sys_parser::{stat, stat::{ProcStat, CpuStat}};

let proc_stat = stat::read();
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
    softirq: [99012, 30, 8368, 2, 24666, 11, 0, 208, 15031, 0, 50696]
}
```
(edited for readability)

If you want to change the path and/or file that is read for [`ProcStat`], which is `/proc/stat`, by
default, use:
```no_run
use proc_sys_parser::{stat, stat::{ProcStat, CpuStat, Builder}};

let proc_stat = Builder::new().path("/myproc").read();
```

*/
use nix::unistd::{sysconf, SysconfVar};
use std::fs::read_to_string;
use crate::ProcSysParserError;
use log::warn;


/// Struct for holding cpu times in milliseconds
#[derive(Debug, PartialEq, Default)]
pub struct CpuStat {
    /// cpu name. 'cpu' means total of all cpus, cpuN means individual cpu
    pub name: String,
    /// user time in milliseconds
    pub user: u64,
    /// user time reniced in milliseconds
    pub nice: u64,
    /// system/kernel time in milliseconds
    pub system: u64,
    /// idle time in milliseconds
    pub idle: u64,
    /// idle time in milliseconds attributed to performing IO
    pub iowait: Option<u64>,
    /// irq time in milliseconds
    pub irq: Option<u64>,
    /// softirq time in milliseconds
    pub softirq: Option<u64>,
    /// steal time in milliseconds
    /// Introduced with kernel version 2.6.11
    pub steal: Option<u64>,
    /// guest user time in milliseconds
    /// Introduced with kernel version 2.6.24
    pub guest: Option<u64>,
    /// guest user time reniced in milliseconds
    /// Introduced with kernel version 2.6.24
    pub guest_nice: Option<u64>,
}

/// Builder pattern for [`ProcStat`]
#[derive(Default)]
pub struct Builder {
    pub proc_path : String,
    pub proc_file : String,
}

impl Builder {
    pub fn new() -> Builder {
        Builder { 
            proc_path: "/proc".to_string(),
            proc_file: "stat".to_string(),
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
    pub fn read(self) -> Result<ProcStat, ProcSysParserError> {
        ProcStat::read_proc_stat(format!("{}/{}", &self.proc_path, &self.proc_file).as_str())
    }
}

/// The main function for building a [`ProcStat`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> Result<ProcStat, ProcSysParserError> {
   Builder::new().read()
}

/// Struct for holding `/proc/stat` statistics
#[derive(Debug, PartialEq, Default)]
pub struct ProcStat {
    pub cpu_total: CpuStat,
    pub cpu_individual: Vec<CpuStat>,
    pub interrupts: Vec<u64>,
    pub context_switches: u64,
    pub boot_time: u64,
    pub processes: u64,
    pub processes_running: u64,
    pub processes_blocked: u64,
    pub softirq: Vec<u64>,
}

impl ProcStat {
    pub fn new() -> ProcStat {
        ProcStat::default() 
    }
    pub fn parse_proc_stat_output(proc_stat: &str,) -> Result<ProcStat, ProcSysParserError> {
        let mut procstat = ProcStat::new();
        for line in proc_stat.lines() {
            match line {
                line if line.starts_with("cpu ") => {
                    procstat.cpu_total = CpuStat::generate_cpu_times(line)?;
                },
                line if line.starts_with("cpu") && line.chars().nth(3) != Some(' ') => {
                    procstat.cpu_individual.push(CpuStat::generate_cpu_times(line)?);
                },
                line if line.starts_with("intr ") => {
                    procstat.interrupts = ProcStat::generate_number_vector(line)?;
                },
                line if line.starts_with("ctxt ") => {
                    procstat.context_switches = ProcStat::generate_number_unsigned(line)?;
                },
                line if line.starts_with("btime ") => {
                    procstat.boot_time = ProcStat::generate_number_unsigned(line)?;
                },
                line if line.starts_with("processes ") => {
                    procstat.processes = ProcStat::generate_number_unsigned(line)?;
                },
                line if line.starts_with("procs_running ") => {
                    procstat.processes_running = ProcStat::generate_number_unsigned(line)?;
                },
                line if line.starts_with("procs_blocked ") => {
                    procstat.processes_blocked = ProcStat::generate_number_unsigned(line)?;
                },
                line if line.starts_with("softirq ") => {
                    procstat.softirq = ProcStat::generate_number_vector(line)?;
                },
                _  => warn!("stat: unknown entry found: {}", line),
            }
        }
        Ok(procstat)
    }
    fn generate_number_vector(proc_stat_line: &str) -> Result<Vec<u64>, ProcSysParserError> {
        proc_stat_line.split_whitespace()
            .skip(1)
            .map(|row| row.parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError))
            .collect::<Vec<_>>()
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
    }
    fn generate_number_unsigned(proc_stat_line: &str) -> Result<u64, ProcSysParserError> {
        proc_stat_line.split_whitespace()
            .nth(1)
            .ok_or(ProcSysParserError::IteratorItemError {item: "stat generate_number_unsigned".to_string() })?
            .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)
    }
    pub fn read_proc_stat(proc_stat_file: &str) -> Result<ProcStat, ProcSysParserError> {
        let proc_stat_output = read_to_string(proc_stat_file)
            .map_err(|error| ProcSysParserError::FileReadError { file: proc_stat_file.to_string(), error })?;
        ProcStat::parse_proc_stat_output(&proc_stat_output)
    }
}

impl CpuStat {
    pub fn generate_cpu_times(proc_stat_cpu_line: &str) -> Result<CpuStat, ProcSysParserError> {
        // Note: time in jiffies, must be divided by CLK_TCK to show time in seconds.
        // CLK_TCK is set by CONFIG_HZ and is 100 on most enterprise linuxes.
        let clock_time = sysconf(SysconfVar::CLK_TCK).unwrap_or(Some(100)).unwrap_or(100) as u64;

        let parse_next_and_conversion_into_option_milliseconds = |result: Option<&str>, clock_time: u64 | -> Option<u64> {
            match result {
                None => None,
                Some(value) => {
                    match value.parse::<u64>() {
                        Err(_) => None,
                        Ok(number) => Some((number*1000_u64)/clock_time),
                    }
                },
            }
        };

        let mut splitted = proc_stat_cpu_line.split_whitespace();
        Ok(CpuStat {
            name: splitted.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "stat generate_cpu_times name".to_string() })?
                .to_string(),
            user: ((splitted.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "stat generate_cpu_times user".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)? *1000_u64)/clock_time),
            nice: ((splitted.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "stat generate_cpu_times nice".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)? *1000_u64)/clock_time),
            system: ((splitted.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "stat generate_cpu_times system".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)? *1000_u64)/clock_time),
            idle: ((splitted.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "stat generate_cpu_times idle".to_string() })?
                .parse::<u64>().map_err(ProcSysParserError::ParseToIntegerError)? *1000_u64)/clock_time),
            iowait: parse_next_and_conversion_into_option_milliseconds(splitted.next(), clock_time),
            irq: parse_next_and_conversion_into_option_milliseconds(splitted.next(), clock_time),
            softirq: parse_next_and_conversion_into_option_milliseconds(splitted.next(), clock_time),
            steal: parse_next_and_conversion_into_option_milliseconds(splitted.next(), clock_time),
            guest: parse_next_and_conversion_into_option_milliseconds(splitted.next(), clock_time),
            guest_nice: parse_next_and_conversion_into_option_milliseconds(splitted.next(), clock_time),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{write, create_dir_all, remove_dir_all};
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    use super::*;

    // cpu times are in jiffies, which are clock ticks.
    // clock ticks are defined in the getconf value CLK_TCK.
    // this crate dynamically obtains the CLK_TCK value.
    // the common value of CLK_TCK is 100, which is a hard assumption here.
    #[test]
    fn parse_cpu_line() {
        let cpu_line = "cpu  101521 47 66467 43586274 7651 0 1367 0 0 0";
        let result = CpuStat::generate_cpu_times(&cpu_line).unwrap();
        assert_eq!(result, CpuStat { name:"cpu".to_string(), user:1015210, nice:470, system:664670, idle:435862740, iowait:Some(76510), irq:Some(0), softirq:Some(13670), steal:Some(0), guest:Some(0), guest_nice:Some(0) });
    }

    // This mimics a (much) lower linux version which provides lesser statistics
    // The statistics will be set to zero.
    #[test]
    fn parse_cpu_line_with_less_statistics() {
        let cpu_line = "cpu  101521 47 66467 43586274";
        let result = CpuStat::generate_cpu_times(&cpu_line).unwrap();
        assert_eq!(result, CpuStat { name:"cpu".to_string(), user:1015210, nice:470, system:664670, idle:435862740, iowait:None, irq:None, softirq:None, steal:None, guest:None, guest_nice:None });
    }


    #[test]
    fn parse_interrupt_line() {
        let interrupt_line = "intr 21965856 0 520030 7300523 0 0 0 2 0 0 0 12267292 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 644 0 0 0 0 0 2 0 77822 81889 80164 70697 68349 79207 0 0 0 6172 6117 6131 5983 6483 6062 0 588204 437602 0 0 1202 0 0 0 0 0 0 0 0 0 0 0 355279 0 0";
        let result = ProcStat::generate_number_vector(&interrupt_line).unwrap();
        assert_eq!(result, vec![21965856, 0, 520030, 7300523, 0, 0, 0, 2, 0, 0, 0, 12267292, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 644, 0, 0, 0, 0, 0, 2, 0, 77822, 81889, 80164, 70697, 68349, 79207, 0, 0, 0, 6172, 6117, 6131, 5983, 6483, 6062, 0, 588204, 437602, 0, 0, 1202, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 355279, 0, 0]);
    }

    #[test]
    fn parse_context_switches_line() {
        let context_switches_line = "ctxt 36432936";
        let result = ProcStat::generate_number_unsigned(context_switches_line).unwrap();
        assert_eq!(result, 36432936);

    }

    #[test]
    fn parse_full_proc_stat_file() {
        let proc_stat = "cpu  101521 47 66467 43586274 7651 0 1367 0 0 0
cpu0 16298 0 11590 7259262 1213 0 846 0 0 0
cpu1 16272 0 11291 7265615 1289 0 110 0 0 0
cpu2 16121 47 10986 7266358 1251 0 111 0 0 0
cpu3 17786 0 11023 7264715 1350 0 116 0 0 0
cpu4 17426 0 10736 7265491 1195 0 79 0 0 0
cpu5 17616 0 10840 7264832 1351 0 103 0 0 0
intr 21965856 0 520030 7300523 0 0 0 2 0 0 0 12267292 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 644 0 0 0 0 0 2 0 77822 81889 80164 70697 68349 79207 0 0 0 6172 6117 6131 5983 6483 6062 0 588204 437602 0 0 1202 0 0 0 0 0 0 0 0 0 0 0 355279 0 0
ctxt 36432936
btime 1701783048
processes 345159
procs_running 1
procs_blocked 0
softirq 7616206 32 1416021 213 1102885 11 0 1409 2270709 0 2824926";
        let result = ProcStat::parse_proc_stat_output(proc_stat).unwrap();
        assert_eq!(result, ProcStat { cpu_total: CpuStat { name: "cpu".to_string(), user: 1015210, nice: 470, system: 664670, idle: 435862740, iowait: Some(76510), irq: Some(0), softirq: Some(13670), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
            cpu_individual: vec![CpuStat { name: "cpu0".to_string(), user: 162980, nice: 0, system: 115900, idle: 72592620, iowait: Some(12130), irq: Some(0), softirq: Some(8460), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
                                 CpuStat { name: "cpu1".to_string(), user: 162720, nice: 0, system: 112910, idle: 72656150, iowait: Some(12890), irq: Some(0), softirq: Some(1100), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
                                 CpuStat { name: "cpu2".to_string(), user: 161210, nice: 470, system: 109860, idle: 72663580, iowait: Some(12510), irq: Some(0), softirq: Some(1110), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
                                 CpuStat { name: "cpu3".to_string(), user: 177860, nice: 0, system: 110230, idle: 72647150, iowait: Some(13500), irq: Some(0), softirq: Some(1160), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
                                 CpuStat { name: "cpu4".to_string(), user: 174260, nice: 0, system: 107360, idle: 72654910, iowait: Some(11950), irq: Some(0), softirq: Some(790), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
                                 CpuStat { name: "cpu5".to_string(), user: 176160, nice: 0, system: 108400, idle: 72648320, iowait: Some(13510), irq: Some(0), softirq: Some(1030), steal: Some(0), guest: Some(0), guest_nice: Some(0) }],
            interrupts: vec![21965856, 0, 520030, 7300523, 0, 0, 0, 2, 0, 0, 0, 12267292, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 644, 0, 0, 0, 0, 0, 2, 0, 77822, 81889, 80164, 70697, 68349, 79207, 0, 0, 0, 6172, 6117, 6131, 5983, 6483, 6062, 0, 588204, 437602, 0, 0, 1202, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 355279, 0, 0],
            context_switches: 36432936,
            boot_time: 1701783048,
            processes: 345159,
            processes_running: 1,
            processes_blocked: 0,
            softirq: vec![7616206, 32, 1416021, 213, 1102885, 11, 0, 1409, 2270709, 0, 2824926],
        });
    }

    #[test]
    fn create_proc_stat_file_and_read()
    {
        let proc_stat = "cpu  1 1 1 1 1 0 1 0 0 0
cpu0 1 1 1 1 1 0 1 0 0 0
intr 100 0 1 1
ctxt 100
btime 100
processes 10
procs_running 1
procs_blocked 0
softirq 100 0 1 1";
        let directory_suffix: String = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        create_dir_all(test_path.clone()).expect("Error creating mock sysfs directories.");
        
        write(format!("{}/stat", test_path), proc_stat).expect(format!("Error writing to {}/stat", test_path).as_str());
        let result = Builder::new().path(&test_path).read().unwrap();
        remove_dir_all(test_path).unwrap();

        assert_eq!(result, ProcStat { cpu_total: CpuStat { name: "cpu".to_string(), user: 10, nice: 10, system: 10, idle: 10, iowait: Some(10), irq: Some(0), softirq: Some(10), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
            cpu_individual: vec![CpuStat { name: "cpu0".to_string(),user: 10, nice: 10, system: 10, idle: 10, iowait: Some(10), irq: Some(0), softirq: Some(10), steal: Some(0), guest: Some(0), guest_nice: Some(0) }],
            interrupts: vec![100, 0, 1, 1],
            context_switches: 100,
            boot_time: 100,
            processes: 10,
            processes_running: 1,
            processes_blocked: 0,
            softirq: vec![100, 0, 1, 1],
        });
    }
}
