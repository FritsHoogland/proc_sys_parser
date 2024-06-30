use proc_sys_parser::schedstat;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pid: u32,
}

fn main() {
    let args = Args::parse();
    
    loop {
        let r = schedstat::read_pid( args.pid );
        println!("{:?}", r);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}


