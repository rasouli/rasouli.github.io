use std::{
    future::Future,
    pin::Pin,
    sync::{Arc},
    time::Duration,
};


use tokio::{
    runtime::{Runtime},
    sync::mpsc::{self},
};
use tokio_stream::StreamExt;

use crate::{
    errors::{self, AppErrors},
    models::{IpPortScanResult, SubnetScanConfiguration},
    port_helpers,
    progress_helper::ScanProgressTracker,
    scan_stream::ScanResultStreamer,
    tokio_helpers,
};

use anyhow::bail;

const PROGRESS_BAR_SIZE: u64 = 100;

pub struct SubnetScannerApp {
    subnet_scan_configurations: Vec<SubnetScanConfiguration>,
    scan_timeout: Duration,
    runtime: Arc<Runtime>,
    scan_results: ScanResultStreamer,
    scan_progress: ScanProgressTracker,
    scan_futures: Vec<Pin<Box<dyn Future<Output = ()>>>>,
}

impl SubnetScannerApp {
    pub fn builder() -> SubnetScannerAppBuilder {
        SubnetScannerAppBuilder::new()
    }

    pub fn start_subnet_scans(&mut self) {
        for config in &self.subnet_scan_configurations {
            let (tx, rx) = mpsc::unbounded_channel::<IpPortScanResult>();

            let config = config.clone();
            let runtime = self.runtime.clone();
            let timeout = self.scan_timeout.clone();
            let scan_name = format!("scan_{}", config.subnet);

            self.scan_results
                .add_stream_from_rx(config.subnet, rx);

            self.scan_progress
                .initate_subnet_progress(
                    config.subnet,
                    (config.end_port - config.begin_port) as u64,
                );
            let scan_fut = tokio_helpers::run_named_task(
                scan_name,
                runtime,
                Self::scan_ipv4_subnet(config, timeout, tx),
            );

            self.scan_futures
                .push(Box::pin(scan_fut));
        }
    }

    async fn stream_progress(
        mut scan_stream: ScanResultStreamer,
        mut scan_progress: ScanProgressTracker,
    ) {
        while let Some((subnet, scan_result)) = scan_stream.next().await {
            match scan_result {
                Some(_port_scan_result) => scan_progress.update_progress(subnet),
                None => scan_progress.complete_progress(subnet),
            }
        }
    }

    pub fn run(self) {
        let runtime = self.runtime;
        let scan_stream = self.scan_results;
        let scan_progress = self.scan_progress;
        let mut tasks = self.scan_futures;
        let progerss_fut = tokio_helpers::run_named_task(
            String::from("stream_progress"),
            runtime.clone(),
            Self::stream_progress(scan_stream, scan_progress),
        );

        tasks.push(Box::pin(progerss_fut));
        runtime.block_on(futures::future::join_all(tasks));
    }

    async fn scan_ipv4_subnet(
        config: SubnetScanConfiguration,
        scan_timeout: Duration,
        tx: mpsc::UnboundedSender<IpPortScanResult>,
    ) -> anyhow::Result<()> {
        for ip in config.subnet.hosts() {
            for port in config.begin_port..config.end_port {
                let scan_result =
                    port_helpers::check_port_status_with_timeout(ip, port, scan_timeout).await;

                if let Err(send_error) = tx.send(scan_result) {
                    bail!(AppErrors::IpScanResultChannelSendError {
                        channel: format!("subnet: {}", config.subnet),
                        result: scan_result,
                        source: send_error
                    })
                }
            }
        }

        Ok(())
    }
}

pub struct SubnetScannerAppBuilder {
    subnet_scan_configurations: Vec<SubnetScanConfiguration>,
    scan_timeout: Duration,
    runtime: Option<Arc<Runtime>>,
}

impl SubnetScannerAppBuilder {
    pub fn new() -> Self {
        SubnetScannerAppBuilder {
            subnet_scan_configurations: Vec::new(),
            scan_timeout: Duration::from_secs(1),
            runtime: None,
        }
    }

    pub fn set_configs(mut self, subnet_scan_configurations: Vec<SubnetScanConfiguration>) -> Self {
        self.subnet_scan_configurations
            .extend(subnet_scan_configurations);
        self
    }

    pub fn set_scan_timeout(mut self, scan_timeout: Duration) -> Self {
        self.scan_timeout = scan_timeout;
        self
    }

    pub fn set_runtime(mut self, runtime: &Arc<Runtime>) -> Self {
        self.runtime = Some(runtime.clone());
        self
    }

    pub fn build(self) -> anyhow::Result<SubnetScannerApp> {
        if self.runtime.is_none() {
            bail!(errors::AppErrors::NoRuntimeProvidedError)
        }

        let subnet_config_size = self
            .subnet_scan_configurations
            .len();

        Ok(SubnetScannerApp {
            subnet_scan_configurations: self.subnet_scan_configurations,
            scan_timeout: self.scan_timeout,
            runtime: self.runtime.unwrap(),
            scan_results: ScanResultStreamer::new(),
            scan_progress: ScanProgressTracker::new(PROGRESS_BAR_SIZE),
            scan_futures: Vec::with_capacity(subnet_config_size),
        })
    }
}
