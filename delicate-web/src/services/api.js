export default {
  queryRouteList: '/routes',

  logoutUser: 'POST /user/logout',
  loginUser: 'POST /user/login',

  queryDashboard: '/dashboard',
  queryWeather: '/weather/now.json',

  // common
  checkUser: 'POST /user/check',
  dashboard: '/tasks_state/one_day',

  // 任务相关
  queryTaskList: 'POST /task/list',
  taskAdvance: 'POST /task/advance',
  taskCreate: 'POST /task/create',
  taskUpdate: 'POST /task/update',
  taskDelete: 'POST /task/delete',
  taskRun: 'POST /task/run',
  taskSuspend: 'POST /task/suspend',
  taskBindList: '/binding/list',

  // 任务日志
  taskLogList: 'POST /task_log/list',
  taskLogDetail: 'POST /task_log/detail',
  taskKill: 'POST /task_instance/kill',

  // 执行调度器
  executorList: 'POST /executor_processor/list',
  executorCreate: 'POST /executor_processor/create',
  executorUpdate: 'POST /executor_processor/update',
  executorDelete: 'POST /executor_processor/delete',
  executorActivate: 'POST /executor_processor/activate',

  // 执行组
  groupList: 'POST /executor_group/list',
  groupCreate: 'POST /executor_group/create',
  groupUpdate: 'POST /executor_group/update',
  groupDelete: 'POST /executor_group/delete',
  groupBindList: '/executor/list',
  groupBindExecutor: 'POST /executor_processor_bind/create',
  groupUpdateExecutor: 'POST /executor_processor_bind/update',
  groupUsedExecutor: 'POST /executor_group/detail',
  groupDeleteExecutor: 'POST /executor_processor_bind/delete',

  // 用户相关
  queryUserList: 'POST /user/list',
  createUser: 'POST /user/create',
  updateUser: 'POST /user/update',
  deleteUser: 'POST /user/delete'
}
