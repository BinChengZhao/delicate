[delicate](https://github.com/BinChengZhao/delicate) A lightweight and distributed task scheduling platform written in rust.


<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/delicate_logo.png"
         alt="delicate logo" title="delicate" height="125" width="125"  align="right"/>
</a>

## Features
- **Friendly UI:** [Front-end] Convenient management of tasks & executors, monitoring their status and supporting manual maintenance etc.

- **Flexible Operations:** Flexible task operations, support for limiting the maximum number of parallelism in a single node, time zone settings corresponding to cron expressions, scheduling modes (single, fixed number, constantly repeating), the ability to manually trigger tasks at any time, manually terminate task instances, and view task logs online.

- **High Availability:**  Delicate supports unlimited horizontal expansion. It's easy to achieve high availability and performance by deploying as many Delicate servers and executors.

- **High Performance:** Lightweight and essential features speed up the performance, The basic resource overhead for `delicate` is roughly (less than 0.1% cpu usage, with about 10m of memory.)

- **Observability:**  There are many meaningful statistics periodically in a readable way.

- **Upgrade:**  Dynamic upgrade of the system (upgrade is done by obtaining the latest source code and performing database migration.


![architecture](https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/delicate-architecture.svg)

## Technology-stack

Backend: Rust  
main-dependencies: (actix-web & diesel & delay-timer & serde & tracing)

Front-end: antd-admin (React)

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


## Renderings

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/dashboard.jpg"
         alt="" title="delicate" align="right"/>
</a>

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/task_list.jpg"
         alt="" title="delicate" align="right"/>
</a>

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/task_list_operation.jpg"
         alt="" title="delicate" align="right"/>
</a>

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/task_logs.jpg"
         alt="" title="delicate" align="right"/>
</a>

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/task_log_kill.jpg"
         alt="" title="delicate" align="right"/>
</a>

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/executor_list.jpg"
         alt="" title="delicate" align="right"/>
</a>

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/group_list.jpg"
         alt="" title="delicate" align="right"/>
</a>

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/group_inner_bind.jpg"
         alt="" title="delicate" align="right"/>
</a>

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/task_logs_2.jpg"
         alt="" title="delicate" align="right"/>
</a>

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/user_list.jpg"
         alt="" title="delicate" align="right"/>
</a>

