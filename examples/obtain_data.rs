use proc_sys_parser::stat;
use proc_sys_parser::schedstat;
use proc_sys_parser::diskstats;
use proc_sys_parser::net_dev;
use proc_sys_parser::meminfo;
use proc_sys_parser::block;
use proc_sys_parser::vmstat;
use proc_sys_parser::loadavg;
use proc_sys_parser::pressure;

fn main()
{
    let stat = stat::read();
    println!("{:?}", stat);

    let schedstat = schedstat::read();
    println!("{:?}", schedstat);

    let diskstats = diskstats::read();
    println!("{:?}", diskstats);

    let net_dev = net_dev::read();
    println!("{:?}", net_dev);

    let meminfo = meminfo::read();
    println!("{:?}", meminfo);

    let block = block::read();
    println!("{:?}", block);

    let vmstat = vmstat::read();
    println!("{:?}", vmstat);

    let loadavg = loadavg::read();
    println!("{:?}", loadavg);

    let pressure = pressure::read();
    println!("{:?}", pressure);
}
