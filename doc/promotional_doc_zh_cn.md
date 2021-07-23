## [delicate](https://github.com/BinChengZhao/delicate) 一个轻量的分布式的任务调度平台通过rust编写.


<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/delicate_logo.png"
         alt="delicate logo" title="delicate" height="125" width="125"  align="right"/>
</a>

//TODO: there is features and pictures.
//TODO: 1.Introduction (repo address) 2.Function 3.Technology stack 4.roadmap 5.Thanks (repo address).

## 特性
- **友好的用户界面：** [前端]方便地管理任务和执行者，监控其状态，支持手动维护等。

- **灵活的操作：** 灵活的任务操作，支持限制单个节点的最大并行数，与cron表达式相对应的时区设置，调度模式（单一、固定数量、不断重复），能够在任何时候手动触发任务，手动终止任务实例，在线查看任务日志。

- **高可用性：** Delicate支持横向扩展。通过部署尽可能多的Delicate服务器和执行器，很容易实现高可用性和性能。

- **高性能：** 轻量级和基本功能加快了性能，`delicate'的基本资源开销大约是（小于0.1%的cpu使用率，10m的内存.)

- **可观察性:**有许多有意义的统计数据定期以图表的方式展现。

- **升级：**系统的动态升级（升级是通过获得最新的源代码和进行数据库迁移.)



![architecture](https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/delicate-architecture.svg)



## 技术栈

后端( scheduler & executor ): Rust  
主要的依赖: (actix-web & diesel & delay-timer & serde & tracing)

前端: antd-admin (React)

Ui: Ant Design

数据库: mysql , postgres (计划支持)



[Quick-start](https://github.com/BinChengZhao/delicate/blob/main/doc/promotional_doc_zh_cn.md)


## 下一步是什么
在今年的计划中，我们宣布了2021年Delicate的路线图。因此，我们将继续遵循这个路线图.

[Roadmap](https://github.com/BinChengZhao/delicate/blob/main/doc/Roadmap.md)

## 感谢

谢谢你

我们要感谢整个社区和代码贡献者。特别是，感谢过去两个月的代码贡献者:
[Walker-os](https://github.com/Walker-os)


感谢用户报告文档中的拼写错误, 这非常感谢大家。
感谢用户加入我们，提供反馈，讨论功能，并获得帮助!