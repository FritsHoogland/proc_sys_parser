/*!
Read data from `/proc/vmstat` into the struct [`ProcVmStat`].

The processor of `/proc/vmstat` takes the values for the memory areas specified, and puts them in the
struct [`ProcVmStat`]. The values are in kilobytes (kB), just like the values in the original `/proc/vmstat`
file.

The file `/proc/vmstat` has no absolute documentation.

- <https://github.com/torvalds/linux/blob/master/mm/vmstat.c>
-

Here is an example obtaining the data from `/proc/vmstat`:
```no_run
use proc_sys_parser::{vmstat, vmstat::ProcVmStat};

let proc_vmstat = vmstat::read();

println!("{:#?}", proc_vmstat);
```
Example output:
```text
ProcVmStat {
    nr_free_pages: 778263,
    nr_zone_inactive_anon: 212,
    nr_zone_active_anon: 21214,
    nr_zone_inactive_file: 86210,
    nr_zone_active_file: 85676,
    nr_zone_unevictable: 0,
    nr_zone_write_pending: 1,
    nr_mlock: 0,
    nr_bounce: 0,
    nr_zspages: 0,
    nr_free_cma: 7808,
    numa_hit: 40773813,
    numa_miss: 0,
    numa_foreign: 0,
    numa_interleave: 1212,
    numa_local: 40773813,
    numa_other: 0,
    nr_inactive_anon: 212,
    nr_active_anon: 21214,
    nr_inactive_file: 86210,
    nr_active_file: 85676,
    nr_unevictable: 0,
    nr_slab_reclaimable: 8551,
    nr_slab_unreclaimable: 8749,
    nr_isolated_anon: 0,
    nr_isolated_file: 0,
    workingset_nodes: 0,
    workingset_refault_anon: 0,
    workingset_refault_file: 0,
    workingset_activate_anon: 0,
    workingset_activate_file: 0,
    workingset_restore_anon: 0,
    workingset_restore_file: 0,
    workingset_nodereclaim: 0,
    nr_anon_pages: 21233,
    nr_mapped: 33359,
    nr_file_pages: 0,
    nr_dirty: 93023,
    nr_writeback: 0,
    nr_writeback_temp: 0,
    nr_shmem: 0,
    nr_shmem_hugepages: 0,
    nr_shmem_pmdmapped: 0,
    nr_file_hugepages: 0,
    nr_file_pmdmapped: 0,
    nr_anon_transparent_hugepages: 0,
    nr_vmscan_write: 0,
    nr_vmscan_immediate_reclaim: 0,
    nr_dirtied: 66050,
    nr_written: 62014,
    nr_throttled_written: 0,
    nr_kernel_misc_reclaimable: 0,
    nr_foll_pin_acquired: 0,
    nr_foll_pin_released: 0,
    nr_kernel_stack: 2768,
    nr_shadow_call_stack: 712,
    nr_page_table_pages: 580,
    nr_sec_page_table_pages: 0,
    nr_swapcached: 0,
    pgpromote_success: 0,
    pgpromote_candidate: 0,
    nr_dirty_threshold: 0,
    nr_dirty_background_threshold: 0,
    pgpgin: 569048,
    pgpgout: 264157,
    pswpin: 0,
    pswpout: 0,
    pgalloc_dma: 0,
    pgalloc_dma32: 0,
    pgalloc_normal: 42962188,
    pgalloc_movable: 0,
    pgalloc_device: 0,
    allocstall_dma: 0,
    allocstall_dma32: 0,
    allocstall_normal: 0,
    allocstall_movable: 0,
    allocstall_device: 0,
    pgskip_dma: 0,
    pgskip_dma32: 0,
    pgskip_normal: 0,
    pgskip_movable: 0,
    pgskip_device: 0,
    pgfree: 43741863,
    pgactivate: 0,
    pgdeactivate: 0,
    pglazyfree: 0,
    pgfault: 55051790,
    pgmajfault: 2851,
    pglazyfreed: 0,
    pgrefill: 0,
    pgreuse: 1854584,
    pgsteal_kswapd: 0,
    pgsteal_direct: 0,
    pgsteal_khugepaged: 0,
    pgdemote_kswapd: 0,
    pgdemote_direct: 0,
    pgdemote_khugepaged: 0,
    pgscan_kswapd: 0,
    pgscan_direct: 0,
    pgscan_khugepaged: 0,
    pgscan_direct_throttle: 0,
    pgscan_anon: 0,
    pgscan_file: 0,
    pgsteal_anon: 0,
    pgsteal_file: 0,
    zone_reclaim_failed: 0,
    pginodesteal: 0,
    slabs_scanned: 0,
    kswapd_inodesteal: 0,
    kswapd_low_wmark_hit_quickly: 0,
    kswapd_high_wmark_hit_quickly: 0,
    pageoutrun: 0,
    pgrotated: 6,
    drop_pagecache: 0,
    drop_slab: 0,
    oom_kill: 0,
    numa_pte_updates: 0,
    numa_huge_pte_updates: 0,
    numa_hint_faults: 0,
    numa_hint_faults_local: 0,
    numa_pages_migrated: 0,
    pgmigrate_success: 0,
    pgmigrate_fail: 0,
    thp_migration_success: 0,
    thp_migration_fail: 0,
    thp_migration_split: 0,
    compact_migrate_scanned: 0,
    compact_free_scanned: 0,
    compact_isolated: 896,
    compact_stall: 0,
    compact_fail: 0,
    compact_success: 0,
    compact_daemon_wake: 0,
    compact_daemon_migrate_scanned: 0,
    compact_daemon_free_scanned: 0,
    htlb_buddy_alloc_success: 0,
    htlb_buddy_alloc_fail: 0,
    cma_alloc_success: 3,
    cma_alloc_fail: 0,
    unevictable_pgs_culled: 0,
    unevictable_pgs_scanned: 0,
    unevictable_pgs_rescued: 0,
    unevictable_pgs_mlocked: 0,
    unevictable_pgs_munlocked: 0,
    unevictable_pgs_cleared: 0,
    unevictable_pgs_stranded: 0,
    thp_fault_alloc: 0,
    thp_fault_fallback: 0,
    thp_fault_fallback_charge: 0,
    thp_collapse_alloc: 0,
    thp_collapse_alloc_failed: 0,
    thp_file_alloc: 0,
    thp_file_fallback: 0,
    thp_file_fallback_charge: 0,
    thp_file_mapped: 0,
    thp_split_page: 0,
    thp_split_page_failed: 0,
    thp_deferred_split_page: 0,
    thp_split_pmd: 0,
    thp_scan_exceed_none_pte: 0,
    thp_scan_exceed_swap_pte: 0,
    thp_scan_exceed_share_pte: 0,
    thp_zero_page_alloc: 0,
    thp_zero_page_alloc_failed: 0,
    thp_swpout: 0,
    thp_swpout_fallback: 0,
    balloon_inflate: 0,
    balloon_deflate: 0,
    balloon_migrate: 0,
    swap_ra: 0,
    swap_ra_hit: 0,
    ksm_swpin_copy: 0,
    cow_ksm: 0,
    zswpin: 0,
    zswpout: 0,
    nr_unstable: 0,
}
```
(edited for readability)

If you want to change the path and/or file that is read for [`ProcVmStat`], which is `/proc/vmstat`
by default, use:
```no_run
use proc_sys_parser::{vmstat, vmstat::{ProcVmStat, Builder}};

let proc_vmstat = Builder::new().file_name("/myproc/vmstat").read();
```
*/
use std::fs::read_to_string;
use log::warn;

/// Struct for holding `/proc/vmstat` statistics
#[derive(Debug, PartialEq, Default)]
pub struct ProcVmStat {
    /// absolute number: number of pages free
    pub nr_free_pages: u64,
    pub nr_zone_inactive_anon: u64,
    pub nr_zone_active_anon: u64,
    pub nr_zone_inactive_file: u64,
    pub nr_zone_active_file: u64,
    pub nr_zone_unevictable: u64,
    pub nr_zone_write_pending: u64,
    /// absolute number: number of pages mlocked
    pub nr_mlock: u64,
    /// absolue number: number of pages as bounce buffers
    pub nr_bounce: u64,
    pub nr_zspages: u64,
    pub nr_free_cma: u64,
    pub numa_hit: u64,
    pub numa_miss: u64,
    pub numa_foreign: u64,
    pub numa_interleave: u64,
    pub numa_local: u64,
    pub numa_other: u64,
    /// absolute number: the number of anonymous pages considered inactive
    /// inactive means not considered to be currently in use
    pub nr_inactive_anon: u64,
    /// absolute number: the number of anonymous pages considered active
    /// active means considered to be currently in use
    pub nr_active_anon: u64,
    /// absolute number: the number of pages containing file data considered inactive
    /// inactive means not considered to be currently in use
    pub nr_inactive_file: u64,
    /// absolute number: the number of pages containing file data considered active
    /// active means considered to be currently in use
    pub nr_active_file: u64,
    /// absolute number: the number of pages that cannot be paged out
    /// common reasons are: belonging to a ramdisk, protected by mlock(), shared and locked, or
    /// other reasons for the kernel not to allow paging out.
    pub nr_unevictable: u64,
    pub nr_slab_reclaimable: u64,
    pub nr_slab_unreclaimable: u64,
    pub nr_isolated_anon: u64,
    pub nr_isolated_file: u64,
    pub workingset_nodes: Option<u64>,
    pub workingset_refault_anon: Option<u64>,
    pub workingset_refault_file: Option<u64>,
    pub workingset_activate_anon: Option<u64>,
    pub workingset_activate_file: Option<u64>,
    pub workingset_restore_anon: Option<u64>,
    pub workingset_restore_file: Option<u64>,
    pub workingset_nodereclaim: u64,
    /// absolute number: number of anonymous memory pages
    pub nr_anon_pages: u64,
    /// absolute number: number of mapped memory pages
    pub nr_mapped: u64,
    /// absolute number: the number of pages containing file data
    /// (nr_inactive_file, nr_active_file, nr_shmem, ...?)
    pub nr_file_pages: u64,
    /// absolute number: the number of (file) pages changed and waiting to be written out to disk
    pub nr_dirty: u64,
    /// absolute number: the number of (file) pages that are actively being written to disk
    /// (this lowers nr_dirty)
    pub nr_writeback: u64,
    /// absolute number: the number of (file) pages that are actively being written to disk using
    /// temporary buffers. Used by fuse(?)
    pub nr_writeback_temp: u64,
    /// absolute number: the number of smallpages shared memory pages
    pub nr_shmem: u64,
    pub nr_shmem_hugepages: u64,
    pub nr_shmem_pmdmapped: u64,
    pub nr_file_hugepages: Option<u64>,
    pub nr_file_pmdmapped: Option<u64>,
    pub nr_anon_transparent_hugepages: u64,
    pub nr_vmscan_write: u64,
    pub nr_vmscan_immediate_reclaim: u64,
    pub nr_dirtied: u64,
    pub nr_written: u64,
    pub nr_throttled_written: Option<u64>,
    pub nr_kernel_misc_reclaimable: Option<u64>,
    pub nr_foll_pin_acquired: Option<u64>,
    pub nr_foll_pin_released: Option<u64>,
    pub nr_kernel_stack: u64,
    pub nr_shadow_call_stack: Option<u64>,
    /// absolute number: number of pages used for pagetables
    pub nr_page_table_pages: u64,
    pub nr_sec_page_table_pages: Option<u64>,
    pub nr_swapcached: Option<u64>,
    pub pgpromote_success: Option<u64>,
    pub pgpromote_candidate: Option<u64>,
    /// absolute number: the current number of pages used as dirty threshold by the kernel
    pub nr_dirty_threshold: u64,
    /// absolute number: the current number of pages used as dirty background threshold by the
    /// kernel
    pub nr_dirty_background_threshold: u64,
    /// counter: the number of kilobytes paged in (read) from disk
    pub pgpgin: u64,
    /// counter: the number of kilobytes paged out (written) to disk
    pub pgpgout: u64,
    /// counter: the number of pages swapped in (read back from swap device)
    pub pswpin: u64,
    /// counter: the number of pages swapped out (written to swap device)
    pub pswpout: u64,
    /// counter: the number of page allocations in dma memory
    pub pgalloc_dma: u64,
    /// counter: the number of page allocations in dma32 memory
    pub pgalloc_dma32: u64,
    /// counter: the number of page allocations in normal memory
    pub pgalloc_normal: u64,
    /// counter: the number of page allocations in movable memory
    pub pgalloc_movable: u64,
    /// counter: the number of page allocations in device memory
    pub pgalloc_device: Option<u64>,
    pub allocstall_dma: u64,
    pub allocstall_dma32: u64,
    pub allocstall_normal: u64,
    pub allocstall_movable: u64,
    pub allocstall_device: Option<u64>,
    pub pgskip_dma: u64,
    pub pgskip_dma32: u64,
    pub pgskip_normal: u64,
    pub pgskip_movable: u64,
    pub pgskip_device: Option<u64>,
    /// counter: the number of pages placed in the freelist
    pub pgfree: u64,
    /// counter: the number of pages moved from inactive -> active
    pub pgactivate: u64,
    /// counter: the number of pages moved from active -> inactive
    pub pgdeactivate: u64,
    pub pglazyfree: u64,
    pub pglazyfreed: u64,
    /// counter: the number of page faults
    pub pgfault: u64,
    /// counter: the number of page faults requiring a disk read
    pub pgmajfault: u64,
    /// counter: the number of scanned pages in an active LRU list
    pub pgrefill: u64,
    pub pgreuse: Option<u64>,
    /// counter: the number of pages reclaimed from the pagecache and swapcache by kswapd
    pub pgsteal_kswapd: u64,
    /// counter: the number of pages reclaimed from by the pagecache and swapcache by user tasks
    pub pgsteal_direct: u64,
    /// counter: the number of pages reclaimed from the pagecache and swapcache by khugepaged
    pub pgsteal_khugepaged: Option<u64>,
    pub pgdemote_kswapd: Option<u64>,
    pub pgdemote_direct: Option<u64>,
    pub pgdemote_khugepaged: Option<u64>,
    /// counter: the number of pages scanned by kswapd
    pub pgscan_kswapd: u64,
    /// counter: the number of pages scanned by user tasks
    pub pgscan_direct: u64,
    /// counter: the number of pages scanned by khugepagd
    pub pgscan_khugepaged: Option<u64>,
    /// counter: the number of occurences that direct reclaimers (user tasks) get throttled 
    /// This means they get stalled. Suggested solution is increasing vm.min_free_kbytes.
    pub pgscan_direct_throttle: u64,
    /// counter: the number of pages scanned from anonymous memory
    pub pgscan_anon: Option<u64>,
    /// counter: the number of pages scanned from file backed memory
    pub pgscan_file: Option<u64>,
    /// counter: the number of pages reclaimed from anonymous memory
    pub pgsteal_anon: Option<u64>,
    /// counter: the number of pages reclaimed from file backed memory
    pub pgsteal_file: Option<u64>,
    pub zone_reclaim_failed: u64,
    /// counter: the number of pages reclaimed via inode freeing
    pub pginodesteal: u64,
    pub slabs_scanned: Option<u64>,
    /// counter: the number of pages reclaimed by kswapd via inode freeing
    pub kswapd_inodesteal: u64,
    pub kswapd_low_wmark_hit_quickly: u64,
    pub kswapd_high_wmark_hit_quickly: u64,
    pub pageoutrun: u64,
    /// counter: the number of pages rotated to the tail of the LRU
    pub pgrotated: u64,
    /// counter: the number of requests for dropping the page cache
    pub drop_pagecache: u64,
    /// counter: the number of requests for dropping the slab cache
    pub drop_slab: u64,
    /// counter: the number of occurences of the kernel invoking the OOM killer
    pub oom_kill: u64,
    pub numa_pte_updates: u64,
    pub numa_huge_pte_updates: u64,
    pub numa_hint_faults: u64,
    pub numa_hint_faults_local: u64,
    pub numa_pages_migrated: u64,
    pub pgmigrate_success: u64,
    pub pgmigrate_fail: u64,
    pub thp_migration_success: Option<u64>,
    pub thp_migration_fail: Option<u64>,
    pub thp_migration_split: Option<u64>,
    pub compact_migrate_scanned: u64,
    pub compact_free_scanned: u64,
    pub compact_isolated: u64,
    pub compact_stall: u64,
    pub compact_fail: u64,
    pub compact_success: u64,
    pub compact_daemon_wake: u64,
    pub compact_daemon_migrate_scanned: u64,
    pub compact_daemon_free_scanned: u64,
    pub htlb_buddy_alloc_success: u64,
    pub htlb_buddy_alloc_fail: u64,
    pub cma_alloc_success: Option<u64>,
    pub cma_alloc_fail: Option<u64>,
    pub unevictable_pgs_culled: u64,
    pub unevictable_pgs_scanned: u64,
    pub unevictable_pgs_rescued: u64,
    pub unevictable_pgs_mlocked: u64,
    pub unevictable_pgs_munlocked: u64,
    pub unevictable_pgs_cleared: u64,
    pub unevictable_pgs_stranded: u64,
    /// counter: the number of transparent hugepages allocated to satisfy a page fault
    pub thp_fault_alloc: u64,
    pub thp_fault_fallback: u64,
    pub thp_fault_fallback_charge: Option<u64>,
    /// counter: the number of transparent hugepages allocated to allow collapsing an existing
    /// range of pages
    pub thp_collapse_alloc: u64,
    pub thp_collapse_alloc_failed: u64,
    pub thp_file_alloc: u64,
    pub thp_file_fallback: Option<u64>,
    pub thp_file_fallback_charge: Option<u64>,
    pub thp_file_mapped: u64,
    pub thp_split_page: u64,
    pub thp_split_page_failed: u64,
    pub thp_deferred_split_page: u64,
    pub thp_split_pmd: u64,
    pub thp_scan_exceed_none_pte: Option<u64>,
    pub thp_scan_exceed_swap_pte: Option<u64>,
    pub thp_scan_exceed_share_pte: Option<u64>,
    pub thp_zero_page_alloc: u64,
    pub thp_zero_page_alloc_failed: u64,
    /// counter: the number of transparent hugepages which are swapped out in one piece (wihtout
    /// splitting)
    pub thp_swpout: u64,
    /// clounter: the number of transparent hugepages which are split before swapout. 
    /// This usually happens because of the inability to allocate continuous swap space for the
    /// huge page.
    pub thp_swpout_fallback: u64,
    pub balloon_inflate: u64,
    pub balloon_deflate: u64,
    pub balloon_migrate: u64,
    pub swap_ra: u64,
    pub swap_ra_hit: u64,
    pub ksm_swpin_copy: Option<u64>,
    pub cow_ksm: Option<u64>,
    pub zswpin: Option<u64>,
    pub zswpout: Option<u64>,
    /// absolute number: number of NFS unstable pages.
    pub nr_unstable: u64,
}

/// Builder pattern for [`ProcVmStat`]
#[derive(Default)]
pub struct Builder {
    pub proc_vmstat_file: String,
}
impl Builder {
    pub fn new() -> Builder {
        Builder {
            proc_vmstat_file: "/proc/vmstat".to_string(),
        }
    }

    pub fn file_name(mut self, proc_vmstat_file: &str) -> Builder {
        self.proc_vmstat_file = proc_vmstat_file.to_string();
        self
    }
    pub fn read(self) -> ProcVmStat {
        ProcVmStat::read_proc_vmstat(&self.proc_vmstat_file)
    }
}

/// The main function for building a [`ProcVmStat`] struct with current data.
/// This uses the Builder pattern, which allows settings such as the filename to specified.
pub fn read() -> ProcVmStat {
    Builder::new().read()
}

impl ProcVmStat {
    pub fn new() -> Self {
        ProcVmStat::default()
    }
    pub fn parse_proc_vmstat_output(proc_vmstat: &str) -> ProcVmStat {
        let mut procvmstat = ProcVmStat::new();
        for line in proc_vmstat.lines() {
            let statistic = line.split_whitespace().next().unwrap();
            match statistic {
                "nr_free_pages" => {
                    procvmstat.nr_free_pages = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_zone_inactive_anon" => {
                    procvmstat.nr_zone_inactive_anon = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_zone_active_anon" => {
                    procvmstat.nr_zone_active_anon = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_zone_inactive_file" => {
                    procvmstat.nr_zone_inactive_file = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_zone_active_file" => {
                    procvmstat.nr_zone_active_file = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_zone_unevictable" => {
                    procvmstat.nr_zone_unevictable = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_zone_write_pending" => {
                    procvmstat.nr_zone_write_pending = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_mlock" => procvmstat.nr_mlock = ProcVmStat::parse_proc_vmstat_line(line),
                "nr_bounce" => procvmstat.nr_bounce = ProcVmStat::parse_proc_vmstat_line(line),
                "nr_zspages" => procvmstat.nr_zspages = ProcVmStat::parse_proc_vmstat_line(line),
                "nr_free_cma" => procvmstat.nr_free_cma = ProcVmStat::parse_proc_vmstat_line(line),
                "numa_hit" => procvmstat.numa_hit = ProcVmStat::parse_proc_vmstat_line(line),
                "numa_miss" => procvmstat.numa_miss = ProcVmStat::parse_proc_vmstat_line(line),
                "numa_foreign" => {
                    procvmstat.numa_foreign = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "numa_interleave" => {
                    procvmstat.numa_interleave = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "numa_local" => procvmstat.numa_local = ProcVmStat::parse_proc_vmstat_line(line),
                "numa_other" => procvmstat.numa_other = ProcVmStat::parse_proc_vmstat_line(line),
                "nr_inactive_anon" => {
                    procvmstat.nr_inactive_anon = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_active_anon" => {
                    procvmstat.nr_active_anon = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_inactive_file" => {
                    procvmstat.nr_inactive_file = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_active_file" => {
                    procvmstat.nr_active_file = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_unevictable" => {
                    procvmstat.nr_unevictable = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_slab_reclaimable" => {
                    procvmstat.nr_slab_reclaimable = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_slab_unreclaimable" => {
                    procvmstat.nr_slab_unreclaimable = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_isolated_anon" => {
                    procvmstat.nr_isolated_anon = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_isolated_file" => {
                    procvmstat.nr_isolated_file = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "workingset_nodes" => {
                    procvmstat.workingset_nodes = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "workingset_refault_anon" => {
                    procvmstat.workingset_refault_anon = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "workingset_refault_file" => {
                    procvmstat.workingset_refault_file = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "workingset_activate_anon" => {
                    procvmstat.workingset_activate_anon = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "workingset_activate_file" => {
                    procvmstat.workingset_activate_file = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "workingset_restore_file" => {
                    procvmstat.workingset_restore_file = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "workingset_restore_anon" => {
                    procvmstat.workingset_restore_anon = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "workingset_nodereclaim" => {
                    procvmstat.workingset_nodereclaim = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_anon_pages" => {
                    procvmstat.nr_anon_pages = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_mapped" => procvmstat.nr_mapped = ProcVmStat::parse_proc_vmstat_line(line),
                "nr_file_pages" => {
                    procvmstat.nr_file_pages = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_dirty" => procvmstat.nr_dirty = ProcVmStat::parse_proc_vmstat_line(line),
                "nr_writeback" => {
                    procvmstat.nr_writeback = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_writeback_temp" => {
                    procvmstat.nr_writeback_temp = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_shmem" => procvmstat.nr_shmem = ProcVmStat::parse_proc_vmstat_line(line),
                "nr_shmem_hugepages" => {
                    procvmstat.nr_shmem_hugepages = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_shmem_pmdmapped" => {
                    procvmstat.nr_shmem_pmdmapped = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_file_hugepages" => {
                    procvmstat.nr_file_hugepages = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "nr_file_pmdmapped" => {
                    procvmstat.nr_file_pmdmapped = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "nr_anon_transparent_hugepages" => {
                    procvmstat.nr_anon_transparent_hugepages =
                        ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_vmscan_write" => {
                    procvmstat.nr_vmscan_write = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_vmscan_immediate_reclaim" => {
                    procvmstat.nr_vmscan_immediate_reclaim =
                        ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_dirtied" => procvmstat.nr_dirtied = ProcVmStat::parse_proc_vmstat_line(line),
                "nr_written" => procvmstat.nr_written = ProcVmStat::parse_proc_vmstat_line(line),
                "nr_throttled_written" => {
                    procvmstat.nr_throttled_written = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "nr_kernel_misc_reclaimable" => {
                    procvmstat.nr_kernel_misc_reclaimable = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "nr_foll_pin_acquired" => {
                    procvmstat.nr_foll_pin_acquired = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "nr_foll_pin_released" => {
                    procvmstat.nr_foll_pin_released = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "nr_kernel_stack" => {
                    procvmstat.nr_kernel_stack = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_shadow_call_stack" => {
                    procvmstat.nr_shadow_call_stack = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "nr_page_table_pages" => {
                    procvmstat.nr_page_table_pages = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_sec_page_table_pages" => {
                    procvmstat.nr_sec_page_table_pages = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "nr_swapcached" => {
                    procvmstat.nr_swapcached = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "pgpromote_success" => {
                    procvmstat.pgpromote_success = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "pgpromote_candidate" => {
                    procvmstat.pgpromote_candidate = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "nr_dirty_threshold" => {
                    procvmstat.nr_dirty_threshold = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "nr_dirty_background_threshold" => {
                    procvmstat.nr_dirty_background_threshold =
                        ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgpgin" => procvmstat.pgpgin = ProcVmStat::parse_proc_vmstat_line(line),
                "pgpgout" => procvmstat.pgpgout = ProcVmStat::parse_proc_vmstat_line(line),
                "pswpin" => procvmstat.pswpin = ProcVmStat::parse_proc_vmstat_line(line),
                "pswpout" => procvmstat.pswpout = ProcVmStat::parse_proc_vmstat_line(line),
                "pgalloc_dma" => procvmstat.pgalloc_dma = ProcVmStat::parse_proc_vmstat_line(line),
                "pgalloc_dma32" => {
                    procvmstat.pgalloc_dma32 = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgalloc_normal" => {
                    procvmstat.pgalloc_normal = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgalloc_movable" => {
                    procvmstat.nr_file_pages = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgalloc_device" => {
                    procvmstat.pgalloc_device = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "allocstall_dma" => {
                    procvmstat.allocstall_dma = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "allocstall_dma32" => {
                    procvmstat.allocstall_dma32 = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "allocstall_normal" => {
                    procvmstat.allocstall_normal = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "allocstall_movable" => {
                    procvmstat.allocstall_movable = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "allocstall_device" => {
                    procvmstat.allocstall_device = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "pgskip_dma" => procvmstat.pgskip_dma = ProcVmStat::parse_proc_vmstat_line(line),
                "pgskip_dma32" => {
                    procvmstat.pgskip_dma32 = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgskip_normal" => {
                    procvmstat.pgskip_normal = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgskip_movable" => {
                    procvmstat.pgskip_movable = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgskip_device" => {
                    procvmstat.pgskip_device = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "pgfree" => procvmstat.pgfree = ProcVmStat::parse_proc_vmstat_line(line),
                "pgactivate" => procvmstat.pgactivate = ProcVmStat::parse_proc_vmstat_line(line),
                "pgdeactivate" => {
                    procvmstat.pgdeactivate = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pglazyfree" => procvmstat.pglazyfree = ProcVmStat::parse_proc_vmstat_line(line),
                "pgfault" => procvmstat.pgfault = ProcVmStat::parse_proc_vmstat_line(line),
                "pgmajfault" => procvmstat.pgmajfault = ProcVmStat::parse_proc_vmstat_line(line),
                "pglazyfreed" => procvmstat.pglazyfreed = ProcVmStat::parse_proc_vmstat_line(line),
                "pgrefill" => procvmstat.pgrefill = ProcVmStat::parse_proc_vmstat_line(line),
                "pgreuse" => procvmstat.pgreuse = ProcVmStat::parse_proc_vmstat_line_option(line),
                "pgsteal_kswapd" => {
                    procvmstat.pgsteal_kswapd = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgsteal_direct" => {
                    procvmstat.pgsteal_direct = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgsteal_khugepaged" => {
                    procvmstat.pgsteal_khugepaged = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "pgdemote_kswapd" => {
                    procvmstat.pgdemote_kswapd = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "pgdemote_direct" => {
                    procvmstat.pgdemote_direct = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "pgdemote_khugepaged" => {
                    procvmstat.pgdemote_khugepaged = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "pgscan_kswapd" => {
                    procvmstat.pgscan_kswapd = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgscan_direct" => {
                    procvmstat.pgscan_direct = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgscan_khugepaged" => {
                    procvmstat.pgscan_khugepaged = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "pgscan_direct_throttle" => {
                    procvmstat.pgscan_direct_throttle = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgscan_anon" => procvmstat.pgscan_anon = ProcVmStat::parse_proc_vmstat_line_option(line),
                "pgscan_file" => procvmstat.pgscan_file = ProcVmStat::parse_proc_vmstat_line_option(line),
                "pgsteal_anon" => {
                    procvmstat.pgsteal_anon = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "pgsteal_file" => {
                    procvmstat.pgsteal_file = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "zone_reclaim_failed" => {
                    procvmstat.zone_reclaim_failed = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pginodesteal" => {
                    procvmstat.pginodesteal = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "slabs_scanned" => {
                    procvmstat.slabs_scanned = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "kswapd_inodesteal" => {
                    procvmstat.kswapd_inodesteal = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "kswapd_low_wmark_hit_quickly" => {
                    procvmstat.kswapd_low_wmark_hit_quickly =
                        ProcVmStat::parse_proc_vmstat_line(line)
                }
                "kswapd_high_wmark_hit_quickly" => {
                    procvmstat.kswapd_high_wmark_hit_quickly =
                        ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pageoutrun" => procvmstat.pageoutrun = ProcVmStat::parse_proc_vmstat_line(line),
                "pgrotated" => procvmstat.pgrotated = ProcVmStat::parse_proc_vmstat_line(line),
                "drop_pagecache" => {
                    procvmstat.drop_pagecache = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "drop_slab" => procvmstat.drop_slab = ProcVmStat::parse_proc_vmstat_line(line),
                "oom_kill" => procvmstat.oom_kill = ProcVmStat::parse_proc_vmstat_line(line),
                "numa_pte_updates" => {
                    procvmstat.numa_pte_updates = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "numa_huge_pte_updates" => {
                    procvmstat.numa_huge_pte_updates = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "numa_hint_faults" => {
                    procvmstat.numa_hint_faults = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "numa_hint_faults_local" => {
                    procvmstat.numa_hint_faults_local = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "numa_pages_migrated" => {
                    procvmstat.numa_pages_migrated = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgmigrate_success" => {
                    procvmstat.pgmigrate_success = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "pgmigrate_fail" => {
                    procvmstat.pgmigrate_fail = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_migration_success" => {
                    procvmstat.thp_migration_success = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "thp_migration_fail" => {
                    procvmstat.thp_migration_fail = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "thp_migration_split" => {
                    procvmstat.thp_migration_split = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "compact_migrate_scanned" => {
                    procvmstat.compact_migrate_scanned = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "compact_free_scanned" => {
                    procvmstat.compact_free_scanned = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "compact_isolated" => {
                    procvmstat.compact_isolated = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "compact_stall" => {
                    procvmstat.compact_stall = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "compact_fail" => {
                    procvmstat.compact_fail = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "compact_success" => {
                    procvmstat.compact_success = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "compact_daemon_wake" => {
                    procvmstat.compact_daemon_wake = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "compact_daemon_migrate_scanned" => {
                    procvmstat.compact_daemon_migrate_scanned =
                        ProcVmStat::parse_proc_vmstat_line(line)
                }
                "compact_daemon_free_scanned" => {
                    procvmstat.compact_daemon_free_scanned =
                        ProcVmStat::parse_proc_vmstat_line(line)
                }
                "htlb_buddy_alloc_success" => {
                    procvmstat.htlb_buddy_alloc_success = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "htlb_buddy_alloc_fail" => {
                    procvmstat.htlb_buddy_alloc_fail = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "cma_alloc_success" => {
                    procvmstat.cma_alloc_success = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "cma_alloc_fail" => {
                    procvmstat.cma_alloc_fail = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "unevictable_pgs_culled" => {
                    procvmstat.unevictable_pgs_culled = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "unevictable_pgs_scanned" => {
                    procvmstat.unevictable_pgs_scanned = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "unevictable_pgs_rescued" => {
                    procvmstat.unevictable_pgs_rescued = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "unevictable_pgs_mlocked" => {
                    procvmstat.unevictable_pgs_mlocked = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "unevictable_pgs_munlocked" => {
                    procvmstat.unevictable_pgs_munlocked = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "unevictable_pgs_cleared" => {
                    procvmstat.unevictable_pgs_cleared = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "unevictable_pgs_stranded" => {
                    procvmstat.unevictable_pgs_stranded = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_fault_alloc" => {
                    procvmstat.thp_fault_alloc = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_fault_fallback" => {
                    procvmstat.thp_fault_fallback = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_fault_fallback_charge" => {
                    procvmstat.thp_fault_fallback_charge = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "thp_collapse_alloc" => {
                    procvmstat.thp_collapse_alloc = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_collapse_alloc_failed" => {
                    procvmstat.thp_collapse_alloc_failed = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_file_alloc" => {
                    procvmstat.thp_file_alloc = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_file_fallback" => {
                    procvmstat.thp_file_fallback = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "thp_file_fallback_charge" => {
                    procvmstat.thp_file_fallback_charge = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "thp_file_mapped" => {
                    procvmstat.thp_file_mapped = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_split_page" => {
                    procvmstat.thp_split_page = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_split_page_failed" => {
                    procvmstat.thp_split_page_failed = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_deferred_split_page" => {
                    procvmstat.thp_deferred_split_page = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_split_pmd" => {
                    procvmstat.thp_split_pmd = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_scan_exceed_none_pte" => {
                    procvmstat.thp_scan_exceed_none_pte = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "thp_scan_exceed_swap_pte" => {
                    procvmstat.thp_scan_exceed_swap_pte = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "thp_scan_exceed_share_pte" => {
                    procvmstat.thp_scan_exceed_share_pte = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "thp_zero_page_alloc" => {
                    procvmstat.thp_zero_page_alloc = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_zero_page_alloc_failed" => {
                    procvmstat.thp_zero_page_alloc_failed = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "thp_swpout" => procvmstat.thp_swpout = ProcVmStat::parse_proc_vmstat_line(line),
                "thp_swpout_fallback" => {
                    procvmstat.thp_swpout_fallback = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "balloon_inflate" => {
                    procvmstat.balloon_inflate = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "balloon_deflate" => {
                    procvmstat.balloon_deflate = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "balloon_migrate" => {
                    procvmstat.balloon_migrate = ProcVmStat::parse_proc_vmstat_line(line)
                }
                "swap_ra" => procvmstat.swap_ra = ProcVmStat::parse_proc_vmstat_line(line),
                "swap_ra_hit" => procvmstat.swap_ra_hit = ProcVmStat::parse_proc_vmstat_line(line),
                "ksm_swpin_copy" => {
                    procvmstat.ksm_swpin_copy = ProcVmStat::parse_proc_vmstat_line_option(line)
                }
                "cow_ksm" => procvmstat.cow_ksm = ProcVmStat::parse_proc_vmstat_line_option(line),
                "zswpin" => procvmstat.zswpin = ProcVmStat::parse_proc_vmstat_line_option(line),
                "zswpout" => procvmstat.zswpout = ProcVmStat::parse_proc_vmstat_line_option(line),
                "nr_unstable" => procvmstat.nr_unstable = ProcVmStat::parse_proc_vmstat_line(line),
                _ => warn!("vmstat: unknown entry found: {}", line),
            }
        }
        procvmstat
    }
    fn parse_proc_vmstat_line(proc_vmstat_line: &str) -> u64 {
        proc_vmstat_line
            .split_whitespace()
            .skip(1)
            .map(|number| number.parse::<u64>().unwrap())
            .nth(0)
            .unwrap_or(0)
    }
    fn parse_proc_vmstat_line_option(proc_vmstat_line: &str) -> Option<u64> {
        Some(proc_vmstat_line
            .split_whitespace()
            .skip(1)
            .map(|number| number.parse::<u64>().unwrap())
            .nth(0)
            .unwrap_or(0))
    }
    pub fn read_proc_vmstat(proc_vmstat_file: &str) -> ProcVmStat {
        let proc_vmstat_output = read_to_string(proc_vmstat_file)
            .unwrap_or_else(|error| panic!("Error {} reading file: {}", error, proc_vmstat_file));
        ProcVmStat::parse_proc_vmstat_output(&proc_vmstat_output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::fs::{create_dir_all, remove_dir_all, write};

    #[test]
    fn parse_vmstat_line() {
        let vmstat_line = "nr_free_pages 778308";
        let result = ProcVmStat::parse_proc_vmstat_line(&vmstat_line);
        assert_eq!(result, 778308_u64);
    }

    #[test]
    fn parse_full_proc_vmstat_file_contents() {
        let proc_vmstat = "nr_free_pages 778263
nr_zone_inactive_anon 212
nr_zone_active_anon 21214
nr_zone_inactive_file 86210
nr_zone_active_file 85676
nr_zone_unevictable 0
nr_zone_write_pending 1
nr_mlock 0
nr_bounce 0
nr_zspages 0
nr_free_cma 7808
numa_hit 40773813
numa_miss 0
numa_foreign 0
numa_interleave 1212
numa_local 40773813
numa_other 0
nr_inactive_anon 212
nr_active_anon 21214
nr_inactive_file 86210
nr_active_file 85676
nr_unevictable 0
nr_slab_reclaimable 8551
nr_slab_unreclaimable 8749
nr_isolated_anon 0
nr_isolated_file 0
workingset_nodes 0
workingset_refault_anon 0
workingset_refault_file 0
workingset_activate_anon 0
workingset_activate_file 0
workingset_restore_anon 0
workingset_restore_file 0
workingset_nodereclaim 0
nr_anon_pages 21233
nr_mapped 33359
nr_file_pages 172080
nr_dirty 1
nr_writeback 0
nr_writeback_temp 0
nr_shmem 194
nr_shmem_hugepages 0
nr_shmem_pmdmapped 0
nr_file_hugepages 0
nr_file_pmdmapped 0
nr_anon_transparent_hugepages 0
nr_vmscan_write 0
nr_vmscan_immediate_reclaim 0
nr_dirtied 66050
nr_written 62014
nr_throttled_written 0
nr_kernel_misc_reclaimable 0
nr_foll_pin_acquired 0
nr_foll_pin_released 0
nr_kernel_stack 2768
nr_shadow_call_stack 712
nr_page_table_pages 580
nr_sec_page_table_pages 0
nr_swapcached 0
pgpromote_success 0
pgpromote_candidate 0
nr_dirty_threshold 186274
nr_dirty_background_threshold 93023
pgpgin 569048
pgpgout 264157
pswpin 0
pswpout 0
pgalloc_dma 0
pgalloc_dma32 0
pgalloc_normal 42962188
pgalloc_movable 0
pgalloc_device 0
allocstall_dma 0
allocstall_dma32 0
allocstall_normal 0
allocstall_movable 0
allocstall_device 0
pgskip_dma 0
pgskip_dma32 0
pgskip_normal 0
pgskip_movable 0
pgskip_device 0
pgfree 43741863
pgactivate 0
pgdeactivate 0
pglazyfree 0
pgfault 55051790
pgmajfault 2851
pglazyfreed 0
pgrefill 0
pgreuse 1854584
pgsteal_kswapd 0
pgsteal_direct 0
pgsteal_khugepaged 0
pgdemote_kswapd 0
pgdemote_direct 0
pgdemote_khugepaged 0
pgscan_kswapd 0
pgscan_direct 0
pgscan_khugepaged 0
pgscan_direct_throttle 0
pgscan_anon 0
pgscan_file 0
pgsteal_anon 0
pgsteal_file 0
zone_reclaim_failed 0
pginodesteal 0
slabs_scanned 0
kswapd_inodesteal 0
kswapd_low_wmark_hit_quickly 0
kswapd_high_wmark_hit_quickly 0
pageoutrun 0
pgrotated 6
drop_pagecache 0
drop_slab 0
oom_kill 0
numa_pte_updates 0
numa_huge_pte_updates 0
numa_hint_faults 0
numa_hint_faults_local 0
numa_pages_migrated 0
pgmigrate_success 0
pgmigrate_fail 0
thp_migration_success 0
thp_migration_fail 0
thp_migration_split 0
compact_migrate_scanned 0
compact_free_scanned 0
compact_isolated 896
compact_stall 0
compact_fail 0
compact_success 0
compact_daemon_wake 0
compact_daemon_migrate_scanned 0
compact_daemon_free_scanned 0
htlb_buddy_alloc_success 0
htlb_buddy_alloc_fail 0
cma_alloc_success 3
cma_alloc_fail 0
unevictable_pgs_culled 0
unevictable_pgs_scanned 0
unevictable_pgs_rescued 0
unevictable_pgs_mlocked 0
unevictable_pgs_munlocked 0
unevictable_pgs_cleared 0
unevictable_pgs_stranded 0
thp_fault_alloc 0
thp_fault_fallback 0
thp_fault_fallback_charge 0
thp_collapse_alloc 0
thp_collapse_alloc_failed 0
thp_file_alloc 0
thp_file_fallback 0
thp_file_fallback_charge 0
thp_file_mapped 0
thp_split_page 0
thp_split_page_failed 0
thp_deferred_split_page 0
thp_split_pmd 0
thp_scan_exceed_none_pte 0
thp_scan_exceed_swap_pte 0
thp_scan_exceed_share_pte 0
thp_zero_page_alloc 0
thp_zero_page_alloc_failed 0
thp_swpout 0
thp_swpout_fallback 0
balloon_inflate 0
balloon_deflate 0
balloon_migrate 0
swap_ra 0
swap_ra_hit 0
ksm_swpin_copy 0
cow_ksm 0
zswpin 0
zswpout 0
nr_unstable 0";
        let result = ProcVmStat::parse_proc_vmstat_output(proc_vmstat);
        assert_eq!(
            result,
            ProcVmStat { nr_free_pages: 778263, nr_zone_inactive_anon: 212, nr_zone_active_anon: 21214, nr_zone_inactive_file: 86210, nr_zone_active_file: 85676, nr_zone_unevictable: 0, nr_zone_write_pending: 1, nr_mlock: 0, nr_bounce: 0, nr_zspages: 0 , nr_free_cma: 7808, numa_hit: 40773813, numa_miss: 0, numa_foreign: 0, numa_interleave: 1212, numa_local: 40773813, numa_other: 0, nr_inactive_anon: 212, nr_active_anon: 21214, nr_inactive_file: 86210, nr_active_file: 85676, nr_unevictable: 0, nr_slab_reclaimable: 8551, nr_slab_unreclaimable: 8749, nr_isolated_anon: 0, nr_isolated_file: 0, workingset_nodes: Some(0), workingset_refault_anon: Some(0), workingset_refault_file: Some(0), workingset_activate_anon: Some(0), workingset_activate_file: Some(0), workingset_restore_anon: Some(0), workingset_restore_file: Some(0), workingset_nodereclaim: 0, nr_anon_pages: 21233, nr_mapped: 33359, nr_file_pages: 0, nr_dirty: 1, nr_writeback: 0, nr_writeback_temp: 0, nr_shmem: 194, nr_shmem_hugepages: 0, nr_shmem_pmdmapped: 0, nr_file_hugepages: Some(0), nr_file_pmdmapped: Some(0), nr_anon_transparent_hugepages: 0, nr_vmscan_write: 0, nr_vmscan_immediate_reclaim: 0, nr_dirtied: 66050, nr_written: 62014, nr_throttled_written: Some(0), nr_kernel_misc_reclaimable: Some(0), nr_foll_pin_acquired: Some(0), nr_foll_pin_released: Some(0), nr_kernel_stack: 2768, nr_shadow_call_stack: Some(712), nr_page_table_pages: 580, nr_sec_page_table_pages: Some(0), nr_swapcached: Some(0), pgpromote_success: Some(0), pgpromote_candidate: Some(0), nr_dirty_threshold: 186274, nr_dirty_background_threshold: 93023, pgpgin: 569048, pgpgout: 264157, pswpin: 0, pswpout: 0, pgalloc_dma: 0, pgalloc_dma32: 0, pgalloc_normal: 42962188, pgalloc_movable: 0, pgalloc_device: Some(0), allocstall_dma: 0, allocstall_dma32: 0, allocstall_normal: 0, allocstall_movable: 0, allocstall_device: Some(0), pgskip_dma: 0, pgskip_dma32: 0, pgskip_normal: 0, pgskip_movable: 0, pgskip_device: Some(0), pgfree: 43741863, pgactivate: 0, pgdeactivate: 0, pglazyfree: 0, pglazyfreed: 0, pgfault: 55051790, pgmajfault: 2851, pgrefill: 0, pgreuse: Some(1854584), pgsteal_kswapd: 0, pgsteal_direct: 0, pgsteal_khugepaged: Some(0), pgdemote_kswapd: Some(0), pgdemote_direct: Some (0), pgdemote_khugepaged: Some(0), pgscan_kswapd: 0, pgscan_direct: 0, pgscan_khugepaged: Some(0), pgscan_direct_throttle: 0, pgscan_anon: Some(0), pgscan_file: Some(0), pgsteal_anon: Some(0), pgsteal_file: Some(0), zone_reclaim_failed: 0, pginodesteal: 0, slabs_scanned: Some(0), kswapd_inodesteal: 0, kswapd_low_wmark_hit_quickly: 0, kswapd_high_wmark_hit_quickly: 0, pageoutrun: 0, pgrotated: 6, drop_pagecache: 0, drop_slab: 0, oom_kill: 0, numa_pte_updates: 0, numa_huge_pte_updates: 0, numa_hint_faults: 0, numa_hint_faults_local: 0, numa_pages_migrated: 0, pgmigrate_success: 0, pgmigrate_fail: 0, thp_migration_success: Some(0), thp_migration_fail: Some(0), thp_migration_split: Some(0), compact_migrate_scanned: 0, compact_free_scanned : 0, compact_isolated: 896, compact_stall: 0, compact_fail: 0, compact_success: 0, compact_daemon_wake: 0, compact_daemon_migrate_scanned: 0, compact_daemon_free_scanned: 0, htlb_buddy_alloc_success: 0, htlb_buddy_alloc_fail: 0, cma_alloc_success: Some(3), cma_alloc_fail: Some(0), unevictable_pgs_culled: 0, unevictable_pgs_scanned: 0, unevictable_pgs_rescued: 0, unevictable_pgs_mlocked: 0, unevictable_pgs_munlocked: 0, unevictable_pgs_cleared: 0, unevictable_pgs_stranded: 0, thp_fault_alloc: 0, thp_fault_fallback: 0, thp_fault_fallback_charge: Some(0), thp_collapse_alloc: 0, thp_collapse_alloc_failed: 0, thp_file_alloc: 0, thp_file_fallback: Some(0), thp_file_fallback_charge: Some(0), thp_file_mapped: 0, thp_split_page: 0, thp_split_page_failed: 0, thp_deferred_split_page: 0, thp_split_pmd: 0, thp_scan_exceed_none_pte: Some(0), thp_scan_exceed_swap_pte: Some(0), thp_scan_exceed_share_pte: Some(0), thp_zero_page_alloc: 0, thp_zero_page_alloc_failed: 0, thp_swpout: 0, thp_swpout_fallback: 0, balloon_inflate: 0, balloon_deflate: 0, balloon_migrate: 0, swap_ra: 0, swap_ra_hit: 0, ksm_swpin_copy: Some(0), cow_ksm: Some(0), zswpin: Some(0), zswpout: Some(0), nr_unstable: 0 }
        );
    }

    #[test]
    fn create_proc_vmstat_file_and_read() {
        let proc_vmstat = "nr_free_pages 778263
nr_zone_inactive_anon 212
nr_zone_active_anon 21214
nr_zone_inactive_file 86210
nr_zone_active_file 85676
nr_zone_unevictable 0
nr_zone_write_pending 1
nr_mlock 0
nr_bounce 0
nr_zspages 0
nr_free_cma 7808
numa_hit 40773813
numa_miss 0
numa_foreign 0
numa_interleave 1212
numa_local 40773813
numa_other 0
nr_inactive_anon 212
nr_active_anon 21214
nr_inactive_file 86210
nr_active_file 85676
nr_unevictable 0
nr_slab_reclaimable 8551
nr_slab_unreclaimable 8749
nr_isolated_anon 0
nr_isolated_file 0
workingset_nodes 0
workingset_refault_anon 0
workingset_refault_file 0
workingset_activate_anon 0
workingset_activate_file 0
workingset_restore_anon 0
workingset_restore_file 0
workingset_nodereclaim 0
nr_anon_pages 21233
nr_mapped 33359
nr_file_pages 172080
nr_dirty 1
nr_writeback 0
nr_writeback_temp 0
nr_shmem 194
nr_shmem_hugepages 0
nr_shmem_pmdmapped 0
nr_file_hugepages 0
nr_file_pmdmapped 0
nr_anon_transparent_hugepages 0
nr_vmscan_write 0
nr_vmscan_immediate_reclaim 0
nr_dirtied 66050
nr_written 62014
nr_throttled_written 0
nr_kernel_misc_reclaimable 0
nr_foll_pin_acquired 0
nr_foll_pin_released 0
nr_kernel_stack 2768
nr_shadow_call_stack 712
nr_page_table_pages 580
nr_sec_page_table_pages 0
nr_swapcached 0
pgpromote_success 0
pgpromote_candidate 0
nr_dirty_threshold 186274
nr_dirty_background_threshold 93023
pgpgin 569048
pgpgout 264157
pswpin 0
pswpout 0
pgalloc_dma 0
pgalloc_dma32 0
pgalloc_normal 42962188
pgalloc_movable 0
pgalloc_device 0
allocstall_dma 0
allocstall_dma32 0
allocstall_normal 0
allocstall_movable 0
allocstall_device 0
pgskip_dma 0
pgskip_dma32 0
pgskip_normal 0
pgskip_movable 0
pgskip_device 0
pgfree 43741863
pgactivate 0
pgdeactivate 0
pglazyfree 0
pgfault 55051790
pgmajfault 2851
pglazyfreed 0
pgrefill 0
pgreuse 1854584
pgsteal_kswapd 0
pgsteal_direct 0
pgsteal_khugepaged 0
pgdemote_kswapd 0
pgdemote_direct 0
pgdemote_khugepaged 0
pgscan_kswapd 0
pgscan_direct 0
pgscan_khugepaged 0
pgscan_direct_throttle 0
pgscan_anon 0
pgscan_file 0
pgsteal_anon 0
pgsteal_file 0
zone_reclaim_failed 0
pginodesteal 0
slabs_scanned 0
kswapd_inodesteal 0
kswapd_low_wmark_hit_quickly 0
kswapd_high_wmark_hit_quickly 0
pageoutrun 0
pgrotated 6
drop_pagecache 0
drop_slab 0
oom_kill 0
numa_pte_updates 0
numa_huge_pte_updates 0
numa_hint_faults 0
numa_hint_faults_local 0
numa_pages_migrated 0
pgmigrate_success 0
pgmigrate_fail 0
thp_migration_success 0
thp_migration_fail 0
thp_migration_split 0
compact_migrate_scanned 0
compact_free_scanned 0
compact_isolated 896
compact_stall 0
compact_fail 0
compact_success 0
compact_daemon_wake 0
compact_daemon_migrate_scanned 0
compact_daemon_free_scanned 0
htlb_buddy_alloc_success 0
htlb_buddy_alloc_fail 0
cma_alloc_success 3
cma_alloc_fail 0
unevictable_pgs_culled 0
unevictable_pgs_scanned 0
unevictable_pgs_rescued 0
unevictable_pgs_mlocked 0
unevictable_pgs_munlocked 0
unevictable_pgs_cleared 0
unevictable_pgs_stranded 0
thp_fault_alloc 0
thp_fault_fallback 0
thp_fault_fallback_charge 0
thp_collapse_alloc 0
thp_collapse_alloc_failed 0
thp_file_alloc 0
thp_file_fallback 0
thp_file_fallback_charge 0
thp_file_mapped 0
thp_split_page 0
thp_split_page_failed 0
thp_deferred_split_page 0
thp_split_pmd 0
thp_scan_exceed_none_pte 0
thp_scan_exceed_swap_pte 0
thp_scan_exceed_share_pte 0
thp_zero_page_alloc 0
thp_zero_page_alloc_failed 0
thp_swpout 0
thp_swpout_fallback 0
balloon_inflate 0
balloon_deflate 0
balloon_migrate 0
swap_ra 0
swap_ra_hit 0
ksm_swpin_copy 0
cow_ksm 0
zswpin 0
zswpout 0
nr_unstable 0";
        let directory_suffix: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        let test_path = format!("/tmp/test.{}", directory_suffix);
        create_dir_all(format!("{}", test_path)).expect("Error creating mock directory.");

        write(format!("{}/vmstat", test_path), proc_vmstat)
            .expect(format!("Error writing to {}/vmstat", test_path).as_str());
        let result = Builder::new()
            .file_name(format!("{}/vmstat", test_path).as_str())
            .read();

        remove_dir_all(test_path).unwrap();

        assert_eq!(
            result,
            ProcVmStat { nr_free_pages: 778263, nr_zone_inactive_anon: 212, nr_zone_active_anon: 21214, nr_zone_inactive_file: 86210, nr_zone_active_file: 85676, nr_zone_unevictable: 0, nr_zone_write_pending: 1, nr_mlock: 0, nr_bounce: 0, nr_zspages: 0 , nr_free_cma: 7808, numa_hit: 40773813, numa_miss: 0, numa_foreign: 0, numa_interleave: 1212, numa_local: 40773813, numa_other: 0, nr_inactive_anon: 212, nr_active_anon: 21214, nr_inactive_file: 86210, nr_active_file: 85676, nr_unevictable: 0, nr_slab_reclaimable: 8551, nr_slab_unreclaimable: 8749, nr_isolated_anon: 0, nr_isolated_file: 0, workingset_nodes: Some(0), workingset_refault_anon: Some(0), workingset_refault_file: Some(0), workingset_activate_anon: Some(0), workingset_activate_file: Some(0), workingset_restore_anon: Some(0), workingset_restore_file: Some(0), workingset_nodereclaim: 0, nr_anon_pages: 21233, nr_mapped: 33359, nr_file_pages: 0, nr_dirty: 1, nr_writeback: 0, nr_writeback_temp: 0, nr_shmem: 194, nr_shmem_hugepages: 0, nr_shmem_pmdmapped: 0, nr_file_hugepages: Some(0), nr_file_pmdmapped: Some(0), nr_anon_transparent_hugepages: 0, nr_vmscan_write: 0, nr_vmscan_immediate_reclaim: 0, nr_dirtied: 66050, nr_written: 62014, nr_throttled_written: Some(0), nr_kernel_misc_reclaimable: Some(0), nr_foll_pin_acquired: Some(0), nr_foll_pin_released: Some(0), nr_kernel_stack: 2768, nr_shadow_call_stack: Some(712), nr_page_table_pages: 580, nr_sec_page_table_pages: Some(0), nr_swapcached: Some(0), pgpromote_success: Some(0), pgpromote_candidate: Some(0), nr_dirty_threshold: 186274, nr_dirty_background_threshold: 93023, pgpgin: 569048, pgpgout: 264157, pswpin: 0, pswpout: 0, pgalloc_dma: 0, pgalloc_dma32: 0, pgalloc_normal: 42962188, pgalloc_movable: 0, pgalloc_device: Some(0), allocstall_dma: 0, allocstall_dma32: 0, allocstall_normal: 0, allocstall_movable: 0, allocstall_device: Some(0), pgskip_dma: 0, pgskip_dma32: 0, pgskip_normal: 0, pgskip_movable: 0, pgskip_device: Some(0), pgfree: 43741863, pgactivate: 0, pgdeactivate: 0, pglazyfree: 0, pglazyfreed: 0, pgfault: 55051790, pgmajfault: 2851, pgrefill: 0, pgreuse: Some(1854584), pgsteal_kswapd: 0, pgsteal_direct: 0, pgsteal_khugepaged: Some(0), pgdemote_kswapd: Some(0), pgdemote_direct: Some (0), pgdemote_khugepaged: Some(0), pgscan_kswapd: 0, pgscan_direct: 0, pgscan_khugepaged: Some(0), pgscan_direct_throttle: 0, pgscan_anon: Some(0), pgscan_file: Some(0), pgsteal_anon: Some(0), pgsteal_file: Some(0), zone_reclaim_failed: 0, pginodesteal: 0, slabs_scanned: Some(0), kswapd_inodesteal: 0, kswapd_low_wmark_hit_quickly: 0, kswapd_high_wmark_hit_quickly: 0, pageoutrun: 0, pgrotated: 6, drop_pagecache: 0, drop_slab: 0, oom_kill: 0, numa_pte_updates: 0, numa_huge_pte_updates: 0, numa_hint_faults: 0, numa_hint_faults_local: 0, numa_pages_migrated: 0, pgmigrate_success: 0, pgmigrate_fail: 0, thp_migration_success: Some(0), thp_migration_fail: Some(0), thp_migration_split: Some(0), compact_migrate_scanned: 0, compact_free_scanned : 0, compact_isolated: 896, compact_stall: 0, compact_fail: 0, compact_success: 0, compact_daemon_wake: 0, compact_daemon_migrate_scanned: 0, compact_daemon_free_scanned: 0, htlb_buddy_alloc_success: 0, htlb_buddy_alloc_fail: 0, cma_alloc_success: Some(3), cma_alloc_fail: Some(0), unevictable_pgs_culled: 0, unevictable_pgs_scanned: 0, unevictable_pgs_rescued: 0, unevictable_pgs_mlocked: 0, unevictable_pgs_munlocked: 0, unevictable_pgs_cleared: 0, unevictable_pgs_stranded: 0, thp_fault_alloc: 0, thp_fault_fallback: 0, thp_fault_fallback_charge: Some(0), thp_collapse_alloc: 0, thp_collapse_alloc_failed: 0, thp_file_alloc: 0, thp_file_fallback: Some(0), thp_file_fallback_charge: Some(0), thp_file_mapped: 0, thp_split_page: 0, thp_split_page_failed: 0, thp_deferred_split_page: 0, thp_split_pmd: 0, thp_scan_exceed_none_pte: Some(0), thp_scan_exceed_swap_pte: Some(0), thp_scan_exceed_share_pte: Some(0), thp_zero_page_alloc: 0, thp_zero_page_alloc_failed: 0, thp_swpout: 0, thp_swpout_fallback: 0, balloon_inflate: 0, balloon_deflate: 0, balloon_migrate: 0, swap_ra: 0, swap_ra_hit: 0, ksm_swpin_copy: Some(0), cow_ksm: Some(0), zswpin: Some(0), zswpout: Some(0), nr_unstable: 0 }
        );
    }
}
