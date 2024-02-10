use std::{collections::HashMap, ops::Add};

use crate::{models::IpPortScanResult, scan_stream::ScanResultStreamer};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use ipnet::Ipv4Net;
use tokio_stream::StreamExt;

pub struct ScanProgress {
    scan_streamer: ScanResultStreamer,
    subnet_to_pb: HashMap<Ipv4Net, ProgressBar>,
    subnet_progress: HashMap<Ipv4Net, u64>,
    subnet_host_size: HashMap<Ipv4Net, u64>,
    multi_pb: MultiProgress,
}

const PROGRESS_BAR_SIZE: u64 = 200;

impl ScanProgress {
    pub fn new(scan_streamer: ScanResultStreamer, subnets: Vec<Ipv4Net>) -> Self {
        let multi_pb = MultiProgress::new();
        let style = Self::get_style();

        let mut subnet_to_pb: HashMap<Ipv4Net, ProgressBar> = HashMap::new();
        let mut subnet_progress: HashMap<Ipv4Net, u64> = HashMap::new();
        let mut subnet_host_size: HashMap<Ipv4Net, u64> = HashMap::new();

        for subnet in subnets {
            let pb = multi_pb.add(ProgressBar::new(PROGRESS_BAR_SIZE));
            pb.set_style(style.clone());
            subnet_to_pb.insert(subnet, pb);
            subnet_progress.insert(subnet, 0);
            subnet_host_size.insert(subnet, subnet.hosts().count() as u64);
        }

        Self {
            scan_streamer,
            subnet_progress,
            subnet_to_pb,
            multi_pb,
            subnet_host_size,
        }
    }

    pub async fn present_progress(&mut self) {
        // while let Some(subnet, scan_result) = self.scan_streamer.next().await {}
        while let Some((subnet, scan_result)) = self.scan_streamer.next().await {
            match scan_result {
                Some(port_scan_result) => self.update_progress(&subnet),
                None => self.complete_progress(&subnet),
            }
        }
    }

    pub fn update_progress(&mut self, subnet: &Ipv4Net) {
        println!("progress...");
        self.subnet_progress
            .entry(*subnet)
            .and_modify(|v| *v += 1);

        self.subnet_to_pb[subnet].set_position(
            PROGRESS_BAR_SIZE * (self.subnet_progress[subnet] / self.subnet_host_size[subnet]),
        )
    }

    fn get_style() -> ProgressStyle {
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-")
    }

    fn complete_progress(&mut self, subnet: &Ipv4Net) {
        self.subnet_to_pb[subnet]
            .finish_with_message(format!("subnet {} scanning is done!", subnet));
    }
}
