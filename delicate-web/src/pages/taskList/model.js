import api from '../../services'

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
    currentItem: {},
    modalVisible: false,
    modalType: 'create',
    queryWhere: {}
  },

  subscriptions: {
    setup({ dispatch, history }) {
      history.listen((location) => {})
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

    *delete({ payload }, { call, put, select }) {
      console.log(payload)
      // const data = yield call(taskDelete, payload)
      // if (!data.code) {
      //   message.success('删除成功')
      // }
    },

    *onTaskAdvance({ payload }, { call, put }) {
      const data = yield call(taskAdvance, payload)
      if (!data.code) {
        message.success('执行成功')
      }
    },

    *onTaskRun({ payload }, { call, put }) {
      const data = yield call(taskRun, payload)
      if (!data.code) {
        message.success('启动成功')
      }
    }
  },

  *create({ payload }, { call, put }) {
    console.log(payload)
    // const data = yield call(taskCreate, payload)
    // if (!data.code) {
    //   yield put({ type: 'hideModal' })
    // } else {
    //   throw data
    // }
  },

  *update({ payload }, { select, call, put }) {
    const id = yield select(({ taskList }) => taskList.currentItem.id)
    const newUser = { ...payload, id }
    const data = yield call(taskUpdate, newUser)
    if (!data.code) {
      yield put({ type: 'hideModal' })
    } else {
      throw data
    }
  },

  reducers: {
    showModal(state, { payload }) {
      return { ...state, ...payload, modalVisible: true }
    },
    hideModal(state) {
      return { ...state, modalVisible: false }
    },
    updateState(state, { payload }) {
      return { ...state, ...payload }
    }
  }
}
