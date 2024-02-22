use crate::{models::SubnetScanConfiguration, subnet_helpers::parse_subnet};
use anyhow::anyhow;
use anyhow::Context;

pub fn prepare_subnets_and_port_ranges(
    subnets: Vec<String>,
    port_ranges: Vec<String>,
) -> anyhow::Result<Vec<SubnetScanConfiguration>> {
    subnets
        .into_iter()
        .zip(port_ranges)
        .map(|(subnet_str, port_range_str)| {
            parse_subnet(subnet_str).and_then(|subnet| {
                parse_port_ranges(port_range_str).map(|(begin_port, end_port)| {
                    SubnetScanConfiguration {
                        subnet,
                        begin_port,
                        end_port,
                    }
                })
            })
        })
        .collect()
}

pub fn parse_port_ranges(port_range: String) -> anyhow::Result<(u16, u16)> {
    let (begin_port_str, end_port_str) = port_range
        .split_once(':')
        .context("Port ranges should be seperated in following format: [begin_port]:[end_port]")?;

    if begin_port_str.is_empty() || end_port_str.is_empty() {
        return Err(anyhow!(
            "Empty port values given in the port range {}",
            port_range
        ));
    }

    let begin_port = begin_port_str
        .parse::<u16>()
        .context(format!(
            "Unable to parse the begining of port range: {}",
            begin_port_str
        ))?;

    let end_port = end_port_str
        .parse::<u16>()
        .context(format!(
            "Unable to parse the end of port range: {}",
            end_port_str
        ))?;

    if begin_port > end_port {
        return Err(anyhow!(
            "Begin port {} is bigger than the end port {}",
            begin_port,
            end_port
        ));
    }

    Ok((begin_port, end_port))
}

#[cfg(test)]
mod port_tests {
    use crate::arg_helpers::parse_port_ranges;

    #[test]
    fn parse_port_ranges_test() {
        assert_eq!(
            (5000_u16, 8000_u16),
            parse_port_ranges(String::from("5000:8000")).unwrap()
        );

        assert_eq!(
            "Port ranges should be seperated in following format: [begin_port]:[end_port]",
            parse_port_ranges(String::from("garBage"))
                .err()
                .unwrap()
                .to_string()
        );

        assert_eq!(
            format!("Unable to parse the begining of port range: {}", "garBage"),
            parse_port_ranges(String::from("garBage:5000"))
                .err()
                .unwrap()
                .to_string()
        );

        assert_eq!(
            format!("Unable to parse the end of port range: {}", "garBage"),
            parse_port_ranges(String::from("5000:garBage"))
                .err()
                .unwrap()
                .to_string()
        );

        assert_eq!(
            format!("Begin port {} is bigger than the end port {}", 8000, 5000),
            parse_port_ranges(String::from("8000:5000"))
                .err()
                .unwrap()
                .to_string()
        );
    }
}

#[cfg(test)]
mod parsing_input_arg_tests {

    #[test]
    fn prepare_subnets_and_port_ranges_test() {
        // TODO: implement this test
        //        assert_eq!(vec![Ok(SubnetScanConfiguration {})])
    }
}
