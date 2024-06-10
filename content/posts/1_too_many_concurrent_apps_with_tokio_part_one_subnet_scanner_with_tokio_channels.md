+++
title = 'Too Many Concurrent Apps with Tokio Part 1: A Humble Subnet Scanner with Tokio Channels'
date = 2023-11-26T17:26:12+01:00 
draft = true 
slug = '2023-11-26-tmcawt-p1-humble-subnet-scanner-tokio-channel'
tags = ['tokio', 'channels', 'rust', 'io', 'tokio-console']
+++

## Premier
Too Many Concurrent Apps with Tokio is my attempt at exploring and learning Tokio ecosystem while trying to use various Tokio features with practical use-cases; somehow more than just basic examples.


## Our Humble Subnet Scanner
Welcome to Part 1 of these article series, where we will build up a simple Port Scanner on top of Tokio Runtime. Our port scanner will accept bunch of network subnets and a port range, and tries to discover open ports on all IPs on those subnets.

In a nutshell once we run a command like this:
```bash
  $ cargo run -- -s 192.168.0.0/16 -p 8000:8050

```

our desired output would be:

```text
PortClosed for host 192.168.0.1 on port 8000
PortClosed for host 192.168.0.1 on port 8001
PortClosed for host 192.168.0.1 on port 8002
PortClosed for host 192.168.0.1 on port 8003
// Rest is omitted
<<<<<<< Updated upstream
```

### Using Clap to parse arguments

Let's start by introducing `Clap` to our Cargo project dependencies to start with parsing arguments, on the command line:

```bash
$ cargo add clap@4.4.11 --features derive 
```

Next we would like to accepts any number of subnets and port ranges to scan, but we only would care if number of subnets and
port ranges match. In file `main.rs`:  

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct PortScannerArgs {
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    subnets: Vec<String>,
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    ports: Vec<String>,
}

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

    println!("all  good.");
}
```

#### Parsing Subnets and Ports

Having ports and subnets parsed from command line as simple String, is not enough. What we would like to do is to iterate over all the
ip addresses in each subnet. On the other hand we also need to be able to extract ports from a string with format of `{port-begin}:{port-end}`, parsing ports as a tuple of `u8`, or more accurately: `(u8, u8)` should be enough for our purpose.

To work with subnets we would rely on `ipnet` crate:

```bash
$ cargo add ipnet@2.9.0
```

Let's introduce a new function that parses subnets using `ipnet` and their associated port ranges next:

```rust

```
### Setting up Tokio Runtime


### Checking Status of a Port

### Using Tokio Channels to communicate the results

### Using Tokio Monitors to see what is going on

### Wrapping Up
=======
``` 

## Setting up the basic structure

Now let's setup the basic app structure before proceeding. In this section we will set-up basic app structure before proceeding to next section where we will implement the main functionality using Tokio channels.

### Parsing command line arguments with Clap
hint use clap arg group

### Iterating through IPs in the Subnet 
>>>>>>> Stashed changes
