const database = [
  {
    id: '1',
    icon: 'dashboard',
    name: 'Dashboard',
    zh: {
      name: '仪表盘'
    },
    route: '/dashboard'
  },
  {
    id: '2',
    breadcrumbParentId: '1',
    name: 'Task List',
    zh: { name: '任务列表' },
    icon: 'task-list',
    route: '/taskList'
  },
  {
    id: '21',
    menuParentId: '-1',
    breadcrumbParentId: '2',
    name: 'Task Log',
    zh: { name: '任务日志' },
    route: '/taskList/:id'
  },
  {
    id: '3',
    breadcrumbParentId: '1',
    name: 'Executor Resources',
    zh: { name: '执行资源' },
    icon: 'executor-o'
  },
  {
    id: '31',
    menuParentId: '3',
    breadcrumbParentId: '3',
    name: 'Execution Node',
    zh: { name: '执行节点' },
    route: '/executor'
  },
  {
    id: '32',
    menuParentId: '3',
    breadcrumbParentId: '3',
    name: 'Execution Group',
    zh: { name: '执行组' },
    route: '/executorGroup'
  },
  {
    id: '321',
    menuParentId: '-1',
    breadcrumbParentId: '32',
    name: 'Execution Group',
    zh: { name: '组详情' },
    route: '/executorGroup/:id'
  },
  {
    id: '4',
    breadcrumbParentId: '1',
    name: 'Logs',
    zh: { name: '日志相关' },
    icon: 'readOut'
  },
  {
    id: '41',
    menuParentId: '4',
    breadcrumbParentId: '4',
    name: 'Operate Logs',
    zh: { name: '操作日志' },
    route: '/operateLogs'
  },
  {
    id: '411',
    menuParentId: '-1',
    breadcrumbParentId: '41',
    name: 'Operate Log Detail',
    zh: { name: '操作日志详情' },
    route: '/operateLogs/:id'
  },
  {
    id: '9',
    breadcrumbParentId: '1',
    name: 'user',
    zh: { name: '用户管理' },
    icon: 'user',
    route: '/user'
  },
  {
    id: '91',
    menuParentId: '-1',
    breadcrumbParentId: '9',
    name: 'User Detail',
    zh: {
      name: '用户详情'
    },
    route: '/user/:id'
  }
]

export default database
