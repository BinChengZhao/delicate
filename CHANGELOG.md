# Version 1.1.0 [unreleased]

- More tracking of user behavior, more advancement of auxiliary functions, and better applications etc.

1. stock issues are all solved.

2. operation logs, login logs, I18N, heartbeat checks, robust event callbacks, related refinements.

3. Permission module (do research and design model in this period).
All notable changes to this project will be documented in this file.

### Documentation

- Optimize delicate-web's doc.

### Features

- Log detail & task advance .
- Explore permission-related features.
- Implement task-log deletion function.
- Optimize casbin-auth components.

### Fix

- Operation log details id batch assign issue.
- Update executor-process-bind task mess error.

### Miscellaneous Tasks

- Optimize code & casbin import.
- Handle some compile error.
- Optimize code and append auth-conf.
- Update doc & Optimize code.
- Optimize state-desc in API-Response.

### Performance

- Optimize task-log deletion fuction.


# Version 1.0.0

- Delicate a lightweight and distributed task scheduling platform written in rust.

## Features
- **Friendly UI:** [Front-end] Convenient management of tasks & executors, monitoring their status and supporting manual maintenance etc.

- **Flexible Operations:** Flexible task operations, support for limiting the maximum number of parallelism in a single node, time zone settings corresponding to cron expressions, scheduling modes (single, fixed number, constantly repeating), the ability to manually trigger tasks at any time, manually terminate task instances, and view task logs online.

- **High Availability:**  Delicate supports unlimited horizontal expansion. It's easy to achieve high availability and performance by deploying as many Delicate servers and executors.

- **High Performance:** Lightweight and essential features speed up the performance, The basic resource overhead for `delicate` is roughly (less than 0.1% cpu usage, with about 10m of memory.)

- **Observability:**  There are many meaningful statistics periodically in a readable way.

- **Upgrade:**  Dynamic upgrade of the system (upgrade is done by obtaining the latest source code and performing database migration.
