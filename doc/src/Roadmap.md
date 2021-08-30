## What is delicate
<a href="">
    <img src="./delicate_logo.png"
         alt="delicate logo" title="delicate" height="125" width="125"  align="right"/>
</a>

`delicate` A lightweight and distributed task scheduling platform written in rust.

## features
- **Friendly UI:** [Front-end] Convenient management of tasks & executors, monitoring their status and supporting manual maintenance etc.

- **Flexible Operations:** Flexible task operations, support for limiting the maximum number of parallelism in a single node, time zone settings corresponding to cron expressions, scheduling modes (single, fixed number, constantly repeating), the ability to manually trigger tasks at any time, manually terminate task instances, and view task logs online.

- **High Availability:**  Delicate supports unlimited horizontal expansion. It's easy to achieve high availability and performance by deploying as many Delicate servers and executors.

- **High Performance:** Lightweight and essential features speed up the performance, The basic resource overhead for `delicate` is roughly (less than 0.1% cpu usage, with about 10m of memory.)

- **Observability:**  There are many meaningful statistics periodically in a readable way.

- **Upgrade:**  Dynamic upgrade of the system (upgrade is done by obtaining the latest source code and performing database migration.

- **Reusability:**  Excutor provides `restful-api` that allows user applications to maintain custom tasks.


## Delicate Roadmap

- [Delicate Roadmap](#delicate-roadmap)
  - [Product Principles](#product-principles)
  - [Features](#features)
    - [Business Extensibility](#business-extensibility)
  - [Roadmap 2021](#roadmap-2021)

## Product Principles
1. **Operability & Experience**.  Designed with easy-to-understand features that provide a user-friendly system experience, the job is done between silky smooth operations. 

### Business Extensibility
* Easy to develop new features with Battery-included Delicate.
* Easy to operate/easy to install.

## Roadmap 2021

| Name                         | Issue                                                | Description                                                                                                                                        |
| ---------------------------- | ---------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| I18n                         |                                                      | Internationalization, convenient for users in different countries and regions.                                                                     |
| Operation Log                         |                                                      | Record the user's operation behavior.                                                                     |
| Permission Management        |                                                      | Humanized permission management to protect machine task resources and maintain system security.                                                    |
| Multiple login protocols     |                                                      | Support multiple login protocols to open up the user system within the enterprise and enhance user experience.                                     |
| Machine resource panel       |                                                      | Powerful online resource panel with online visualization of machine / task support for multiple user actions.                                      |
| Failover       |                                                      | Automatically migrate tasks when a node is down.                                      |
| Upgrade to actix-web 4       |                                                      | Wait for actix-web 4 stable which will support with tokio 1 , So that `scheduler & executor` communication can using RPC  (`tonic ｜ tarpc`).      |
