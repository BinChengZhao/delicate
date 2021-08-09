# Version 1.1.0

- More tracking of user behavior, more advancement of auxiliary functions, and better applications etc.

1. stock issues are all solved.

2. operation logs, login logs, I18N, heartbeat checks, robust event callbacks, related refinements.

3. Permission module (do research and design model in this period).


# Version 1.0.0

- Delicate a lightweight and distributed task scheduling platform written in rust.

## Features
- **Friendly UI:** [Front-end] Convenient management of tasks & executors, monitoring their status and supporting manual maintenance etc.

- **Flexible Operations:** Flexible task operations, support for limiting the maximum number of parallelism in a single node, time zone settings corresponding to cron expressions, scheduling modes (single, fixed number, constantly repeating), the ability to manually trigger tasks at any time, manually terminate task instances, and view task logs online.

- **High Availability:**  Delicate supports unlimited horizontal expansion. It's easy to achieve high availability and performance by deploying as many Delicate servers and executors.

- **High Performance:** Lightweight and essential features speed up the performance, The basic resource overhead for `delicate` is roughly (less than 0.1% cpu usage, with about 10m of memory.)

- **Observability:**  There are many meaningful statistics periodically in a readable way.

- **Upgrade:**  Dynamic upgrade of the system (upgrade is done by obtaining the latest source code and performing database migration.
