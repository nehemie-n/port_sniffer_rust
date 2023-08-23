//
// ip_sniffer.exe -h (Show help)
// ip_sniffer.exe -j 100 192.168.1.1 (100 threads)
// ip_sniffer.exe 192.168.1.1 (use default n threads)

use core::num;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::{env, fmt::Debug, process, str::FromStr};

const MAX_PORT: u16 = 65535;

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

fn scan(tx: Sender<u16>, start_port: u16, ip: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((ip, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap()
            }
            Err(_) => {}
        }

        if (MAX_PORT - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn snif_around(args: Args) {
    let num_threads = args.threads;
    let (tx, rx) = channel::<u16>();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || scan(tx, i, args.ip.clone(), num_threads));
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let args = Args::new(args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0)
        } else {
            eprintln!("Probelm parsing arguments:: {}", err);
            process::exit(0)
        }
    });
    println!("Arguments:: {:?}", args);
    snif_around(args);
}
