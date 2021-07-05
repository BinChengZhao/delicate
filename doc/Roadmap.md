# Delicate Roadmap

- [Delicate Roadmap](#Delicate-roadmap)
  - [Product Principles](#product-principles)
  - [Features](#features)
    - [Business Extensibility](#business-extensibility)
  - [Roadmap 2021](#roadmap-2021)
    - [Business Extensibility](#business-extensibility-1)

## Product Principles
1. **Opening & Extensibility**.  It aims to be an **extensible-development** platform. Users can organize the existing filters into a pipeline, or completely customize a brand-new filter/controller for their specific business logic. With simple, clean, and flat software architecture, anyone can develop their own filter/controller/pipeline rapidly and easily. 
  
## Features
Based on our product principles, we have made a classification of Delicate's features for powering users' business capabilities into two categories: business-specific .
### Business Extensibility
* Easy to develop new features with Battery-included Delicate.
* Easy to operate/easy to install.

### Traffic Orchestration 
* Supporting traffic management -  load balance, rate limiting, etc. 
* xxxxxx
* yyyyyy


## Roadmap 2021
### Business Extensibility

| Name                         | Issue                                                | Description                                                                                                                                        |
| ---------------------------- | ---------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| I18n                         |                                                      | Internationalization, convenient for users in different countries and regions.                                                                     |
| Permission Management        |                                                      | Humanized permission management to protect machine task resources and maintain system security.                                                    |
| Multiple login protocols     |                                                      | Support multiple login protocols to open up the user system within the enterprise and enhance user experience.                                     |
| Machine resource panel       |                                                      | Powerful online resource panel with online visualization of machine / task support for multiple user actions.                                      |
| Upgrade to actix-web 4       |                                                      | Wait for actix-web 4 stable which will support with tokio 1 , So that `scheduler & executor` communication can using RPC  (`tonic ｜ tarpc`).      |



- [ ] I18n.
- [ ] Permission Management.
- [ ] Multiple login protocols, LDAP CAS .
- [ ] Machine resource panel, online view of processes, memory, cpu, etc.
- [ ] Database back-end support Postgres.
- [ ]  `scheduler & executor` communication using RPC, but currently there are problems with dependencies (RPC framework (`tonic ｜ tarpc`) both depend on tokio 1,current actix-web stable version 3, does not support integration with tokio 1 ).