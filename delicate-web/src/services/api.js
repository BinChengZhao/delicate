export default {
  queryRouteList: '/v1/routes',

  queryUserInfo: '/v1/user',
  logoutUser: '/v1/user/logout',
  loginUser: 'POST /v1/user/login',

  queryUser: '/v1/user/:id',
  queryUserList: '/v1/users',
  updateUser: 'Patch /v1/user/:id',
  createUser: 'POST /v1/user',
  removeUser: 'DELETE /v1/user/:id',
  removeUserList: 'POST /v1/users/delete',

  queryPostList: '/v1/posts',

  queryDashboard: '/v1/dashboard',
  queryWeather: '/weather/now.json',

  // 任务相关
  queryTaskList: 'POST /task/list'
}
