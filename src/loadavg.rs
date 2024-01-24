/*!
Read data from `/proc/loadavg` into the struct [`ProcLoadavg`].

The processor of `/proc/loadavg` takes the values in the file, and puts them in the struct [`ProcLoadavg`].
The values are the load averages for 1, 5 and 15 minutes, the current number runnable processes *slash* total number of
processes and the last pid created.

Documentation: <https://docs.kernel.org/filesystems/proc.html>

Here is an example obtaining the data from `/proc/loadavg`:
```no_run
use proc_sys_parser::loadavg;

let proc_loadavg = loadavg::read();

println!("{:#?}", proc_loadavg);
```
Example output:
```text
ProcLoadavg {
    load_1: 0.0,
    load_5: 0.0,
    load_15: 0.0,
    current_runnable: 0,
    total: 0,
    last_pid: 12345,
}
```
(edited for readability)

If you want to change the path and/or file that is read for [`ProcLoadavg`], which is `/proc/loadavg`
by default, use:
```no_run
use proc_sys_parser::{meminfo, meminfo::Builder};

let proc_loadavg = Builder::new().file_name("/myproc/loadavg").read();
```

*/
use std::fs::read_to_string;

/// Struct for holding `/proc/loadavg` statistics
#[derive(Debug, PartialEq, Default)]
pub struct ProcLoadavg {
    pub load_1: f64,
    pub load_5: f64,
    pub load_15: f64,
    pub current_runnable: u64,
    pub total: u64,
    pub last_pid: u64,
}

/// Builder pattern for [`ProcLoadavg`]
pub struct Builder
{
    pub proc_loadavg_file: String
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
        Builder { proc_loadavg_file: "/proc/loadavg".to_string() }
    }

    pub fn file_name(mut self, proc_loadavg_file: &str) -> Builder
    {
        self.proc_loadavg_file = proc_loadavg_file.to_string();
        self
    }
    pub fn read(self) -> ProcLoadavg
    {
        ProcLoadavg::read_proc_loadavg(&self.proc_loadavg_file)
    }
}

/// The main function for building a [`ProcMemInfo`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> ProcLoadavg
{
   Builder::new().read()
}

impl ProcLoadavg {
    pub fn new() -> ProcLoadavg {
        ProcLoadavg::default()
    }
    pub fn parse_proc_loadavg(
        proc_loadavg: &str,
    ) -> ProcLoadavg
    {
        let mut fields = proc_loadavg.split_whitespace();
        let mut fields_copy = fields.clone();

        ProcLoadavg {
            load_1: fields.next().unwrap().parse::<f64>().unwrap(),
            load_5: fields.next().unwrap().parse::<f64>().unwrap(),
            load_15: fields.next().unwrap().parse::<f64>().unwrap(),
            current_runnable: fields.next().unwrap().split('/').next().unwrap().parse::<u64>().unwrap(),
            total: fields_copy.nth(3).unwrap().split('/').nth(1).unwrap().parse::<u64>().unwrap(),
            last_pid: fields.next().unwrap().parse::<u64>().unwrap(),
        }
    }
    pub fn read_proc_loadavg(proc_meminfo_file: &str) -> ProcLoadavg
    {
        let proc_loadavg_output = read_to_string(proc_meminfo_file).unwrap_or_else(|error|panic!("Error {} reading file: {}", error, proc_meminfo_file));
        ProcLoadavg::parse_proc_loadavg(&proc_loadavg_output)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{write, remove_dir_all, create_dir_all};
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    use super::*;

    #[test]
    fn parse_proc_loadavg_line() {
        let loadavg_line = format!("0.05 0.19 0.13 1/161 7\n");
        let result = ProcLoadavg::parse_proc_loadavg(&loadavg_line);
        assert_eq!(result, ProcLoadavg { load_1: 0.05, load_5: 0.19, load_15: 0.13, current_runnable: 1, total: 161, last_pid: 7 });
    }

    #[test]
    fn create_proc_loadavg_file_and_read() {
        let proc_loadavg = format!("0.05 0.19 0.13 1/161 7\n");

        let directory_suffix: String = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        create_dir_all(format!("{}", test_path)).expect("Error creating mock directory.");

        write(format!("{}/loadavg", test_path), proc_loadavg).expect(format!("Error writing to {}/loadavg", test_path).as_str());
        let result = Builder::new().file_name(format!("{}/loadavg", test_path).as_str()).read();

        remove_dir_all(test_path).unwrap();

        assert_eq!(result, ProcLoadavg { load_1: 0.05, load_5: 0.19, load_15: 0.13, current_runnable: 1, total: 161, last_pid: 7 });
    }
}


