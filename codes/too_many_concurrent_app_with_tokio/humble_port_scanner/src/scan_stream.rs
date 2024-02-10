// TODO: merge channels into tokio stream
//   use the tokio stream to calculate the progress in each subnet
//   use ratatui to display : progress bar per each subnet and a scrollable table to
//   present the scan result.

use std::{future::Future, pin::Pin};

use futures_core::Stream;
use ipnet::Ipv4Subnets;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_stream::{StreamExt, StreamMap, StreamNotifyClose};

use crate::models::IpPortScanResult;

struct ScanResultStreamer {
    futures: Vec<Box<dyn Future<Output = ()>>>,
    stream_map: Pin<
        Box<
            StreamMap<
                Ipv4Subnets,
                StreamNotifyClose<Pin<Box<dyn Stream<Item = IpPortScanResult>>>>,
            >,
        >,
    >,
}

impl ScanResultStreamer {
    pub fn new(
        scan_tasks: Vec<(
            Ipv4Subnets,
            Box<dyn Future<Output = ()> + 'static>,
            UnboundedReceiver<IpPortScanResult>,
        )>,
    ) -> Self {
        let mut futures: Vec<Box<dyn Future<Output = ()>>> = Vec::new();
        let mut stream_map = StreamMap::new();

        for (subnet, scan_task, rx) in scan_tasks {
            let rx_stream = StreamNotifyClose::new(Self::make_stream(rx));

            stream_map.insert(subnet, rx_stream);
            futures.push(scan_task);
        }

        Self {
            futures,
            stream_map: Box::pin(stream_map),
        }
    }

    fn make_stream(
        mut rx: UnboundedReceiver<IpPortScanResult>,
    ) -> Pin<Box<dyn Stream<Item = IpPortScanResult>>> {
        Box::pin(async_stream::stream! {
                while let Some(scan_result) = rx.recv().await {
                    yield scan_result;
                }
        })
    }
}

impl Stream for ScanResultStreamer {
    type Item = (Ipv4Subnets, Option<IpPortScanResult>);

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        Stream::poll_next(self.stream_map.as_mut(), cx)
    }
}
