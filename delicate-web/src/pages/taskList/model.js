import api from '../../services'
import { message } from 'antd'
import { t } from '@lingui/macro'

const {
  queryTaskList,
  taskAdvance,
  taskCreate,
  taskUpdate,
  taskDelete,
  taskRun,
  taskSuspend,
  taskLogList,
  taskLogDetail,
  taskLogDelete,
  taskKill,
  taskBindList
} = api

export default {
  namespace: 'taskModel',

  state: {
    dataSource: [],
    pagination: {
      showSizeChanger: true,
      showQuickJumper: true,
      current: 1,
      total: 0,
      pageSize: 10
    },
    logSource: [],
    logPagination: {
      showSizeChanger: true,
      showQuickJumper: true,
      current: 1,
      total: 0,
      pageSize: 10
    },
    currentItem: {},
    currentLog: {},
    modalVisible: false,
    modalType: 'create',
    queryWhere: {},
    logQueryWhere: {}
  },

  subscriptions: {
    setup({ dispatch, history }) {
      history.listen((location) => {
        const pattern = /^\/taskList\/\d$/
        const str = location.pathname
        if (pattern.test(str) && location.state === undefined) history.push({ pathname: '/taskList' })
      })
    }
  },
  effects: {
    // 查询任务列表
    *query({ payload = {} }, { call, put }) {
      const data = yield call(queryTaskList, payload)
      if (!data.code) {
        yield put({
          type: 'updateState',
          payload: {
            dataSource: data.data.dataSource,
            pagination: data.data.pagination,
            queryWhere: payload
          }
        })
      }
    },
    // 创建任务
    *create({ payload }, { call, put }) {
      const data = yield call(taskCreate, payload)
      if (!data.code) yield put({ type: 'hideModal' })
    },
    // 修改任务
    *update({ payload }, { call, put }) {
      const data = yield call(taskUpdate, payload)
      if (!data.code) yield put({ type: 'hideModal' })
    },
    // 删除任务
    *delete({ payload }, { call, put, select }) {
      const data = yield call(taskDelete, payload)
      if (!data.code) message.success('删除成功')
    },
    // 查询任务日志列表
    *taskLogList({ payload = {} }, { call, put }) {
      const data = yield call(taskLogList, payload)
      if (!data.code) {
        yield put({
          type: 'updateState',
          payload: {
            logSource: data.data.dataSource,
            logPagination: data.data.pagination,
            logQueryWhere: payload
          }
        })
      }
    },
    // 任务日志详情
    *taskLogDetail({ payload }, { call, put }) {
      try {
        return yield call(taskLogDetail, payload)
      } catch (e) {
        console.error(e)
      }
    },
    // 删除任务日志
    *taskLogDelete({ payload }, { call, put }) {
      try {
        const data = yield call(taskLogDelete, payload)
        if (!data.code) message.success(t`Success`)
      } catch (e) {
        console.error(e)
      }
    },
    // 立即执行任务
    *onTaskAdvance({ payload }, { call, put }) {
      try {
        const data = yield call(taskAdvance, payload)
        if (!data.code) message.success(t`Success`)
      } catch (e) {
        console.log(e)
      }
    },
    // 启用任务
    *onTaskRun({ payload }, { call, put }) {
      const data = yield call(taskRun, payload)
      if (!data.code) message.success(t`Success`)
    },
    // 停用任务
    *onTaskSuspend({ payload }, { call, put }) {
      const data = yield call(taskSuspend, payload)
      if (!data.code) message.warning(t`Success`)
    },
    // 杀死任务
    *onTaskKill({ payload }, { call, put }) {
      const data = yield call(taskKill, payload)
      if (!data.code) message.warning(t`Success`)
      return data
    },
    *taskBindList({ payload }, { call, put }) {
      const data = yield call(taskBindList)
      return !data.code ? data.data : []
    }
  },

  reducers: {
    showModal(state, { payload }) {
      return { ...state, ...payload, modalVisible: true }
    },
    hideModal(state) {
      return { ...state, modalVisible: false }
    },
    showLogModal(state, { payload }) {
      return { ...state, ...payload, logModalVisible: true }
    },
    hideLogModal(state) {
      return { ...state, logModalVisible: false }
    },
    updateState(state, { payload }) {
      return { ...state, ...payload }
    }
  }
}
