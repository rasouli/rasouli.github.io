use std::{net::Ipv4Addr, time::Duration};

use crate::models::{IpPortScanResult, PortState};
use anyhow::{anyhow, Context};
use tokio::{net::TcpStream, time};

pub async fn check_port_status_with_timeout(
    ip: Ipv4Addr,
    port: u16,
    timeout: Duration,
) -> IpPortScanResult {
    let stream = time::timeout(timeout, async { TcpStream::connect((ip, port)).await }).await;

    let state = match stream {
        Err(_) => PortState::TimeOut,
        Ok(tcp_result) if tcp_result.is_ok() => PortState::Open,
        _ => PortState::Closed,
    };

    IpPortScanResult { ip, port, state }
}

#[cfg(test)]
mod port_status_tests {
    use std::{net::Ipv4Addr, time::Duration};

    use anyhow::Context;
    use tokio::sync::oneshot;

    use crate::{
        models::{IpPortScanResult, PortState},
        port_helpers::check_port_status_with_timeout,
    };

    #[tokio::test]
    async fn should_return_closed_state_for_a_closed_port() {
        let scan_result = check_port_status_with_timeout(
            "127.0.0.1"
                .parse::<Ipv4Addr>()
                .unwrap(),
            15411_u16,
            Duration::from_secs(1),
        )
        .await;
        assert_eq!(scan_result.state, PortState::Closed);
    }

    #[tokio::test]
    async fn should_return_timeout_state_for_non_routable_ip() {
        // hopefully there is no NAT setup on this ip.
        let scan_result = check_port_status_with_timeout(
            "172.31.255.255"
                .parse::<Ipv4Addr>()
                .unwrap(),
            5432_u16,
            Duration::from_secs(1),
        )
        .await;
        assert_eq!(scan_result.state, PortState::TimeOut);
    }

    #[tokio::test]
    async fn should_return_open_port_state_for_open_port() {
        let random_port_tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .context(
                "Unable to bind to a random tcp port to test for an open port state on 127.0.0.1",
            )
            .unwrap();

        let random_port = random_port_tcp_listener
            .local_addr()
            .context("Unable to get local address on the bound random port on 127.0.0.1")
            .unwrap()
            .port();

        // just accept any incoming connection and dorp it afterwards.
        let random_socket_handler = async move {
            random_port_tcp_listener
                .accept()
                .await
                .context(format!(
                    "Unable to accept connection on the random port {}",
                    random_port
                ))
                .unwrap();

            tokio::time::sleep(Duration::from_millis(200));
        };

        // launch the socket handler so we can proceed with the test in current
        // tokio task.
        tokio::spawn(random_socket_handler);

        // define the time interval in which this task needs to succeed, otherwise fail it.
        let mut test_interval_timeout = tokio::time::interval(Duration::from_secs(3));

        loop {
            tokio::select! {
                _ = test_interval_timeout.tick() => {
                    assert!(false, "Test could not copmelet within the acceptable time.");
                    break;
                },
                scan_result = check_port_status_with_timeout(
                        "127.0.0.1".parse::<Ipv4Addr>().unwrap(),
                        random_port,
                        Duration::from_secs(1)) => {

                    if let IpPortScanResult{state: PortState::Open, ..} = scan_result {
                        assert!(true);
                        break;
                    }
                },
            } // select!
        } // loop
    }
}
