use std::net::ToSocketAddrs;
use clap::{Args, Parser, Subcommand};
use hostfile::parse_hostfile;

#[derive(Parser, Debug)]
#[command(name = "rust-hc", version, about = "Rust Hostname Controller with daemon services.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Test,
    #[command(subcommand)]
    Hostnames(NoArgumentCommands),
}

#[derive(Subcommand, Debug)]
pub enum NoArgumentCommands {
    List,
    Check(GenerateKeyArgs),
}

#[derive(Args, Debug)]
pub struct GenerateKeyArgs {
    pub hostname: String,
}


fn main() {
    let cli_args = Cli::parse();
    match cli_args.command {
        Commands::Test => {
            println!("Testing hostnames...");
            let files = parse_hostfile();
            for file in files {
                for sub_files in file {
                    if sub_files.ip.is_ipv6() {
                        for name in sub_files.names {
                            let hostname = format!("{}:80", name);
                            match hostname.to_socket_addrs() {
                                Ok(addrs) => {
                                    for addr in addrs {
                                        if addr.is_ipv6() {
                                            println!("{}'s ipv6 addresses: {}", hostname,addr.ip());
                                            if addr.ip() == sub_files.ip {
                                                println!("Matched address.");
                                            } else {
                                                println!("Unmatched address.");
                                            }
                                        } else {
                                            println!("{} is not ipv6 addresses", hostname);
                                        }
                                        println!("\n");
                                    }
                                }
                                Err(e) => eprintln!("Bir hata oluştu ({}): {}", e, hostname),
                            }
                        }

                    } else if sub_files.ip.is_ipv4() {
                        for name in sub_files.names {
                            let hostname = format!("{}:80", name);
                            match hostname.to_socket_addrs() {
                                Ok(addrs) => {
                                    for addr in addrs {
                                        if addr.is_ipv4() {
                                            println!("{}'s ipv4 addresses: {}", hostname,addr.ip());
                                            if addr.ip() == sub_files.ip {
                                                println!("Matched address.");
                                            } else {
                                                println!("Unmatched address.");
                                            }
                                        } else {
                                            println!("{} is not ipv4 addresses", hostname);
                                        }
                                    }
                                    println!("\n");
                                }
                                Err(e) => eprintln!("Bir hata oluştu ({}): {}", e, hostname),
                            }
                        }
                    } else {
                        print!("Unrecognized ip address: {}", sub_files.ip);
                    }
                }
            }

        }
        Commands::Hostnames(cmd) => match cmd {
            NoArgumentCommands::Check(GenerateKeyArgs { hostname }) => {
                let edited = hostname + ":80";
                match edited.to_socket_addrs() {
                    Ok(addrs) => {
                        for addr in addrs {
                            if addr.is_ipv4() {
                                println!("{}'s ipv4 addresses: {}", edited,addr.ip());
                            }
                        }
                    }
                    Err(e) => eprintln!("Bir hata oluştu ({}): {}", e, edited),
                }
            },
            NoArgumentCommands::List => {
                println!("Getting hostnames...");
                let files = parse_hostfile();
                for file in files {
                    for sub_files in file {
                        if sub_files.ip.is_ipv6() {
                            println!("# IPV6: {}, Name: {:?}", sub_files.ip, sub_files.names);
                        } else if sub_files.ip.is_ipv4() {
                            println!("# IPV4: {}, Name: {:?}", sub_files.ip, sub_files.names);
                        } else {
                            print!("Unrecognized ip address: {}", sub_files.ip);
                        }
                    }
                }
            }
        }
    }
}
