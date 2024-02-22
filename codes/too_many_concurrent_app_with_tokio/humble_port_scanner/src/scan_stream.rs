use std::pin::Pin;

use futures_core::Stream;
use ipnet::Ipv4Net;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_stream::{StreamMap, StreamNotifyClose};

use crate::models::IpPortScanResult;

pub struct ScanResultStreamer {
    stream_map: Pin<
        Box<
            StreamMap<
                Ipv4Net,
                StreamNotifyClose<Pin<Box<dyn Stream<Item = IpPortScanResult> + Send>>>,
            >,
        >,
    >,
}

impl ScanResultStreamer {
    pub fn new() -> Self {
        let mut stream_map = StreamMap::new();

        Self {
            stream_map: Box::pin(stream_map),
        }
    }

    pub fn add_stream_from_rx(
        &mut self,
        key: Ipv4Net,
        mut rx: UnboundedReceiver<IpPortScanResult>,
    ) {
        let rx_stream = StreamNotifyClose::new(ScanResultStreamer::make_stream(rx));
        self.stream_map
            .insert(key, rx_stream);
    }

    fn make_stream(
        mut rx: UnboundedReceiver<IpPortScanResult>,
    ) -> Pin<Box<dyn Stream<Item = IpPortScanResult> + Send>> {
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
