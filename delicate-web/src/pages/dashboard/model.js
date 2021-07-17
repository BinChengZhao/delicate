import api from '../../services'

const { pathToRegexp } = require('path-to-regexp')

const { dashboard } = api

export default {
  namespace: 'dashboard',
  state: {
    taskStatusEChart: {}
  },
  subscriptions: {
    setup({ dispatch, history }) {
      history.listen(({ pathname }) => {
        if (pathToRegexp('/dashboard').exec(pathname) || pathToRegexp('/').exec(pathname)) {
          dispatch({ type: 'query' })
        }
      })
    }
  },
  effects: {
    *query({ payload }, { call, put }) {
      const data = yield call(dashboard)
      if (!data.code) {
        yield put({
          type: 'updateState',
          payload: { taskStatusEChart: data.data }
        })
      }
    }
  },

  reducers: {
    updateState(state, { payload }) {
      return { ...state, ...payload }
    }
  }
}
