use std::{future::Future, sync::mpsc::Receiver, time::Duration};

use tokio::{
    runtime::Handle,
    sync::mpsc::{self, unbounded_channel, UnboundedReceiver},
};

use crate::{
    errors::{self, AppErrors},
    models::{IpPortScanResult, SubnetScanConfiguration},
    port_helpers, tokio_helpers,
};

use anyhow::bail;

pub async fn launch_subnet_scan_tasks(
    subnet_scan_configurations: Vec<SubnetScanConfiguration>,
    scan_timeout: Duration,
    handle: &'static Handle,
) -> Vec<(
    Box<impl Future<Output = ()> + 'static>,
    UnboundedReceiver<IpPortScanResult>,
)> {
    subnet_scan_configurations
        .into_iter()
        .map(|config| {
            let (tx, rx) = mpsc::unbounded_channel::<IpPortScanResult>();

            let scan_name = format!("scan_{}", config.subnet.to_string());

            let scan_fut = tokio_helpers::run_named_task(
                scan_name,
                handle,
                scan_ipv4_subnet(config, scan_timeout, tx),
            );

            (Box::new(scan_fut), rx)
        })
        .collect()
}

pub async fn scan_ipv4_subnet(
    scan_configuration: SubnetScanConfiguration,
    scan_timeout: Duration,
    tx: mpsc::UnboundedSender<IpPortScanResult>,
) -> anyhow::Result<()> {
    let SubnetScanConfiguration {
        subnet,
        begin_port,
        end_port,
    } = scan_configuration;

    for ip in subnet.hosts() {
        for port in begin_port..end_port {
            let scan_result =
                port_helpers::check_port_status_with_timeout(ip, port, scan_timeout).await;

            if let Err(send_error) = tx.send(scan_result) {
                bail!(AppErrors::IpScanResultChannelSendError {
                    channel: format!("subnet: {}", subnet),
                    result: scan_result,
                    source: send_error
                })
            }
        }
    }

    Ok(())
}
