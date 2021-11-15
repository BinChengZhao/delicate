## Centralized scheduling support.

#### Weakly centralized scheduling: scheduling state is not easy to maintain, requires asynchronous processing, and is not easy to do DAG scheduling, but without fear of scheduler downtime, tasks are not affected by the executor.

#### Centralized scheduling: Scheduling state is easy to maintain, easy to add various states before and after tasks, also easy to do DAG scheduling, but catastrophic consequences when the scheduler is down.

## The current delicate scheduling solution is weakly centralized, and now we need to add a centralized scheduling solution.

### Centralized Scheduling Solution - (MVP).

Internal scheduling implemented through GRPC.

When the scheduler is deployed standalone, the state of tasks is easily maintained consistently.

When deployed in a cluster, there is some complexity in handling the state of tasks.

#### Example

Scheduler-Cluster has Server A B. Task-X is executed by Scheduler-A. 

Task-X can be closed normally if the user requests it via load balancing to Scheduler-A.

If the user requests to Scheduler-B via load balancing, **Scheduler-B needs to be notified by middleware to Scheduler-A**.


Add task scheduling type.

   Centralized (scheduler initiates scheduling, executor executes, DAG friendly)
   Weakly centralized (executor schedules and executes autonomously)


Execution modes (for centralized): broadcast, polling, random.

### Front-end change 

`/api/task/create` Input add field `schedule_type` `execute_mode`.

`/api/task/update`Input add field `schedule_type` `execute_mode`.

`/api/task/list` Output add field `schedule_type` `execute_mode`.


