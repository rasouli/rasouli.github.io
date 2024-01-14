use std::time::Duration;
use std::{error::Error, future::Future, iter, net::Ipv4Addr, sync::Arc};

use clap::Parser;

// *.
use ipnet::Ipv4Net;

use anyhow::{anyhow, Context, Result};
use tokio::net::TcpStream;
use tokio::runtime::{self, Runtime};

use tokio::{task, time};

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

// *
struct IpPortScanResult {
    ip: Ipv4Addr,
    port: u16,
    state: PortState,
}

enum PortState {
    Open,
    Closed,
    TimeOut,
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

    let runtime = setup_tokio_runtime();

    println!("all  good.");
}

async fn scan_ipv4_subnet(scan_configuration: SubnetScanConfiguration) {
    let SubnetScanConfiguration {
        subnet,
        begin_port,
        end_port,
    } = scan_configuration;

    for ip in subnet.hosts() {
        for port in begin_port..end_port {}
    }
}

async fn check_port_status_with_timeout(
    ip: Ipv4Addr,
    port: u16,
    timeout: Duration,
) -> IpPortScanResult {
    let stream = time::timeout(timeout, async { TcpStream::connect((ip, port)).await }).await;

    let state = match stream {
        Err(_) => PortState::TimeOut,
        Ok(tcp_result) if tcp_result.is_ok() => PortState::Open,
        _ => PortState::Closed,
    };

    IpPortScanResult { ip, port, state }
}

// requires setting up the .cargo/config.toml
async fn run_named_task<F>(task_name: String, runtime: &Arc<Runtime>, fut: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    task::Builder::new()
        .name(&task_name)
        .spawn_on(fut, runtime.clone().handle())
        .unwrap()
        .await;
}

// *
fn setup_tokio_runtime() -> Runtime {
    runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("scan_runtime")
        .enable_io()
        .enable_time()
        .enable_metrics_poll_count_histogram()
        .build()
        .expect("Failed to build Tokio Runtime.")
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

fn prepare_subnets_and_port_ranges(
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

    #[tokio::test]
    async fn check_port_status_with_time_out_tests() {}
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

    async fn setup_test_server(host: &str, port: u16) {
        // cont. test with a mock tcp server
    }

    #[test]
    fn prepare_subnets_and_port_ranges_test() {
        assert_eq!(vec![Ok(SubnetScanConfiguration {})])
    }
}
