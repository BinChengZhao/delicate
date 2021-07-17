## [delicate](https://github.com/BinChengZhao/delicate) 一个轻量的分布式的任务调度平台通过rust编写.


<a href="">
    <img src="https://imgcdn.nicetuan.net/vc/202107/3c397f608aa7f29f23237c97803376f8.png"
         alt="delicate logo" title="delicate" height="125" width="125"  align="right"/>
</a>

//TODO: there is features and pictures.
//TODO: 1.Introduction (repo address) 2.Function 3.Technology stack 4.roadmap 5.Thanks (repo address).

## Features
- **Friendly UI:** [Front-end] Convenient management of tasks & executors, monitoring their status and supporting manual maintenance etc.

- **Flexible Operations:** Flexible task operations, support for limiting the maximum number of parallelism in a single node, time zone settings corresponding to cron expressions, scheduling modes (single, fixed number, constantly repeating), the ability to manually trigger tasks at any time, manually terminate task instances, and view task logs online.

- **High Availability:**  Delicate supports unlimited horizontal expansion. It's easy to achieve high availability and performance by deploying as many Delicate servers and executors.

- **High Performance:** Lightweight and essential features speed up the performance, The basic resource overhead for `delicate` is roughly (less than 0.1% cpu usage, with about 10m of memory.)

- **Observability:**  There are many meaningful statistics periodically in a readable way.

- **Upgrade:**  Dynamic upgrade of the system (upgrade is done by obtaining the latest source code and performing database migration.




![architecture](https://imgcdn.nicetuan.net/vc/202107/447d30e952f75988f31835b82289fdd7.png)



## Technology-stack

Backend: Rust  
main-dependencies: (actix-web & diesel & delay-timer & serde & tracing)

Front-end: React.js

Ui: Ant Design

Database: mysql , postgres (plan support)



[Quick-start](https://github.com/BinChengZhao/delicate/blob/main/doc/promotional_doc_zh_cn.md)


## What's next#
At the this year, we announced our 2021 roadmap for Delicate. So we will continue to follow this roadmap.

[Roadmap](https://github.com/BinChengZhao/delicate/blob/main/doc/Roadmap.md)

## Thanks

Thank you!

We would like to thank the whole community and code contributors. In particular, thanks to the code contributors from the past two months:
[Walker-os](https://github.com/Walker-os)


Thanks to users reporting spelling mistakes on the documentation. This is always appreciated.
Thanks to users joining us provide feedbacks, discuss features, and get assistance!