use std::{error::Error, iter};

use clap::Parser;

// *.
use ipnet::Ipv4Net;

use anyhow::{anyhow, Context, Result};

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
    begin_port: u16,
    end_port: u16,
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

fn parse_subnet(subnet: String) -> Result<Ipv4Net> {
    subnet
        .parse::<Ipv4Net>()
        .context(format!("Unable to parse subnet: {}", subnet))
}

fn parse_port_ranges(port_range: String) -> Result<(u16, u16)> {
    let (begin_port_str, end_port_str) = port_range
        .split_once(':')
        .context("Port ranges should be seperated in following format: [begin_port]:[end_port]")?;

    if begin_port_str.is_empty() || end_port_str.is_empty() {
        return Err(anyhow!(
            "Empty port values given in the port range {}",
            port_range
        ));
    }

    let begin_port = begin_port_str
        .parse::<u16>()
        .context(format!(
            "Unable to parse the begining of port range: {}",
            begin_port_str
        ))?;

    let end_port = end_port_str
        .parse::<u16>()
        .context(format!(
            "Unable to parse the end of port range: {}",
            end_port_str
        ))?;

    if begin_port > end_port {
        return Err(anyhow!(
            "Begin port {} is bigger than the end port {}",
            begin_port,
            end_port
        ));
    }

    Ok((begin_port, end_port))
}

fn prepare_subnets_and_ports(
    subnets: Vec<String>,
    port_ranges: Vec<String>,
) -> Vec<Result<SubnetScanConfiguration>> {
    subnets
        .into_iter()
        .zip(port_ranges)
        .map(|(subnet_str, port_range_str)| {
            parse_subnet(subnet_str).and_then(|subnet| {
                parse_port_ranges(port_range_str).map(|(begin_port, end_port)| {
                    SubnetScanConfiguration {
                        subnet,
                        begin_port,
                        end_port,
                    }
                })
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn parse_port_ranges_test() {
        assert_eq!(
            (5000_u16, 8000_u16),
            parse_port_ranges(String::from("5000:8000")).unwrap()
        );

        assert_eq!(
            "Port ranges should be seperated in following format: [begin_port]:[end_port]",
            parse_port_ranges(String::from("garBage"))
                .err()
                .unwrap()
                .to_string()
        );

        assert_eq!(
            format!("Unable to parse the begining of port range: {}", "garBage"),
            parse_port_ranges(String::from("garBage:5000"))
                .err()
                .unwrap()
                .to_string()
        );

        assert_eq!(
            format!("Unable to parse the end of port range: {}", "garBage"),
            parse_port_ranges(String::from("5000:garBage"))
                .err()
                .unwrap()
                .to_string()
        );

        assert_eq!(
            format!("Begin port {} is bigger than the end port {}", 8000, 5000),
            parse_port_ranges(String::from("8000:5000"))
                .err()
                .unwrap()
                .to_string()
        );
    }

    #[test]
    fn parse_subnet_test() {
        assert_eq!(
            format!("Unable to parse subnet: {}", "garBage"),
            parse_subnet(String::from("garBage"))
                .err()
                .unwrap()
                .to_string()
        );

        assert_eq!(
            Ipv4Net::new(Ipv4Addr::new(172, 16, 0, 0), 16).unwrap(),
            parse_subnet(String::from("172.16.0.0/16")).unwrap()
        )
    }
}
