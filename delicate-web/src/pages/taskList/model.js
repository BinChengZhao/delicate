import api from '../../services'
import { message } from 'antd'
import { history } from 'umi'

const {
  queryTaskList,
  taskAdvance,
  taskCreate,
  taskUpdate,
  taskDelete,
  taskRun,
  taskSuspend,
  taskLogList,
  taskLogEvent,
  taskLogDetail,
  taskKill
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

    *taskLogDetail({ payload }, { call, put }) {
      return yield call(taskLogDetail, payload)
    },

    *delete({ payload }, { call, put, select }) {
      const data = yield call(taskDelete, payload)
      if (!data.code) {
        message.success('删除成功')
      }
    },

    *onTaskAdvance({ payload }, { call, put }) {
      const data = yield call(taskAdvance, payload)
      if (!data.code) {
        message.success('手动执行操作成功')
      }
    },

    *onTaskRun({ payload }, { call, put }) {
      const data = yield call(taskRun, payload)
      if (!data.code) {
        message.success('启动成功')
      }
    },

    *onTaskSuspend({ payload }, { call, put }) {
      const data = yield call(taskSuspend, payload)
      if (!data.code) {
        message.warning('任务已暂停')
      }
    },

    *onTaskKill({ payload }, { call, put }) {
      const data = yield call(taskKill, payload)
      if (!data.code) {
        message.warning('任务强行结束！')
      }
      return data
    },

    *create({ payload }, { call, put }) {
      const data = yield call(taskCreate, payload)
      if (!data.code) {
        yield put({ type: 'hideModal' })
      }
    },

    *update({ payload }, { call, put }) {
      const data = yield call(taskUpdate, payload)
      if (!data.code) {
        yield put({ type: 'hideModal' })
      }
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
