/*!
Read data from `/proc/fs/xfs/stat` into the struct [`ProcFsXfsStat`].

The processor of `/proc/fs/xfs/stat` takes the values in the file, and puts them in the struct [`ProcFsXfsStat`].
The values in the stat file are detailed data about the function of the XFS filesystem on linux.

Documentation:
The documentation seems to be gone, or at least not easy to find. Using the Wayback machine I managed to find an old page that provides a good description:
<https://web.archive.org/web/20220902080208/https://xfs.org/index.php/Runtime_Stats
Here is an example obtaining the data from `/proc/fs/xfs/stat`:
```no_run
use proc_sys_parser::fs_xfs_stat;

let proc_fs_xfs_stat = fs_xfs_stat::read();

println!("{:#?}", proc_fs_xfs_stat);
```
Example output:
```text
ProcFsXfsStat {
    xs_write_calls: 0.0,
    xs_read_calls: 0.0,
    xs_write_bytes: 0.0,
    xs_read_bytes: 0,
}
```
(edited for readability)

If you want to change the path and/or file that is read for [`ProcFsXfsStat`], which is `/proc/fs/xfs/stat`
by default, use:
```no_run
use proc_sys_parser::{meminfo, meminfo::Builder};

let proc_loadavg = Builder::new().path("/myproc").read();
```

*/
use std::fs::read_to_string;

/// Struct for holding `/proc/fs/xfs/stat` statistics
#[derive(Debug, PartialEq, Default)]
pub struct ProcFsXfsStat {
    pub xs_write_calls: Option<u64>,
    pub xs_read_calls: Option<u64>,
    pub xs_write_bytes: Option<u64>,
    pub xs_read_bytes: Option<u64>,
}

/// Builder pattern for [`ProcFsXfsStat`]
#[derive(Default)]
pub struct Builder {
    pub proc_path: String,
    pub proc_file: String,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            proc_path: "/proc".to_string(),
            proc_file: "fs/xfs/stat".to_string(),
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
    pub fn read(self) -> ProcFsXfsStat {
        ProcFsXfsStat::read_proc_fs_xfs_stat(
            format!("{}/{}", &self.proc_path, &self.proc_file).as_str(),
        )
    }
}

/// The main function for building a [`ProcMemInfo`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
//pub fn read() -> Result<()> {
pub fn read() -> ProcFsXfsStat {
    Builder::new().read()
}

impl ProcFsXfsStat {
    pub fn new() -> ProcFsXfsStat {
        ProcFsXfsStat::default()
    }
    pub fn parse_proc_fs_xfs_stat(proc_fs_xfs_stat: &str) -> ProcFsXfsStat {
        let mut proc_fs_xfs_stat_struct = ProcFsXfsStat::new();

        for line in proc_fs_xfs_stat.lines() {
            match line {
                line if line.starts_with("rw ") => {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    proc_fs_xfs_stat_struct.xs_write_calls = fields[1].parse::<u64>().ok();
                    proc_fs_xfs_stat_struct.xs_read_calls = fields[2].parse::<u64>().ok();
                }
                line if line.starts_with("xpc ") => {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    proc_fs_xfs_stat_struct.xs_write_bytes = fields[2].parse::<u64>().ok();
                    proc_fs_xfs_stat_struct.xs_read_bytes = fields[3].parse::<u64>().ok();
                }
                &_ => { /* anything else currently not implemented */ }
            }
        }

        proc_fs_xfs_stat_struct
    }
    pub fn read_proc_fs_xfs_stat(proc_fs_xfs_stat: &str) -> ProcFsXfsStat {
        let proc_fs_xfs_stat_output = read_to_string(proc_fs_xfs_stat).unwrap_or(String::from(""));

        ProcFsXfsStat::parse_proc_fs_xfs_stat(&proc_fs_xfs_stat_output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::fs::{create_dir_all, remove_dir_all, write};

    #[test]
    fn parse_proc_fs_xfs_stat_file() {
        let proc_fs_xfs_stat_file = "extent_alloc 3151384 534926060 3164945 476294498
abt 0 0 0 0
blk_map 353485831 851213330 9893230 6817550 6030586 1560908330 0
bmbt 0 0 0 0
dir 5970443 6945777 6874500 1854279411
trans 39942 3767813357 952
ig 7083866 4980513 0 2103353 0 1340957 3563694470
log 30274117 2045001872 23068 2926274 1401173
push_ail 3772005417 0 347558251 47834761 0 10641092 1400 928650954 0 572818
xstrat 2700722 0
rw 654716312 3877049901
attr 4264349303 6452433 454032 7692
icluster 0 41315812 1121686510
vnodes 762396 0 0 0 6315567 6315567 6315567 0
buf 2125016617 439870 2124576784 214995 432462 439837 0 568672 35579
abtb2 8079740 64538375 1770206 1765154 164 160 17096 10764 5314 4577 197 180 361 340 483092775
abtc2 16641798 135205710 5522173 5517194 236 232 17074 5466 4602 6952 280 264 516 496 1165831063
bmbt2 1170218 8837872 213928 214306 0 0 2153 2180 2381 2153 38 32 38 32 938847
ibt2 12712723 59099644 10126 9119 4 2 3 0 235 1 7 2 11 4 409636
fibt2 17438004 37870019 1700811 1700870 0 0 0 0 0 0 0 0 0 0 18621882
rmapbt 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
refcntbt 220 12 2 2 0 0 0 0 0 0 0 0 0 0 0
qm 0 0 0 0 0 0 0 0 0
xpc 1839985078272 1924118614718 41117984770168
defer_relog 0
debug 0";
        let result = ProcFsXfsStat::parse_proc_fs_xfs_stat(&proc_fs_xfs_stat_file);
        assert_eq!(
            result,
            ProcFsXfsStat {
                xs_write_calls: Some(654716312),
                xs_read_calls: Some(3877049901),
                xs_write_bytes: Some(1924118614718),
                xs_read_bytes: Some(41117984770168),
            }
        );
    }
    #[test]
    fn create_proc_fs_xfs_stat_file_and_read() {
        let proc_fs_xfs_stat_file = "extent_alloc 3151384 534926060 3164945 476294498
abt 0 0 0 0
blk_map 353485831 851213330 9893230 6817550 6030586 1560908330 0
bmbt 0 0 0 0
dir 5970443 6945777 6874500 1854279411
trans 39942 3767813357 952
ig 7083866 4980513 0 2103353 0 1340957 3563694470
log 30274117 2045001872 23068 2926274 1401173
push_ail 3772005417 0 347558251 47834761 0 10641092 1400 928650954 0 572818
xstrat 2700722 0
rw 654716312 3877049901
attr 4264349303 6452433 454032 7692
icluster 0 41315812 1121686510
vnodes 762396 0 0 0 6315567 6315567 6315567 0
buf 2125016617 439870 2124576784 214995 432462 439837 0 568672 35579
abtb2 8079740 64538375 1770206 1765154 164 160 17096 10764 5314 4577 197 180 361 340 483092775
abtc2 16641798 135205710 5522173 5517194 236 232 17074 5466 4602 6952 280 264 516 496 1165831063
bmbt2 1170218 8837872 213928 214306 0 0 2153 2180 2381 2153 38 32 38 32 938847
ibt2 12712723 59099644 10126 9119 4 2 3 0 235 1 7 2 11 4 409636
fibt2 17438004 37870019 1700811 1700870 0 0 0 0 0 0 0 0 0 0 18621882
rmapbt 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
refcntbt 220 12 2 2 0 0 0 0 0 0 0 0 0 0 0
qm 0 0 0 0 0 0 0 0 0
xpc 1839985078272 1924118614718 41117984770168
defer_relog 0
debug 0";

        let directory_suffix: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        create_dir_all(format!("{}/fs/xfs", test_path)).expect("Error creating mock directory.");

        write(format!("{}/fs/xfs/stat", test_path), proc_fs_xfs_stat_file)
            .expect(format!("Error writing to {}/fs/xfs/stat", test_path).as_str());
        let result = Builder::new().path(&test_path).read();

        remove_dir_all(test_path).unwrap();

        assert_eq!(
            result,
            ProcFsXfsStat {
                xs_write_calls: Some(654716312),
                xs_read_calls: Some(3877049901),
                xs_write_bytes: Some(1924118614718),
                xs_read_bytes: Some(41117984770168),
            }
        );
    }
    #[test]
    fn read_nonexistent_proc_fs_xfs_stat_file() -> Result<(), ProcSysParserError> {
        // uncomment to see the error message
        //let _result = Builder::new().path("/xxxxxxxxxxxx").read()?;
        assert_eq!(
            Builder::new().path("/xxxxxxxxxxxx").read(),
            ProcFsXfsStat {
                xs_write_calls: None,
                xs_read_calls: None,
                xs_write_bytes: None,
                xs_read_bytes: None,
            }
        );
        Ok(())
    }

    #[test]
    fn parse_proc_fs_xfs_stat_file_line_missing_xpc_entries() -> Result<(), ProcSysParserError> {
        let proc_fs_xfs_stat_file = "extent_alloc 3151384 534926060 3164945 476294498
abt 0 0 0 0
blk_map 353485831 851213330 9893230 6817550 6030586 1560908330 0
bmbt 0 0 0 0
dir 5970443 6945777 6874500 1854279411
trans 39942 3767813357 952
ig 7083866 4980513 0 2103353 0 1340957 3563694470
log 30274117 2045001872 23068 2926274 1401173
push_ail 3772005417 0 347558251 47834761 0 10641092 1400 928650954 0 572818
xstrat 2700722 0
rw 654716312 3877049901
attr 4264349303 6452433 454032 7692
icluster 0 41315812 1121686510
vnodes 762396 0 0 0 6315567 6315567 6315567 0
buf 2125016617 439870 2124576784 214995 432462 439837 0 568672 35579
abtb2 8079740 64538375 1770206 1765154 164 160 17096 10764 5314 4577 197 180 361 340 483092775
abtc2 16641798 135205710 5522173 5517194 236 232 17074 5466 4602 6952 280 264 516 496 1165831063
bmbt2 1170218 8837872 213928 214306 0 0 2153 2180 2381 2153 38 32 38 32 938847
ibt2 12712723 59099644 10126 9119 4 2 3 0 235 1 7 2 11 4 409636
fibt2 17438004 37870019 1700811 1700870 0 0 0 0 0 0 0 0 0 0 18621882
rmapbt 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
refcntbt 220 12 2 2 0 0 0 0 0 0 0 0 0 0 0
qm 0 0 0 0 0 0 0 0 0
defer_relog 0
debug 0";
        let result = ProcFsXfsStat::parse_proc_fs_xfs_stat(&proc_fs_xfs_stat_file);
        assert_eq!(
            result,
            ProcFsXfsStat {
                xs_write_calls: Some(654716312),
                xs_read_calls: Some(3877049901),
                xs_write_bytes: None,
                xs_read_bytes: None,
            }
        );
        Ok(())
    }
    /*
    #[test]
    fn parse_corrupted_loadavg_line_wrong_entry() -> Result<(), ProcSysParserError> {
        let loadavg_line = format!("AAA 0.19 0.13 1/161 7\n");
        //let result = ProcLoadavg::parse_proc_loadavg(&loadavg_line)?;
        let result = ProcLoadavg::parse_proc_loadavg(&loadavg_line);
        Ok(assert!(result.is_err()))
    }
    */
}
