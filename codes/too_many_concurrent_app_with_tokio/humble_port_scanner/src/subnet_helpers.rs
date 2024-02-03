use anyhow::Context;
use ipnet::Ipv4Net;

use crate::models::SubnetScanConfiguration;

pub async fn scan_ipv4_subnet(scan_configuration: SubnetScanConfiguration) {
    let SubnetScanConfiguration {
        subnet,
        begin_port,
        end_port,
    } = scan_configuration;

    for ip in subnet.hosts() {
        for port in begin_port..end_port {}
    }
}

pub fn parse_subnet(subnet: String) -> anyhow::Result<Ipv4Net> {
    subnet
        .parse::<Ipv4Net>()
        .context(format!("Unable to parse subnet: {}", subnet))
}

#[cfg(test)]
mod subnet_tests {
    use std::net::Ipv4Addr;

    use ipnet::Ipv4Net;

    use crate::subnet_helpers::parse_subnet;

    #[test]
    fn parse_subnet_test() {
        assert_eq!(
            format!("Unable to parse subnet: {}", "garBage"),
            parse_subnet(String::from("garBage"))
                .err()
                .unwrap()
                .to_string()
        );

        assert_eq!(
            Ipv4Net::new(Ipv4Addr::new(172, 16, 0, 0), 16).unwrap(),
            parse_subnet(String::from("172.16.0.0/16")).unwrap()
        )
    }
}
