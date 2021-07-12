import api from 'api'
import { message } from 'antd'

const { executorList, executorCreate, executorUpdate, executorDelete, executorActivate } = api

export default {
  namespace: 'executorModel',

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
      const data = yield call(executorList, payload)
      if (data.success) {
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
      const data = yield call(executorCreate, payload)
      if (data.code === 0) {
        yield put({ type: 'hideExecutorModal' })
      } else {
        throw data
      }
    },

    *update({ payload }, { select, call, put }) {
      const data = yield call(executorUpdate, payload)
      if (data.code === 0) {
        yield put({ type: 'hideExecutorModal' })
      } else {
        throw data
      }
    },

    *delete({ payload }, { call, put }) {
      const data = yield call(executorDelete, payload)
      if (data.code === 0) {
        message.success('删除成功')
      }
    },

    *activation({ payload }, { call, put }) {
      const data = yield call(executorActivate, payload)
      if (data.code === 0) {
        message.success('激活成功')
      }
    }
  },

  reducers: {
    showExecutorModal(state, { payload }) {
      return { ...state, ...payload, modalVisible: true }
    },
    hideExecutorModal(state) {
      return { ...state, modalVisible: false }
    },
    updateState(state, { payload }) {
      return { ...state, ...payload }
    }
  }
}
