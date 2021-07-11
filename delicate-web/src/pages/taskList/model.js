import taskModel from 'dva-model-extend'
import api from '../../services'
import { pageModel } from '../../utils/model'

const { queryTaskList, createUser, removeUser, updateUser, removeUserList } = api

export default taskModel(pageModel, {
  namespace: 'taskList',

  state: {
    currentItem: {},
    modalVisible: false,
    modalType: 'create',
    selectedRowKeys: []
  },

  subscriptions: {
    setup({ dispatch, history }) {
      history.listen((location) => {})
    }
  },

  effects: {
    *query({ payload = {} }, { call, put }) {
      const data = yield call(queryTaskList, payload)
      if (data) {
        yield put({
          type: 'changeTaskList',
          payload: {
            dataSource: data.data.dataSource,
            pagination: data.data.pagination
          }
        })
      }
    },

    *delete({ payload }, { call, put, select }) {
      const data = yield call(removeUser, { id: payload })
      const { selectedRowKeys } = yield select((_) => _.user)
      if (data.success) {
        yield put({
          type: 'updateState',
          payload: {
            selectedRowKeys: selectedRowKeys.filter((_) => _ !== payload)
          }
        })
      } else {
        throw data
      }
    },

    *multiDelete({ payload }, { call, put }) {
      const data = yield call(removeUserList, payload)
      if (data.success) {
        yield put({ type: 'updateState', payload: { selectedRowKeys: [] } })
      } else {
        throw data
      }
    },

    *create({ payload }, { call, put }) {
      const data = yield call(createUser, payload)
      if (data.success) {
        yield put({ type: 'hideModal' })
      } else {
        throw data
      }
    },

    *update({ payload }, { select, call, put }) {
      const id = yield select(({ taskList }) => taskList.currentItem.id)
      const newUser = { ...payload, id }
      const data = yield call(updateUser, newUser)
      if (data.success) {
        yield put({ type: 'hideModal' })
      } else {
        throw data
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

    changeTaskList(state, { payload }) {
      return { ...state, ...payload }
    }
  }
})
