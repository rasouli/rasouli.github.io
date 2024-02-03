use std::net::Ipv4Addr;

use clap::Parser;
use ipnet::Ipv4Net;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PortScannerArgs {
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub subnets: Vec<String>,
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub ports: Vec<String>,
}

pub struct SubnetScanConfiguration {
    pub subnet: Ipv4Net,
    pub begin_port: u16,
    pub end_port: u16,
}

#[derive(Debug)]
pub struct IpPortScanResult {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub state: PortState,
}

#[derive(Debug, PartialEq)]
pub enum PortState {
    Open,
    Closed,
    TimeOut,
}
