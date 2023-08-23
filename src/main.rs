//
// ip_sniffer.exe -h (Show help)
// ip_sniffer.exe -j 100 192.168.1.1 (100 threads)
// ip_sniffer.exe 192.168.1.1 (use default n threads)

use std::{env, fmt::Debug, net::IpAddr, str::FromStr};

struct Args {
    flag: String,
    ip: IpAddr,
    threads: u16,
}

impl Debug for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(format!("{} {} {}", &self.flag, &self.ip, &self.threads).as_str());
    }
}
impl Args {
    fn new(args: Vec<String>) -> Result<Self, &'static str> {
        let args_len = args.len();
        if args_len < 2 {
            return Err("Not enough arguments");
        } else if args_len > 4 {
            return Err("Not enough arguments");
        }

        let f = args[1].clone();

        if let Ok(ip_address) = IpAddr::from_str(&f) {
            return Ok(Self {
                flag: String::from(""),
                ip: ip_address,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args_len == 2 {
                println!("Usage: -j to select how many threads you want");
                return Err("");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let ip_address = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a  valid IP Address."),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Invalid number of threads."),
                };
                return Ok(Self {
                    threads,
                    flag,
                    ip: ip_address,
                });
            } else {
                return Err("Invalid Syntax");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = Args::new(args);
    println!("Arguments:: {:?}", args);
}
