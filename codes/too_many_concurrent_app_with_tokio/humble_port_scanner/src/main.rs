use std::iter::zip;
use std::pin::Pin;
use std::time::Duration;
use std::{error::Error, future::Future, iter, net::Ipv4Addr, sync::Arc};

use app::SubnetScannerApp;
use clap::Parser;

use ipnet::Ipv4Net;

use anyhow::{anyhow, bail, Context, Result};
use tokio::net::TcpStream;
use tokio::runtime::{self, Runtime};

use tokio::sync::mpsc::UnboundedReceiver;
use tokio::{task, time};

use crate::models::{IpPortScanResult, PortScannerArgs};
use crate::progress_helper::ScanProgressTracker;
use crate::scan_stream::ScanResultStreamer;

mod app;
mod arg_helpers;
mod errors;
mod models;
mod port_helpers;
mod progress_helper;
mod scan_stream;
mod subnet_helpers;
mod tokio_helpers;

const SCAN_TIMEOUT_SEC: u64 = 1;

fn main() -> anyhow::Result<()> {
    let PortScannerArgs { subnets, ports } = PortScannerArgs::parse();

    if subnets.len() != ports.len() {
        bail!(
            "Number of subnets and ports must be equal. subnets count was {}, ports count was: {}",
            subnets.len(),
            ports.len()
        )
    }

    let mut subnet_scan_configurations: Vec<models::SubnetScanConfiguration> = Vec::new();
    let mut ipv4_subnets: Vec<Ipv4Net> = Vec::new();

    for (subnet, port_ranges) in Iterator::zip(subnets.into_iter(), ports.into_iter()) {
        let (begin_port, end_port) = arg_helpers::parse_port_ranges(port_ranges)?;
        let subnet = subnet_helpers::parse_subnet(subnet)?;
        ipv4_subnets.push(subnet);
        subnet_scan_configurations.push(models::SubnetScanConfiguration {
            subnet,
            begin_port,
            end_port,
        });
    }

    let runtime = Arc::new(tokio_helpers::setup_tokio_runtime());
    let scan_timeout = Duration::from_secs(SCAN_TIMEOUT_SEC);

    let app = SubnetScannerApp::builder()
        .set_configs(subnet_scan_configurations)
        .set_scan_timeout(Duration::from_secs(SCAN_TIMEOUT_SEC))
        .set_runtime(&runtime)
        .build()?;

    app.run();

    Ok(())
}
