use proc_sys_parser::stat;
use proc_sys_parser::schedstat;

fn main()
{
    let stat = stat::read();
    println!("{:#?}", stat);

    let schedstat = schedstat::read();
    println!("{:#?}", schedstat);
}
