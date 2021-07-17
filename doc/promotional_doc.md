[delicate](https://github.com/BinChengZhao/delicate) A lightweight and distributed task scheduling platform written in rust.

//TODO: there is features and pictures.
//TODO: 1.Introduction (repo address) 2.Function 3.Technology stack 4.roadmap 5.Thanks (repo address).

<a href="">
    <img src="https://github.com/BinChengZhao/delicate/blob/main/doc/delicate_logo.png"
         alt="delicate logo" title="delicate" height="125" width="125"  align="right"/>
</a>

## Features
- **Friendly UI:** [Front-end] Convenient management of tasks & executors, monitoring their status and supporting manual maintenance etc.

- **Flexible Operations:** Flexible task operations, support for limiting the maximum number of parallelism in a single node, time zone settings corresponding to cron expressions, scheduling modes (single, fixed number, constantly repeating), the ability to manually trigger tasks at any time, manually terminate task instances, and view task logs online.

- **High Availability:**  Delicate supports unlimited horizontal expansion. It's easy to achieve high availability and performance by deploying as many Delicate servers and executors.

- **High Performance:** Lightweight and essential features speed up the performance, The basic resource overhead for `delicate` is roughly (less than 0.1% cpu usage, with about 10m of memory.)

- **Observability:**  There are many meaningful statistics periodically in a readable way.

- **Upgrade:**  Dynamic upgrade of the system (upgrade is done by obtaining the latest source code and performing database migration.


![architecture](https://github.com/BinChengZhao/delicate/blob/main/doc/architecture.svg)

## Technology-stack

Backend: Rust  
main-dependencies: (actix-web & diesel & delay-timer & serde & tracing)

Front-end: React.js

Ui: Ant Design

Database: mysql , postgres (plan support)


[Quick-start](https://github.com/BinChengZhao/delicate/blob/main/doc/quick_start.md)



## What's next#
At the this year, we announced our 2021 roadmap for Delicate. So we will continue to follow this roadmap.

[Roadmap](https://github.com/BinChengZhao/delicate/blob/main/doc/Roadmap.md)

## Thanks

Thank you!

We would like to thank the whole community and code contributors. In particular, thanks to the code contributors from the past two months:
[Walker-os](https://github.com/Walker-os)


Thanks to users reporting spelling mistakes on the documentation. This is always appreciated.
Thanks to users joining us provide feedbacks, discuss features, and get assistance!