
- [Executor Group](#ExecutorGroup)
    - [Configuration](#configuration)

- [Executor Processor](#ExecutorProcessor)
    - [Configuration](#configuration)

- [Executor Processor Bind](#ExecutorProcessorBind)
    - [Configuration](#configuration)

- [Task](#Task)
    - [Configuration](#configuration)


## ExecutorGroup

`Executor Group` which corresponds to a service, or a business.

<a href="">
    <img src="./doc/delicate_logo.png"
         alt="Executor Group" title="Executor Group"  />
</a>


### Configuration

| Name           | Type                                               | Description                                                                                                                                     | Required |
| -------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| *   | int64                                              | a                                                                        | No       |
| * | boolean                                            |   b     | No       |







## ExecutorProcessor

`Executor Processor` which corresponding to a physical machine, or a container.

<a href="">
    <img src="./doc/delicate_logo.png"
         alt="Executor Group" title="Executor Group"  />
</a>


### Configuration

| Name           | Type                                               | Description                                                                                                                                     | Required |
| -------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| maxBodyBytes   | int64                                              | c                                                                         | No       |
| partialSucceed | boolean                                            | d       | No       |
| timeout        | string                                             |e                                                                                                  | No       |
| mergeResponse  | boolean                                            | f | No       |




## ExecutorProcessorBind

`Executor Processor Bind`, which corresponds to the association between a `Executor Group` and a `Executor Processor`, and the task needs to select the machine(Executor Processor) to execute through the association `Executor Processor Bind`. 


Tip:

When there are hundreds of tasks associated with a certain `Executor Processor Bind`, it is easy to switch the machine that executes the task if you want to. Just modify the `Executor Processor` associated with the `Executor Processor Bind`, then hundreds of tasks will be removed from the old machine and executed on the new machine.

<a href="">
    <img src="./doc/a.png"
         alt="Executor Processor Bind" title="Executor Processor Bind"  />
</a>


### Configuration

| Name           | Type                                               | Description                                                                                                                                     | Required |
| -------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| *   | int64                                              | e                                                                         | No       |
| * | boolean                                            | g      | No       |




## Task

`Task` which corresponds to a set of commands given by the user that will be executed on the machine to generate task instances.

<a href="">
    <img src="./doc/delicate_logo.png"
         alt="Task" title="Task"  />
</a>


### Configuration

| Name           | Type                                               | Description                                                                                                                                     | Required |
| -------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| *   | int64                                              |e                                                                     | No       |
| * | boolean                                            | g       | No       |


