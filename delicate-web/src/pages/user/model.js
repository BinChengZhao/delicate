import api from 'api'
import { message } from 'antd'

const { queryUserList, createUser, updateUser, deleteUser, updatePassword } = api

export default {
  namespace: 'user',

  state: {
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
      const data = yield call(queryUserList, payload)
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
      const data = yield call(deleteUser, payload)
      if (!data.code) message.success('删除成功')
    },

    *create({ payload }, { call, put }) {
      const data = yield call(createUser, payload)
      if (!data.code) {
        yield put({ type: 'hideModal' })
      }
    },

    *update({ payload }, { select, call, put }) {
      const data = yield call(updateUser, payload)
      if (!data.code) {
        yield put({ type: 'hideModal' })
      }
    },

    *updatePassword({ payload }, { select, call, put }) {
      try {
        return yield call(updatePassword, payload)
      } catch (e) {
        console.error(e)
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

    updateState(state, { payload }) {
      return { ...state, ...payload }
    }
  }
}
