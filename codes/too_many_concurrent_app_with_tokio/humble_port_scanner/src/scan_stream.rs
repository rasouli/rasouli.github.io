// TODO: merge channels into tokio stream
//   use the tokio stream to calculate the progress in each subnet
//   use ratatui to display : progress bar per each subnet and a scrollable table to
//   present the scan result.

use crate::models::IpPortScanResult;

struct ScanResultStreamer {}

impl ScanResultStreamer {}

impl Iterator for ScanResultStreamer {
    type Item = IpPortScanResult;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
