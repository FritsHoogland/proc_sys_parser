use std::error::Error;
use proc_sys_parser::stat;

fn main() -> Result<(), Box<dyn Error>> {
    let stat = stat::Builder::new().path("/mypath").read();
    println!("{:?}", stat);

    Ok(())
}
