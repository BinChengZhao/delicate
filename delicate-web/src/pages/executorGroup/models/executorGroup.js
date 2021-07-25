import api from 'api'
import { message } from 'antd'

const {
  groupList,
  groupCreate,
  groupUpdate,
  groupDelete,
  groupBindList,
  groupBindExecutor,
  groupUsedExecutor,
  groupDeleteExecutor,
  groupUpdateExecutor
} = api

export default {
  namespace: 'executorGroupModel',

  state: {
    queryWhere: {},
    currentItem: {},
    dataSource: [],
    pagination: {
      showSizeChanger: true,
      showQuickJumper: true,
      current: 1,
      total: 0,
      pageSize: 10
    },
    modalVisible: false,
    modalType: 'create'
  },

  subscriptions: {
    setup({ dispatch, history }) {
      history.listen((location) => {
        // dispatch({ type: 'query', payload: {} })
      })
    }
  },

  effects: {
    *query({ payload }, { call, put }) {
      const data = yield call(groupList, payload)
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

    *create({ payload }, { call, put }) {
      return yield call(groupCreate, payload)
    },

    *update({ payload }, { select, call, put }) {
      const data = yield call(groupUpdate, payload)
      if (!data.code) yield put({ type: 'hideGroupModal' })
    },

    *delete({ payload }, { call, put }) {
      const data = yield call(groupDelete, payload)
      if (!data.code) message.success('删除成功')
    },

    *groupUsedExecutor({ payload }, { call, put }) {
      const data = yield call(groupUsedExecutor, payload)
      return !data.code ? data.data : { inner: { name: '', description: '' }, bindings: [] }
    },
    *groupBindList({ payload }, { call, put }) {
      const data = yield call(groupBindList)
      return !data.code ? data.data : []
    },

    *onGroupBindExecutor({ payload }, { call, put }) {
      const data = yield call(groupBindExecutor, payload)
      if (data.code) message.success('执行器绑定异常')
      yield put({ type: 'hideGroupModal' })
    },

    *onGroupUpdateExecutor({ payload }, { call, put }) {
      const data = yield call(groupUpdateExecutor, payload)
      if (data.code) message.success('执行器绑定异常')
    },

    *groupDeleteExecutor({ payload }, { call, put }) {
      const data = yield call(groupDeleteExecutor, payload)
      if (!data.code) message.warning('解绑成功')
    }
  },

  reducers: {
    showGroupModal(state, { payload }) {
      return { ...state, ...payload, modalVisible: true }
    },
    hideGroupModal(state) {
      return { ...state, modalVisible: false }
    },

    updateState(state, { payload }) {
      return { ...state, ...payload }
    }
  }
}
