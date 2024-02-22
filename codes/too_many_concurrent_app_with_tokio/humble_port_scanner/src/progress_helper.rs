use std::collections::HashMap;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use ipnet::Ipv4Net;

pub struct ScanProgressTracker {
    subnet_to_pb: HashMap<Ipv4Net, ProgressBar>,
    subnet_progress: HashMap<Ipv4Net, u64>,
    subnet_total_scans: HashMap<Ipv4Net, u64>,
    multi_pb: MultiProgress,
    progress_bar_size: u64,
}

impl ScanProgressTracker {
    pub fn new(progress_bar_size: u64) -> Self {
        let multi_pb = MultiProgress::new();
        let subnet_to_pb: HashMap<Ipv4Net, ProgressBar> = HashMap::new();
        let subnet_progress: HashMap<Ipv4Net, u64> = HashMap::new();
        let subnet_total_scans: HashMap<Ipv4Net, u64> = HashMap::new();

        Self {
            subnet_progress,
            subnet_to_pb,
            multi_pb,
            subnet_total_scans,
            progress_bar_size,
        }
    }

    pub fn initate_subnet_progress(&mut self, subnet: Ipv4Net, num_ports: u64) {
        let style = Self::get_style();
        let pb = self
            .multi_pb
            .add(ProgressBar::new(self.progress_bar_size));
        pb.set_style(style.clone());
        self.subnet_to_pb
            .insert(subnet, pb);
        self.subnet_progress
            .insert(subnet, 0);
        self.subnet_total_scans
            .insert(subnet, (subnet.hosts().count() as u64) * num_ports);
    }

    pub fn update_progress(&mut self, subnet: Ipv4Net) {
        self.subnet_progress
            .entry(subnet)
            .and_modify(|v| *v += 1);

        let position = self.progress_bar_size
            * (self.subnet_progress[&subnet] / self.subnet_total_scans[&subnet]);
        self.subnet_to_pb[&subnet].set_position(position);
    }

    fn get_style() -> ProgressStyle {
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-")
    }

    pub fn complete_progress(&mut self, subnet: Ipv4Net) {
        self.subnet_to_pb[&subnet]
            .finish_with_message(format!("subnet {} scanning is done!", subnet));
    }
}
