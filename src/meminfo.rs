/*!
Read data from `/proc/meminfo` into the struct [`ProcMemInfo`].

The processor of `/proc/meminfo` takes the values for the memory areas specified, and puts them in the
struct [`ProcMemInfo`]. The values are in kilobytes (kB), just like the values in the original `/proc/meminfo`
file.

Here is an example obtaining the data from `/proc/meminfo`:
```no_run
use proc_sys_parser::{meminfo, meminfo::ProcMemInfo};

let proc_meminfo = meminfo::read();

println!("{:#?}", proc_meminfo);
```
Example output:
```text
ProcMemInfo {
    memtotal: 3997876,
    memfree: 2415136,
    memavailable: 3654096,
    buffers: 37492,
    cached: 1305568,
    swapcached: 0,
    active: 880772,
    inactive: 549432,
    active_anon: 86968,
    inactive_anon: 5196,
    active_file: 793804,
    inactive_file: 544236,
    unevictable: 4000,
    mlocked: 0,
    swaptotal: 0,
    swapfree: 0,
    zswap: 0,
    zswapped: 0,
    dirty: 0,
    writeback: 0,
    anonpages: 91144,
    mapped: 140948,
    shmem: 5020,
    kreclaimable: 56680,
    slab: 93916,
    sreclaimable: 56680,
    sunreclaim: 37236,
    kernelstack: 3256,
    shadowcallstack: 828,
    pagetables: 2884,
    secpagetables: 0,
    nfs_unstable: 0,
    bounce: 0,
    writebacktmp: 0,
    commitlimit: 1998936,
    committed_as: 944240,
    vmalloctotal: 133141626880,
    vmallocused: 14124,
    vmallocchunk: 0,
    percpu: 2280,
    hardwarecorrupted: 0,
    anonhugepages: 4096,
    shmemhugepages: 0,
    shmempmdmapped: 0,
    filehugepages: 0,
    filepmdmapped: 0,
    cmatotal: 32768,
    cmafree: 31232,
    hugepages_total: 0,
    hugepages_free: 0,
    hugepages_rsvd: 0,
    hugepages_surp: 0,
    hugepagesize: 2048,
    hugetlb: 0
}
```
(edited for readability)

If you want to change the path and/or file that is read for [`ProcMemInfo`], which is `/proc/meminfo`
by default, use:
```no_run
use proc_sys_parser::{meminfo, meminfo::{ProcMemInfo, Builder}};

let proc_meminfo = Builder::new().path("/myproc").read();
```

*/
use std::fs::read_to_string;
use crate::ProcSysParserError;
use log::warn;

/// Struct for holding `/proc/meminfo` statistics
#[derive(Debug, PartialEq, Default)]
pub struct ProcMemInfo {
    pub memtotal: u64,
    pub memfree: u64,
    pub memavailable: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swapcached: u64,
    pub active: u64,
    pub inactive: u64,
    pub active_anon: u64,
    pub inactive_anon: u64,
    pub active_file: u64,
    pub inactive_file: u64,
    pub unevictable: u64,
    pub mlocked: u64,
    pub swaptotal: u64,
    pub swapfree: u64,
    pub zswap: u64,
    pub zswapped: u64,
    pub dirty: u64,
    pub writeback: u64,
    pub anonpages: u64,
    pub mapped: u64,
    pub shmem: u64,
    pub kreclaimable: u64,
    pub slab: u64,
    pub sreclaimable: u64,
    pub sunreclaim: u64,
    pub kernelstack: u64,
    pub shadowcallstack: u64,
    pub pagetables: u64,
    pub secpagetables: u64,
    pub nfs_unstable: u64,
    pub bounce: u64,
    pub writebacktmp: u64,
    pub commitlimit: u64,
    pub committed_as: u64,
    pub vmalloctotal: u64,
    pub vmallocused: u64,
    pub vmallocchunk: u64,
    pub percpu: u64,
    pub hardwarecorrupted: u64,
    pub anonhugepages: u64,
    pub shmemhugepages: u64,
    pub shmempmdmapped: u64,
    pub filehugepages: u64,
    pub filepmdmapped: u64,
    pub cmatotal: u64,
    pub cmafree: u64,
    pub hugepages_total: u64,
    pub hugepages_free: u64,
    pub hugepages_rsvd: u64,
    pub hugepages_surp: u64,
    pub hugepagesize: u64,
    pub hugetlb: u64,
    pub directmap4k: Option<u64>,
    pub directmap2m: Option<u64>,
}

/// Builder pattern for [`ProcMemInfo`]
#[derive(Default)]
pub struct Builder {
    pub proc_path : String,
    pub proc_file : String,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            proc_path: "/proc".to_string(),
            proc_file: "meminfo".to_string(),
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
    pub fn read(self) -> Result<ProcMemInfo, ProcSysParserError> {
        ProcMemInfo::read_proc_meminfo(format!("{}/{}", &self.proc_path, &self.proc_file).as_str())
    }
}

/// The main function for building a [`ProcMemInfo`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> Result<ProcMemInfo, ProcSysParserError> {
    Builder::new().read()
}

impl ProcMemInfo {
    pub fn new() -> ProcMemInfo {
        ProcMemInfo::default()
    }
    pub fn parse_proc_meminfo_output(proc_meminfo: &str) -> Result<ProcMemInfo, ProcSysParserError> {
        let mut procmeminfo = ProcMemInfo::new();
        for line in proc_meminfo.lines() {
            match line {
                line if line.starts_with("MemTotal:") => {
                    procmeminfo.memtotal = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("MemFree:") => {
                    procmeminfo.memfree = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("MemAvailable:") => {
                    procmeminfo.memavailable = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Buffers:") => {
                    procmeminfo.buffers = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Cached:") => {
                    procmeminfo.cached = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("SwapCached:") => {
                    procmeminfo.swapcached = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Active:") => {
                    procmeminfo.active = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Inactive:") => {
                    procmeminfo.inactive = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Active(anon):") => {
                    procmeminfo.active_anon = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Inactive(anon):") => {
                    procmeminfo.inactive_anon = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Active(file):") => {
                    procmeminfo.active_file = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Inactive(file):") => {
                    procmeminfo.inactive_file = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Unevictable:") => {
                    procmeminfo.unevictable = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Mlocked:") => {
                    procmeminfo.mlocked = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("SwapTotal:") => {
                    procmeminfo.swaptotal = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("SwapFree:") => {
                    procmeminfo.swapfree = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Zswap:") => {
                    procmeminfo.zswap = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Zswapped:") => {
                    procmeminfo.zswapped = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Dirty:") => {
                    procmeminfo.dirty = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Writeback:") => {
                    procmeminfo.writeback = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("AnonPages:") => {
                    procmeminfo.anonpages = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Mapped:") => {
                    procmeminfo.mapped = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Shmem:") => {
                    procmeminfo.shmem = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("KReclaimable:") => {
                    procmeminfo.kreclaimable = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Slab:") => {
                    procmeminfo.slab = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("SReclaimable:") => {
                    procmeminfo.sreclaimable = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("SUnreclaim:") => {
                    procmeminfo.sunreclaim = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("KernelStack:") => {
                    procmeminfo.kernelstack = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("ShadowCallStack:") => {
                    procmeminfo.shadowcallstack = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("PageTables:") => {
                    procmeminfo.pagetables = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("SecPageTables:") => {
                    procmeminfo.secpagetables = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("NFS_Unstable:") => {
                    procmeminfo.nfs_unstable = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Bounce:") => {
                    procmeminfo.bounce = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("WritebackTmp:") => {
                    procmeminfo.writebacktmp = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("CommitLimit:") => {
                    procmeminfo.commitlimit = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Committed_AS:") => {
                    procmeminfo.committed_as = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("VmallocTotal:") => {
                    procmeminfo.vmalloctotal = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("VmallocUsed:") => {
                    procmeminfo.vmallocused = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("VmallocChunk:") => {
                    procmeminfo.vmallocchunk = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Percpu:") => {
                    procmeminfo.percpu = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("HardwareCorrupted:") => {
                    procmeminfo.hardwarecorrupted = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("AnonHugePages:") => {
                    procmeminfo.anonhugepages = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("ShmemHugePages:") => {
                    procmeminfo.shmemhugepages = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("ShmemPmdMapped:") => {
                    procmeminfo.shmempmdmapped = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("FileHugePages:") => {
                    procmeminfo.filehugepages = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("FilePmdMapped:") => {
                    procmeminfo.filepmdmapped = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("CmaTotal:") => {
                    procmeminfo.cmatotal = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("CmaFree:") => {
                    procmeminfo.cmafree = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("HugePages_Total:") => {
                    procmeminfo.hugepages_total = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("HugePages_Free:") => {
                    procmeminfo.hugepages_free = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("HugePages_Rsvd:") => {
                    procmeminfo.hugepages_rsvd = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("HugePages_Surp:") => {
                    procmeminfo.hugepages_surp = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Hugepagesize:") => {
                    procmeminfo.hugepagesize = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                line if line.starts_with("Hugetlb:") => {
                    procmeminfo.hugetlb = ProcMemInfo::parse_proc_meminfo_line(line)?
                }
                // Found on EL7
                line if line.starts_with("DirectMap4k:") => {
                    procmeminfo.directmap4k = Some(ProcMemInfo::parse_proc_meminfo_line(line)?)
                }
                // Found on EL7
                line if line.starts_with("DirectMap2M:") => {
                    procmeminfo.directmap2m = Some(ProcMemInfo::parse_proc_meminfo_line(line)?)
                }
                _ => warn!("meminfo: unknown entry found: {}", line),
            }
        }
        Ok(procmeminfo)
    }
    fn parse_proc_meminfo_line(proc_meminfo_line: &str) -> Result<u64, ProcSysParserError> {
        Ok(proc_meminfo_line
            .split_whitespace()
            .nth(1)
            .ok_or(ProcSysParserError::IteratorItemError {item: "meminfo parse_proc_meminfo".to_string() })?
            .parse::<u64>().map_err(|error| ProcSysParserError::ParseToIntegerError(error))?)
    }
    pub fn read_proc_meminfo(proc_meminfo_file: &str) -> Result<ProcMemInfo, ProcSysParserError> {
        let proc_meminfo_output = read_to_string(proc_meminfo_file)
            .map_err(|error| ProcSysParserError::FileReadError { file: proc_meminfo_file.to_string(), error })?;
        ProcMemInfo::parse_proc_meminfo_output(&proc_meminfo_output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::fs::{create_dir_all, remove_dir_all, write};

    #[test]
    fn parse_meminfo_line() {
        let meminfo_line = "MemTotal:        3997876 kB";
        let result = ProcMemInfo::parse_proc_meminfo_line(&meminfo_line).unwrap();
        assert_eq!(result, 3997876_u64);
    }

    #[test]
    fn parse_full_proc_meminfo_file() {
        let proc_meminfo = "MemTotal:        3997876 kB
MemFree:         2415136 kB
MemAvailable:    3654096 kB
Buffers:           37492 kB
Cached:          1305568 kB
SwapCached:            0 kB
Active:           880772 kB
Inactive:         549432 kB
Active(anon):      86968 kB
Inactive(anon):     5196 kB
Active(file):     793804 kB
Inactive(file):   544236 kB
Unevictable:        4000 kB
Mlocked:               0 kB
SwapTotal:             0 kB
SwapFree:              0 kB
Zswap:                 0 kB
Zswapped:              0 kB
Dirty:                 0 kB
Writeback:             0 kB
AnonPages:         91144 kB
Mapped:           140948 kB
Shmem:              5020 kB
KReclaimable:      56680 kB
Slab:              93916 kB
SReclaimable:      56680 kB
SUnreclaim:        37236 kB
KernelStack:        3256 kB
ShadowCallStack:     828 kB
PageTables:         2884 kB
SecPageTables:         0 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:     1998936 kB
Committed_AS:     944240 kB
VmallocTotal:   133141626880 kB
VmallocUsed:       14124 kB
VmallocChunk:          0 kB
Percpu:             2280 kB
HardwareCorrupted:     0 kB
AnonHugePages:      4096 kB
ShmemHugePages:        0 kB
ShmemPmdMapped:        0 kB
FileHugePages:         0 kB
FilePmdMapped:         0 kB
CmaTotal:          32768 kB
CmaFree:           31232 kB
HugePages_Total:       0
HugePages_Free:        0
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:       2048 kB
Hugetlb:               0 kB";
        let result = ProcMemInfo::parse_proc_meminfo_output(proc_meminfo).unwrap();
        assert_eq!(
            result,
            ProcMemInfo {
                memtotal: 3997876,
                memfree: 2415136,
                memavailable: 3654096,
                buffers: 37492,
                cached: 1305568,
                swapcached: 0,
                active: 880772,
                inactive: 549432,
                active_anon: 86968,
                inactive_anon: 5196,
                active_file: 793804,
                inactive_file: 544236,
                unevictable: 4000,
                mlocked: 0,
                swaptotal: 0,
                swapfree: 0,
                zswap: 0,
                zswapped: 0,
                dirty: 0,
                writeback: 0,
                anonpages: 91144,
                mapped: 140948,
                shmem: 5020,
                kreclaimable: 56680,
                slab: 93916,
                sreclaimable: 56680,
                sunreclaim: 37236,
                kernelstack: 3256,
                shadowcallstack: 828,
                pagetables: 2884,
                secpagetables: 0,
                nfs_unstable: 0,
                bounce: 0,
                writebacktmp: 0,
                commitlimit: 1998936,
                committed_as: 944240,
                vmalloctotal: 133141626880,
                vmallocused: 14124,
                vmallocchunk: 0,
                percpu: 2280,
                hardwarecorrupted: 0,
                anonhugepages: 4096,
                shmemhugepages: 0,
                shmempmdmapped: 0,
                filehugepages: 0,
                filepmdmapped: 0,
                cmatotal: 32768,
                cmafree: 31232,
                hugepages_total: 0,
                hugepages_free: 0,
                hugepages_rsvd: 0,
                hugepages_surp: 0,
                hugepagesize: 2048,
                hugetlb: 0,
                directmap4k: None,
                directmap2m: None,
            }
        );
    }

    #[test]
    fn create_proc_meminfo_file_and_read() {
        let proc_meminfo = "MemTotal:        3997876 kB
MemFree:         2415136 kB
MemAvailable:    3654096 kB
Buffers:           37492 kB
Cached:          1305568 kB
SwapCached:            0 kB
Active:           880772 kB
Inactive:         549432 kB
Active(anon):      86968 kB
Inactive(anon):     5196 kB
Active(file):     793804 kB
Inactive(file):   544236 kB
Unevictable:        4000 kB
Mlocked:               0 kB
SwapTotal:             0 kB
SwapFree:              0 kB
Zswap:                 0 kB
Zswapped:              0 kB
Dirty:                 0 kB
Writeback:             0 kB
AnonPages:         91144 kB
Mapped:           140948 kB
Shmem:              5020 kB
KReclaimable:      56680 kB
Slab:              93916 kB
SReclaimable:      56680 kB
SUnreclaim:        37236 kB
KernelStack:        3256 kB
ShadowCallStack:     828 kB
PageTables:         2884 kB
SecPageTables:         0 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:     1998936 kB
Committed_AS:     944240 kB
VmallocTotal:   133141626880 kB
VmallocUsed:       14124 kB
VmallocChunk:          0 kB
Percpu:             2280 kB
HardwareCorrupted:     0 kB
AnonHugePages:      4096 kB
ShmemHugePages:        0 kB
ShmemPmdMapped:        0 kB
FileHugePages:         0 kB
FilePmdMapped:         0 kB
CmaTotal:          32768 kB
CmaFree:           31232 kB
HugePages_Total:       0
HugePages_Free:        0
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:       2048 kB
Hugetlb:               0 kB";
        let directory_suffix: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        create_dir_all(format!("{}", test_path)).expect("Error creating mock directory.");

        write(format!("{}/meminfo", test_path), proc_meminfo)
            .expect(format!("Error writing to {}/meminfo", test_path).as_str());
        let result = Builder::new().path(&test_path).read().unwrap();

        remove_dir_all(test_path).unwrap();

        assert_eq!(
            result,
            ProcMemInfo {
                memtotal: 3997876,
                memfree: 2415136,
                memavailable: 3654096,
                buffers: 37492,
                cached: 1305568,
                swapcached: 0,
                active: 880772,
                inactive: 549432,
                active_anon: 86968,
                inactive_anon: 5196,
                active_file: 793804,
                inactive_file: 544236,
                unevictable: 4000,
                mlocked: 0,
                swaptotal: 0,
                swapfree: 0,
                zswap: 0,
                zswapped: 0,
                dirty: 0,
                writeback: 0,
                anonpages: 91144,
                mapped: 140948,
                shmem: 5020,
                kreclaimable: 56680,
                slab: 93916,
                sreclaimable: 56680,
                sunreclaim: 37236,
                kernelstack: 3256,
                shadowcallstack: 828,
                pagetables: 2884,
                secpagetables: 0,
                nfs_unstable: 0,
                bounce: 0,
                writebacktmp: 0,
                commitlimit: 1998936,
                committed_as: 944240,
                vmalloctotal: 133141626880,
                vmallocused: 14124,
                vmallocchunk: 0,
                percpu: 2280,
                hardwarecorrupted: 0,
                anonhugepages: 4096,
                shmemhugepages: 0,
                shmempmdmapped: 0,
                filehugepages: 0,
                filepmdmapped: 0,
                cmatotal: 32768,
                cmafree: 31232,
                hugepages_total: 0,
                hugepages_free: 0,
                hugepages_rsvd: 0,
                hugepages_surp: 0,
                hugepagesize: 2048,
                hugetlb: 0,
                directmap4k: None,
                directmap2m: None,
            }
        );
    }
}
