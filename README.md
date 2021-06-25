# English | [简体中文](./README_zhCN.md)

- [delicate](#delicate)
  - [What is delicate](#what-is-delicate)
  - [Features](#features)
  - [Benchmark](#benchmark)
  - [Get Started](#get-started)
    - [Setting up delicate](#setting-up-delicate)
  - [Documentation](#documentation)
  - [Roadmap](#roadmap)
  - [License](#license)

## What is delicate
<a href="">
    <img src="./doc/delicate.svg"
         alt="delicate logo" title="delicate" height="100" width="100"  align="right"/>
</a>

`delicate` A lightweight and distributed task scheduling platform written in rust.:

## features
- **Friendly UI:** [Front-end] *** manage their task | executor, monitor the status, check the logs online, etc.
- **High Availability:**  Delicate supports unlimited horizontal expansion. It's easy to achieve high availability and performance by deploying as many Delicate servers and executors.
- **High Performance:** Lightweight and essential features speed up the performance.
- **Observability:**  There are many meaningful statistics periodically in a readable way.
- **Integration:**  etc.

The architecture of delicate:

![architecture](./doc/architecture.png)

- **Service Management**
	- **Security**
- **High Performance and Availability**
	- **Adaption**: .
	- **Validation**: .
	- **Load Balance:** .
	- **Cache:** .
	- **Hot-Update:** updates both config and binary of delicate in place without losing connections.

## Benchmark

## Get Started

The basic common usage of delicate is to quickly set up for the backend servers and executors. We split it into multiple simple steps to illustrate the delicate concepts and operations.

### Setting up delicate

We can download the binary from [release page](https://github.com/BinChengZhao/delicate/releases). For example we use linux version:

```bash
$ mkdir delicate
$ wget https://github.com/BinChengZhao/delicate/releases/download/v1.0.0/delicate-v1.0.0-linux-amd64.tar.gz
$ tar zxvf delicate-v1.0.0-linux-amd64.tar.gz -C delicate && cd delicate
```

or use source code:

```bash
$ git clone https://github.com/BinChengZhao/delicate && cd delicate
$ cargo *****
```


## Documentation

See [reference](./doc/reference.md) and [developer guide](./doc/developer-guide.md) for more information.

## Roadmap 

See [delicate Roadmap](./doc/Roadmap.md) for details.

# Stargazers over time

[![Stargazers over time](https://starchart.cc/BinChengZhao/delicate.svg)](https://starchart.cc/BinChengZhao/delicate)


## License

Licensed under either of

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## To Do List

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
