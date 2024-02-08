use std::time::Duration;
use std::{error::Error, future::Future, iter, net::Ipv4Addr, sync::Arc};

use clap::Parser;

use ipnet::Ipv4Net;

use anyhow::{anyhow, Context, Result};
use tokio::net::TcpStream;
use tokio::runtime::{self, Runtime};

use tokio::{task, time};

use crate::models::PortScannerArgs;

mod app;
mod arg_helpers;
mod errors;
mod models;
mod port_helpers;
mod scan_stream;
mod subnet_helpers;
mod tokio_helpers;

fn main() {
    let PortScannerArgs { subnets, ports } = PortScannerArgs::parse();

    if subnets.len() != ports.len() {
        println!(
            "Number of subnets and ports must be equal. subnets count was {}, ports count was: {}",
            subnets.len(),
            ports.len()
        );

        return;
    }

    let _ = tokio_helpers::setup_tokio_runtime();

    println!("all  good.");
}
