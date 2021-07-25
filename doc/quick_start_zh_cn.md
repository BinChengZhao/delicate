在部署之后，我们可以开始使用delicate的相关功能。

首先通过 .env (`INITIAL_ADMINISTRATOR_USER_NAME`) , 初始化的用户进行登陆。

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/login_zh.jpg"
         alt="" title="delicate" align="right"/>
</a>


首先，我们进入`执行资源-执行节点`将`delicate-executor`作为机器资源维护进入我们的系统，并执行激活操作。

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/executor_list.jpg"
         alt="" title="delicate" align="right"/>
</a>

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/executor_create.jpg"
         alt="" title="delicate" align="right"/>
</a>


然后，进入`执行资源-执行组`菜单添加我们的资源组，并与对应的`执行节点`做绑定生成`绑定项`。

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/group_list.jpg"
         alt="" title="delicate" align="right"/>
</a>


<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/group_inner_bind.jpg"
         alt="" title="delicate" align="right"/>
</a>

下一步，进入`任务列表`添加我们的调度任务，并关联我们的`绑定项`.

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/task_list.jpg"
         alt="" title="delicate" align="right"/>
</a>

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/task_edit.jpg"
         alt="" title="delicate" align="right"/>
</a>


<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/task_list_operation.jpg"
         alt="" title="delicate" align="right"/>
</a>

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/task_list.jpg"
         alt="" title="delicate" align="right"/>
</a>


当任务开始调度后，生成调度日志，我们可以通过任务列表下的指定任务点击`更多`按钮  -> `查看日志`。

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/task_logs_2.jpg"
         alt="" title="delicate" align="right"/>
</a>

每一个任务的运行日志都对应着一个`任务实例`, 在运行中的任务实例是可以随时取消的。已经运行完成，或者超时的任务可以查看任务执行后产生的标准输出与标准错误。

<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/task_log_kill.jpg"
         alt="" title="delicate" align="right"/>
</a>



更多内容请查看doc。