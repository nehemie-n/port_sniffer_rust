/// Codebase sourced from https://www.youtube.com/watch?v=-Jp7sabBCp4&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb
///
///
// ip_sniffer.exe -h (Show help)
// ip_sniffer.exe -j 100 192.168.1.1 (100 threads)
// ip_sniffer.exe 192.168.1.1 (use default n threads)
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::{env, fmt::Debug, process, str::FromStr};

const MAX_PORT: u16 = 65535;

/// Holds the key input arguments from the terminal
struct Args {
    flag: String,
    ip: IpAddr,
    threads: u16,
}

/// implement how above structure is printed when stringifying
impl Debug for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(format!("{} {} {}", &self.flag, &self.ip, &self.threads).as_str());
    }
}

impl Args {
    /// Returns the Args or error when failed to parse the provided arguments
    /// ### Arguments
    /// * `args` - A vector of type strings
    /// ### Example
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// let args = Args::new(env::args().collect());
    /// ```
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
                threads: 4000,
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

/// Scans from a certain port (default being 0) to max port while signaling the open ones found
fn scan(tx: Sender<u16>, start_port: u16, ip: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((ip, port)) {
            Ok(_) => {
                print!(". {}", port);
                io::stdout().flush().unwrap();
                tx.send(port).unwrap()
            }
            Err(err) => {
                print!("Error {:?}", err)
            }
        }
        // exit point
        if (MAX_PORT - port) <= num_threads || port >= MAX_PORT {
            print!("Should exit");
            break;
        }
        println!("port {}", port);
        port += num_threads;
    }
}

/// implements the process of creating threads and listening to send output and adding it to a vector containing open ports
/// later onprints all open ports
fn snif_around(args: Args) {
    let num_threads = args.threads;
    let (tx, rx) = channel::<u16>();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || scan(tx, i, args.ip.clone(), num_threads));
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p)
    }

    out.sort();
    for v in out {
        println!("{} is open", v);
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
