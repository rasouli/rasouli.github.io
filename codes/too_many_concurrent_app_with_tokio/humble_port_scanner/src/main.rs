use std::error::Error;

use clap::Parser;

// *.
use ipnet::Ipv4Net;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct PortScannerArgs {
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    subnets: Vec<String>,
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    ports: Vec<String>,
}
// *
struct SubnetScanConfiguration {
    subnet: Ipv4Net,
    port_begin: u8,
    port_end: u8,
}

fn main() {
    let PortScannerArgs { subnets, ports } = PortScannerArgs::parse();

    if subnets.len() != ports.len() {
        println!(
            "Number of subnets and ports must be equal. subnets count was {}, ports count was: {}",
            subnets.len(),
            ports.len()
        );

        return;
    }

    println!("all  good.");
}

fn parse_ports_ranges(port_range: String) -> Option<(u8, u8)> {
    port_range
        .split_once(':')
        .and_then(|(begin_port_str, end_port_str)| {
            let begin_port = u8::from_str_radix(begin_port_str, 10)?;
            let end_port = u8::from_str_radix(end_port_str, 10).;
            (begin_port, end_port)
        })
}

fn parse_subnets(subnet: String) -> Option<Ipv4Net> {}

fn prepare_subnet_and_ports(
    subnets: Vec<String>,
    ports_ranges: Vec<String>,
) -> Result<Vec<SubnetScanConfiguration>, Box<dyn Error>> {
}
