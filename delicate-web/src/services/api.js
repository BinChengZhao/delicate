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
  groupDelete: 'POST /executor_group/delete'
}
