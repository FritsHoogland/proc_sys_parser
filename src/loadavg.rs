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

let proc_loadavg = Builder::new().path("/myproc").read();
```

*/
use std::fs::read_to_string;
use crate::ProcSysParserError;

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
#[derive(Default)]
pub struct Builder {
    pub proc_path: String,
    pub proc_file: String,
}

impl Builder {
    pub fn new() -> Builder {
        Builder { 
            proc_path: "/proc".to_string(),
            proc_file: "loadavg".to_string(),
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
    //pub fn read(self) -> Result<ProcLoadavg, ProcSysParserError> {
    pub fn read(self) -> Result<ProcLoadavg, ProcSysParserError> {
        ProcLoadavg::read_proc_loadavg(format!("{}/{}", &self.proc_path, &self.proc_file).as_str())
    }
}

/// The main function for building a [`ProcMemInfo`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
//pub fn read() -> Result<()> {
pub fn read() -> Result<ProcLoadavg, ProcSysParserError> {
   Builder::new().read()
}

impl ProcLoadavg {
    pub fn new() -> ProcLoadavg {
        ProcLoadavg::default()
    }
    pub fn parse_proc_loadavg( proc_loadavg: &str,) -> Result<ProcLoadavg, ProcSysParserError>
    {
        let mut fields = proc_loadavg.split_whitespace();
        let mut fields_copy = fields.clone();

        Ok(ProcLoadavg {
            //load_1: fields.next().ok_or(ProcSysParserError::IteratorItemError)?.parse::<f64>().unwrap(),
            load_1: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "loadavg load_1".to_string() })?
                .parse::<f64>().map_err(|error| ProcSysParserError::ParseToFloatError(error))?,
            load_5: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "loadavg load_5".to_string() })?
                .parse::<f64>().map_err(|error| ProcSysParserError::ParseToFloatError(error))?,
            load_15: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "loadavg load_15". to_string() })?
                .parse::<f64>().map_err(|error| ProcSysParserError::ParseToFloatError(error))?,
            current_runnable: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "loadavg current_runnable".to_string() })?
                .split('/').next().ok_or(ProcSysParserError::IteratorItemError {item: "loadavg current_runnable split".to_string() })?
                .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?,
            total: fields_copy.nth(3)
                .ok_or(ProcSysParserError::IteratorItemError {item: "loadavg total".to_string() })?
                .split('/').nth(1).ok_or(ProcSysParserError::IteratorItemError {item: "loadavg total split".to_string() })?
                .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?,
            last_pid: fields.next()
                .ok_or(ProcSysParserError::IteratorItemError {item: "loadavg last_pid".to_string() })?
                .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?,
        })
    }
    pub fn read_proc_loadavg(proc_loadavg: &str) -> Result<ProcLoadavg, ProcSysParserError>
    {
        let proc_loadavg_output = read_to_string(proc_loadavg)
            .map_err(|error| ProcSysParserError::FileReadError { file: proc_loadavg.to_string(), error })?;

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
        let result = ProcLoadavg::parse_proc_loadavg(&loadavg_line).unwrap();
        assert_eq!(result, ProcLoadavg { load_1: 0.05, load_5: 0.19, load_15: 0.13, current_runnable: 1, total: 161, last_pid: 7 });
    }

    #[test]
    fn create_proc_loadavg_file_and_read() {
        let proc_loadavg = format!("0.05 0.19 0.13 1/161 7\n");

        let directory_suffix: String = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        create_dir_all(format!("{}", test_path)).expect("Error creating mock directory.");

        write(format!("{}/loadavg", test_path), proc_loadavg).expect(format!("Error writing to {}/loadavg", test_path).as_str());
        let result = Builder::new().path(&test_path).read().unwrap();

        remove_dir_all(test_path).unwrap();

        assert_eq!(result, ProcLoadavg { load_1: 0.05, load_5: 0.19, load_15: 0.13, current_runnable: 1, total: 161, last_pid: 7 });
    }
    #[test]
   fn read_nonexistent_loadavg_file() -> Result<(), ProcSysParserError> {
        // uncomment to see the error message
        //let _result = Builder::new().path("/xxxxxxxxxxxx").read()?;
        Ok(assert!(Builder::new().path("/xxxxxxxxxxxx").read().is_err()))
    }

    #[test]
    fn parse_corrupted_loadavg_line_missing_entries() -> Result<(), ProcSysParserError> {
        let loadavg_line = format!("0.05 0.19\n");
        //let result = ProcLoadavg::parse_proc_loadavg(&loadavg_line)?;
        let result = ProcLoadavg::parse_proc_loadavg(&loadavg_line);
        Ok(assert!(result.is_err()))
    }
    #[test]
    fn parse_corrupted_loadavg_line_wrong_entry() -> Result<(), ProcSysParserError> {
        let loadavg_line = format!("AAA 0.19 0.13 1/161 7\n");
        //let result = ProcLoadavg::parse_proc_loadavg(&loadavg_line)?;
        let result = ProcLoadavg::parse_proc_loadavg(&loadavg_line);
        Ok(assert!(result.is_err()))
    }
}


