/*!
Read data from `/proc/pressure/cpu`, `/proc/pressure/io`, `/proc/pressure/memory` into the struct [`ProcPressure`].

The processor of `/proc/pressure` takes the values from the files, and puts them in the struct [`ProcPressure`].
The files are cpu, io and memory as topics for pressure information.
Inside the files, these are divided between some and full, meaning some tasks were affected or full, meaning all tasks were.
For both some and full, the fields are a percentage of ? for 10 seconds, 60 seconds and 300 seconds, and total time spent
waiting in microseconds. (the linux kernel is not consistent with time units, having jiffies, nanoseconds and milliseconds as units).

Documentation: <https://docs.kernel.org/accounting/psi.html>

Here is an example obtaining the data from `/proc/pressure`:
```no_run
use proc_sys_parser::pressure;

let proc_pressure = pressure::read();

println!("{:#?}", proc_pressure);
```
Example output:
```text
ProcPressure {
            psi: Some(
                Psi {
                    cpu_some_avg10: 1.0,
                    cpu_some_avg60: 2.0,
                    cpu_some_avg300: 3.0,
                    cpu_some_total: 373300065,
                    cpu_full_avg10: Some( 4.0 ),
                    cpu_full_avg60: Some( 5.0 ),
                    cpu_full_avg300: Some( 6.0 ),
                    cpu_full_total: Some( 0 ),
                    io_some_avg10: 7.0,
                    io_some_avg60: 8.0,
                    io_some_avg300: 9.0,
                    io_some_total: 55345502,
                    io_full_avg10: 10.0,
                    io_full_avg60: 11.0,
                    io_full_avg300: 12.0,
                    io_full_total: 53895423,
                    memory_some_avg10: 13.0,
                    memory_some_avg60: 14.0,
                    memory_some_avg300: 15.0,
                    memory_some_total: 5425111,
                    memory_full_avg10: 16.0,
                    memory_full_avg60: 17.0,
                    memory_full_avg300: 18.0,
                    memory_full_total: 5390695,
                }
            )
        }
```
(edited for readability)

If you want to change the default path that is read for [`ProcPressure`], which is `/proc`, use:
```no_run
use proc_sys_parser::{pressure, pressure::Builder};

let proc_pressure = Builder::new().path("/myproc").read();
```

If the `/proc/pressure` entry is not available because it didn't exist in that linux version, or because it's not enabled
The ProcPressure.psi entry is set to None.

*/
use std::fs::read_to_string;
use log::warn;


/// Struct for holding `/proc/pressure` statistics
#[derive(Debug, PartialEq, Default)]
pub struct ProcPressure {
    /// psi is None if no /proc/pressure is found.
    pub psi: Option<Psi>,
}
///
#[derive(Debug, PartialEq, Default)]
pub struct Psi {
    pub cpu_some_avg10: f64,
    pub cpu_some_avg60: f64,
    pub cpu_some_avg300: f64,
    pub cpu_some_total: u64,
    pub cpu_full_avg10: Option<f64>,
    pub cpu_full_avg60: Option<f64>,
    pub cpu_full_avg300: Option<f64>,
    pub cpu_full_total: Option<u64>,
    pub io_some_avg10: f64,
    pub io_some_avg60: f64,
    pub io_some_avg300: f64,
    pub io_some_total: u64,
    pub io_full_avg10: f64,
    pub io_full_avg60: f64,
    pub io_full_avg300: f64,
    pub io_full_total: u64,
    pub memory_some_avg10: f64,
    pub memory_some_avg60: f64,
    pub memory_some_avg300: f64,
    pub memory_some_total: u64,
    pub memory_full_avg10: f64,
    pub memory_full_avg60: f64,
    pub memory_full_avg300: f64,
    pub memory_full_total: u64,
}

impl Psi
{
    pub fn new() -> Psi
    {
        Psi::default() 
    }
}

/// Builder pattern for [`ProcPressure`]
#[derive(Default)]
pub struct Builder {
    pub proc_path : String,
}

impl Builder
{
    pub fn new() -> Builder {
        Builder { 
            proc_path: "/proc".to_string(),
        }
    }

    pub fn path(mut self, proc_path: &str) -> Builder {
        self.proc_path = proc_path.to_string();
        self
    }
    pub fn read(self) -> ProcPressure {
        ProcPressure::read_proc_pressure(format!("{}/pressure", &self.proc_path).as_str())
    }
}

/// The main function for building a [`ProcPressure`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> ProcPressure {
   Builder::new().read()
}

impl ProcPressure {
    pub fn new() -> ProcPressure {
        ProcPressure {
            psi: None,
        }
    }
    pub fn read_proc_pressure(proc_pressure_path: &str) -> ProcPressure {
        let mut proc_pressure = ProcPressure::new();

        let mut psi = Psi::new();

        for psi_target in ["cpu", "io", "memory"] {
            if ProcPressure::parse_pressure_entity(psi_target, proc_pressure_path, &mut psi).is_none() {
                return proc_pressure;
            }
        }
        proc_pressure.psi = Some(psi);

        proc_pressure
    }
    fn parse_pressure_entity(file: &str, proc_pressure_path: &str, psi: &mut Psi) -> Option<usize> {
        match read_to_string(format!("{}/{}", &proc_pressure_path, file)) {
            Ok(psi_contents)  => {
                for line in psi_contents.lines() {
                    match line.split_whitespace().next() {
                        Some("some") => {
                            match file {
                                "cpu" => {
                                    psi.cpu_some_avg10 = line.split_whitespace().nth(1).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.cpu_some_avg60 = line.split_whitespace().nth(2).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.cpu_some_avg300 = line.split_whitespace().nth(3).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.cpu_some_total = line.split_whitespace().nth(4).unwrap().split('=').nth(1).unwrap().parse::<u64>().unwrap();
                                },
                                "io" => {
                                    psi.io_some_avg10 = line.split_whitespace().nth(1).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.io_some_avg60 = line.split_whitespace().nth(2).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.io_some_avg300 = line.split_whitespace().nth(3).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.io_some_total = line.split_whitespace().nth(4).unwrap().split('=').nth(1).unwrap().parse::<u64>().unwrap();
                                },
                                "memory" => {
                                    psi.memory_some_avg10 = line.split_whitespace().nth(1).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.memory_some_avg60 = line.split_whitespace().nth(2).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.memory_some_avg300 = line.split_whitespace().nth(3).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.memory_some_total = line.split_whitespace().nth(4).unwrap().split('=').nth(1).unwrap().parse::<u64>().unwrap();
                                },
                                &_ => warn!("Unknown entry in some: {}, {}", file, line),
                            }
                        },
                        Some("full") => {
                            match file {
                                "cpu" => {
                                    psi.cpu_full_avg10 = Some(line.split_whitespace().nth(1).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap());
                                    psi.cpu_full_avg60 = Some(line.split_whitespace().nth(2).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap());
                                    psi.cpu_full_avg300 = Some(line.split_whitespace().nth(3).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap());
                                    psi.cpu_full_total = Some(line.split_whitespace().nth(4).unwrap().split('=').nth(1).unwrap().parse::<u64>().unwrap());
                                },
                                "io" => {
                                    psi.io_full_avg10 = line.split_whitespace().nth(1).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.io_full_avg60 = line.split_whitespace().nth(2).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.io_full_avg300 = line.split_whitespace().nth(3).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.io_full_total = line.split_whitespace().nth(4).unwrap().split('=').nth(1).unwrap().parse::<u64>().unwrap();
                                },
                                "memory" => {
                                    psi.memory_full_avg10 = line.split_whitespace().nth(1).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.memory_full_avg60 = line.split_whitespace().nth(2).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.memory_full_avg300 = line.split_whitespace().nth(3).unwrap().split('=').nth(1).unwrap().parse::<f64>().unwrap();
                                    psi.memory_full_total = line.split_whitespace().nth(4).unwrap().split('=').nth(1).unwrap().parse::<u64>().unwrap();
                                },
                                &_ => warn!("Unknown entry in full: {}, {}", file, line),
                            }
                        },
                        Some(&_) => warn!("Unknown entry found: {}", line),
                        None => {},
                    }
                }
                Some(1)
            },
            Err(_) => {
                None
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{write, remove_dir_all, create_dir_all};
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    use super::*;

    #[test]
    fn create_proc_pressure_directory_and_files_and_read() {
        let proc_pressure_cpu = "some avg10=1.00 avg60=2.00 avg300=3.00 total=373300065
full avg10=4.00 avg60=5.00 avg300=6.00 total=0
";
        let proc_pressure_io = "some avg10=7.00 avg60=8.00 avg300=9.00 total=55345502
full avg10=10.00 avg60=11.00 avg300=12.00 total=53895423
";
        let proc_pressure_memory = "some avg10=13.00 avg60=14.00 avg300=15.00 total=5425111
full avg10=16.00 avg60=17.00 avg300=18.00 total=5390695
";

        let directory_suffix: String = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        create_dir_all(format!("{}/pressure", test_path)).expect("Error creating mock directory.");

        write(format!("{}/pressure/cpu", test_path), proc_pressure_cpu).expect(format!("Error writing to {}/pressure/cpu", test_path).as_str());
        write(format!("{}/pressure/io", test_path), proc_pressure_io).expect(format!("Error writing to {}/pressure/io", test_path).as_str());
        write(format!("{}/pressure/memory", test_path), proc_pressure_memory).expect(format!("Error writing to {}/pressure/memory", test_path).as_str());

        let result = Builder::new().path(&test_path).read();

        remove_dir_all(test_path).unwrap();

        assert_eq!(result, ProcPressure {
            psi: Some(
                Psi {
                    cpu_some_avg10: 1.0,
                    cpu_some_avg60: 2.0,
                    cpu_some_avg300: 3.0,
                    cpu_some_total: 373300065,
                    cpu_full_avg10: Some(
                        4.0,
                    ),
                    cpu_full_avg60: Some(
                        5.0,
                    ),
                    cpu_full_avg300: Some(
                        6.0,
                    ),
                    cpu_full_total: Some(
                        0,
                    ),
                    io_some_avg10: 7.0,
                    io_some_avg60: 8.0,
                    io_some_avg300: 9.0,
                    io_some_total: 55345502,
                    io_full_avg10: 10.0,
                    io_full_avg60: 11.0,
                    io_full_avg300: 12.0,
                    io_full_total: 53895423,
                    memory_some_avg10: 13.0,
                    memory_some_avg60: 14.0,
                    memory_some_avg300: 15.0,
                    memory_some_total: 5425111,
                    memory_full_avg10: 16.0,
                    memory_full_avg60: 17.0,
                    memory_full_avg300: 18.0,
                    memory_full_total: 5390695,
                },
            ),
        });
    }
    #[test]
    fn do_not_create_proc_pressure_directory_for_nonexistent_cases_and_read() {
        let directory_suffix: String = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        create_dir_all(format!("{}", test_path)).expect("Error creating mock directory.");

        let result = Builder::new().path(&test_path).read();
        remove_dir_all(test_path).unwrap();

        assert_eq!(result, ProcPressure { psi: None });
    }
}


