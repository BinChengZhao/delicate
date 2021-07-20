import api from 'api'
import { message } from 'antd'

const { groupList, groupCreate, groupUpdate, groupDelete } = api

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
      const data = yield call(groupCreate, payload)
      if (!data.code) {
        yield put({ type: 'hideGroupModal' })
      }
    },

    *update({ payload }, { select, call, put }) {
      const data = yield call(groupUpdate, payload)
      if (!data.code) {
        yield put({ type: 'hideGroupModal' })
      }
    },

    *delete({ payload }, { call, put }) {
      const data = yield call(groupDelete, payload)
      if (!data.code) {
        message.success('删除成功')
      }
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
