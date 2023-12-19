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

let proc_stat = Builder::new().file_name("/myproc/stat").read();
```

*/
use nix::unistd::{sysconf, SysconfVar};
use std::fs::read_to_string;

/// Struct for holding cpu times in milliseconds
#[derive(Debug, PartialEq)]
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
    pub iowait: u64,
    /// irq time in milliseconds
    pub irq: u64,
    /// softirq time in milliseconds
    pub softirq: u64,
    /// steal time in milliseconds
    /// Introduced with kernel version 2.6.11
    pub steal: u64,
    /// guest user time in milliseconds
    /// Introduced with kernel version 2.6.24
    pub guest: u64,
    /// guest user time reniced in milliseconds
    /// Introduced with kernel version 2.6.24
    pub guest_nice: u64,
}

/// Builder pattern for [`ProcStat`]
pub struct Builder
{
    pub proc_stat_file: String
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
        Builder { proc_stat_file: "/proc/stat".to_string() }
    }

    pub fn file_name(mut self, proc_stat_file: &str) -> Builder
    {
        self.proc_stat_file = proc_stat_file.to_string();
        self
    }
    pub fn read(self) -> ProcStat
    {
        ProcStat::read_proc_stat(&self.proc_stat_file)
    }
}

/// The main function for building a [`ProcStat`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> ProcStat
{
   Builder::new().read()
}

/// Struct for holding `/proc/stat` statistics
#[derive(Debug, PartialEq)]
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

impl Default for ProcStat {
    fn default() -> Self {
        Self::new()
    }
}
impl ProcStat {
    pub fn new() -> ProcStat {
        ProcStat {
            cpu_total: CpuStat::new(),
            cpu_individual: vec![],
            interrupts: vec![],
            context_switches: 0,
            boot_time: 0,
            processes: 0,
            processes_running: 0,
            processes_blocked: 0,
            softirq: vec![],
        }
    }
    pub fn parse_proc_stat_output(
        proc_stat: &str,
    ) -> ProcStat
    {
        let mut procstat = ProcStat::new();
        for line in proc_stat.lines()
        {
            match line
            {
                line if line.starts_with("cpu ") => {
                    procstat.cpu_total = CpuStat::generate_cpu_times(line);
                },
                line if line.starts_with("cpu") && line.chars().nth(3) != Some(' ') => {
                    procstat.cpu_individual.push(CpuStat::generate_cpu_times(line));
                },
                line if line.starts_with("intr ") => {
                    procstat.interrupts = ProcStat::generate_number_vector(line);
                },
                line if line.starts_with("ctxt ") => {
                    procstat.context_switches = ProcStat::generate_number_unsigned(line);
                },
                line if line.starts_with("btime ") => {
                    procstat.boot_time = ProcStat::generate_number_unsigned(line);
                },
                line if line.starts_with("processes ") => {
                    procstat.processes = ProcStat::generate_number_unsigned(line);
                },
                line if line.starts_with("procs_running ") => {
                    procstat.processes_running = ProcStat::generate_number_unsigned(line);
                },
                line if line.starts_with("procs_blocked ") => {
                    procstat.processes_blocked = ProcStat::generate_number_unsigned(line);
                },
                line if line.starts_with("softirq ") => {
                    procstat.softirq = ProcStat::generate_number_vector(line);
                },
                _  => {
                    panic!("Unknown line found in stat: {}", line);
                },
            }
        }
        procstat
    }
    fn generate_number_vector(proc_stat_line: &str) -> Vec<u64>
    {
        proc_stat_line.split_whitespace()
            .skip(1)
            .map(|number| number.parse::<u64>().unwrap())
            .collect()
    }
    fn generate_number_unsigned(proc_stat_line: &str) -> u64
    {
        proc_stat_line.split_whitespace()
            .skip(1)
            .map(|number| number.parse::<u64>().unwrap())
            .next()
            .unwrap()
    }
    pub fn read_proc_stat(proc_stat_file: &str) -> ProcStat
    {
        //let proc_stat_file = proc_stat_file.unwrap_or("/proc/stat");
        let proc_stat_output = read_to_string(proc_stat_file).unwrap_or_else(|error|panic!("Error {} reading file: {}", error, proc_stat_file));
        ProcStat::parse_proc_stat_output(&proc_stat_output)
    }
}

impl CpuStat {
    fn new() -> CpuStat {
        CpuStat {
            name: "".to_string(),
            user: 0,
            nice: 0,
            system: 0,
            idle: 0,
            iowait: 0,
            irq: 0,
            softirq: 0,
            steal: 0,
            guest: 0,
            guest_nice: 0,
        }
    }
    pub fn generate_cpu_times(proc_stat_cpu_line: &str) -> CpuStat
    {
        // Note: time in jiffies, must be divided by CLK_TCK to show time in seconds.
        // CLK_TCK is set by CONFIG_HZ and is 100 on most enterprise linuxes.
        let clock_time = sysconf(SysconfVar::CLK_TCK).unwrap_or(Some(100)).unwrap_or(100) as u64;

        let mut splitted = proc_stat_cpu_line.split_whitespace();
        CpuStat {
            name: splitted.next().unwrap().to_string(),
            user: ((splitted.next().unwrap().parse::<u64>().unwrap()*1000_u64)/clock_time),
            nice: ((splitted.next().unwrap().parse::<u64>().unwrap()*1000_u64)/clock_time),
            system: ((splitted.next().unwrap().parse::<u64>().unwrap()*1000_u64)/clock_time),
            idle: ((splitted.next().unwrap().parse::<u64>().unwrap()*1000_u64)/clock_time),
            iowait: ((splitted.next().unwrap_or_default().parse::<u64>().unwrap_or_default()*1000_u64)/clock_time),
            irq: ((splitted.next().unwrap_or_default().parse::<u64>().unwrap_or_default()*1000_u64)/clock_time),
            softirq: ((splitted.next().unwrap_or_default().parse::<u64>().unwrap_or_default()*1000_u64)/clock_time),
            steal: ((splitted.next().unwrap_or_default().parse::<u64>().unwrap_or_default()*1000_u64)/clock_time),
            guest: ((splitted.next().unwrap_or_default().parse::<u64>().unwrap_or_default()*1000_u64)/clock_time),
            guest_nice: ((splitted.next().unwrap_or_default().parse::<u64>().unwrap_or_default()*1000_u64)/clock_time),
        }
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
    fn parse_cpu_line() {
        let cpu_line = "cpu  101521 47 66467 43586274 7651 0 1367 0 0 0";
        let result = CpuStat::generate_cpu_times(&cpu_line);
        assert_eq!(result, CpuStat { name:"cpu".to_string(), user:1015210, nice:470, system:664670, idle:435862740, iowait:76510, irq:0, softirq:13670, steal:0, guest:0, guest_nice:0 });
    }

    // This mimics a (much) lower linux version which provides lesser statistics
    // The statistics will be set to zero.
    #[test]
    fn parse_cpu_line_with_less_statistics() {
        let cpu_line = "cpu  101521 47 66467 43586274";
        let result = CpuStat::generate_cpu_times(&cpu_line);
        assert_eq!(result, CpuStat { name:"cpu".to_string(), user:1015210, nice:470, system:664670, idle:435862740, iowait:0, irq:0, softirq:0, steal:0, guest:0, guest_nice:0 });
    }


    #[test]
    fn parse_interrupt_line() {
        let interrupt_line = "intr 21965856 0 520030 7300523 0 0 0 2 0 0 0 12267292 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 644 0 0 0 0 0 2 0 77822 81889 80164 70697 68349 79207 0 0 0 6172 6117 6131 5983 6483 6062 0 588204 437602 0 0 1202 0 0 0 0 0 0 0 0 0 0 0 355279 0 0";
        let result = ProcStat::generate_number_vector(&interrupt_line);
        assert_eq!(result, vec![21965856, 0, 520030, 7300523, 0, 0, 0, 2, 0, 0, 0, 12267292, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 644, 0, 0, 0, 0, 0, 2, 0, 77822, 81889, 80164, 70697, 68349, 79207, 0, 0, 0, 6172, 6117, 6131, 5983, 6483, 6062, 0, 588204, 437602, 0, 0, 1202, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 355279, 0, 0]);
    }

    #[test]
    fn parse_context_switches_line() {
        let context_switches_line = "ctxt 36432936";
        let result = ProcStat::generate_number_unsigned(context_switches_line);
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
        let result = ProcStat::parse_proc_stat_output(proc_stat);
        assert_eq!(result, ProcStat { cpu_total: CpuStat { name: "cpu".to_string(), user: 1015210, nice: 470, system: 664670, idle: 435862740, iowait: 76510, irq: 0, softirq: 13670, steal: 0, guest: 0, guest_nice: 0 },
            cpu_individual: vec![CpuStat { name: "cpu0".to_string(), user: 162980, nice: 0, system: 115900, idle: 72592620, iowait: 12130, irq: 0, softirq: 8460, steal: 0, guest: 0, guest_nice: 0 },
                                 CpuStat { name: "cpu1".to_string(), user: 162720, nice: 0, system: 112910, idle: 72656150, iowait: 12890, irq: 0, softirq: 1100, steal: 0, guest: 0, guest_nice: 0 },
                                 CpuStat { name: "cpu2".to_string(), user: 161210, nice: 470, system: 109860, idle: 72663580, iowait: 12510, irq: 0, softirq: 1110, steal: 0, guest: 0, guest_nice: 0 },
                                 CpuStat { name: "cpu3".to_string(), user: 177860, nice: 0, system: 110230, idle: 72647150, iowait: 13500, irq: 0, softirq: 1160, steal: 0, guest: 0, guest_nice: 0 },
                                 CpuStat { name: "cpu4".to_string(), user: 174260, nice: 0, system: 107360, idle: 72654910, iowait: 11950, irq: 0, softirq: 790, steal: 0, guest: 0, guest_nice: 0 },
                                 CpuStat { name: "cpu5".to_string(), user: 176160, nice: 0, system: 108400, idle: 72648320, iowait: 13510, irq: 0, softirq: 1030, steal: 0, guest: 0, guest_nice: 0 }],
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
        write("/tmp/_test_proc_stat", proc_stat).expect("Error writing to /tmp/_test_proc_stat");
        let result = Builder::new().file_name("/tmp/_test_proc_stat").read();
        remove_file("/tmp/_test_proc_stat").unwrap();
        assert_eq!(result, ProcStat { cpu_total: CpuStat { name: "cpu".to_string(), user: 10, nice: 10, system: 10, idle: 10, iowait: 10, irq: 0, softirq: 10, steal: 0, guest: 0, guest_nice: 0 },
            cpu_individual: vec![CpuStat { name: "cpu0".to_string(),user: 10, nice: 10, system: 10, idle: 10, iowait: 10, irq: 0, softirq: 10, steal: 0, guest: 0, guest_nice: 0 }],
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
