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
``` 
