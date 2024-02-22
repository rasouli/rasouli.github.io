use std::sync::Arc;
use std::time::Duration;

use app::SubnetScannerApp;
use clap::Parser;

use anyhow::bail;

use crate::models::PortScannerArgs;

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

    let subnet_scan_configurations = arg_helpers::prepare_subnets_and_port_ranges(subnets, ports)?;
    let runtime = Arc::new(tokio_helpers::setup_tokio_runtime());
    let _scan_timeout = Duration::from_secs(SCAN_TIMEOUT_SEC);

    let mut app = SubnetScannerApp::builder()
        .set_configs(subnet_scan_configurations)
        .set_scan_timeout(Duration::from_secs(SCAN_TIMEOUT_SEC))
        .set_runtime(&runtime)
        .build()?;

    app.start_subnet_scans();
    app.run();

    Ok(())
}
