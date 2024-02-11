use std::pin::Pin;

use futures_core::Stream;
use ipnet::Ipv4Net;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_stream::{StreamMap, StreamNotifyClose};

use crate::models::IpPortScanResult;

pub struct ScanResultStreamer {
    stream_map: Pin<
        Box<StreamMap<Ipv4Net, StreamNotifyClose<Pin<Box<dyn Stream<Item = IpPortScanResult>>>>>>,
    >,
}

impl ScanResultStreamer {
    pub fn new(scan_tasks: Vec<(Ipv4Net, UnboundedReceiver<IpPortScanResult>)>) -> Self {
        let mut stream_map = StreamMap::new();

        for (subnet, rx) in scan_tasks {
            let rx_stream = StreamNotifyClose::new(Self::make_stream(rx));
            stream_map.insert(subnet, rx_stream);
        }

        Self {
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
    type Item = (Ipv4Net, Option<IpPortScanResult>);

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        Stream::poll_next(self.stream_map.as_mut(), cx)
    }
}
