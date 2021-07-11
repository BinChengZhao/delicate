import modelExtend from 'dva-model-extend'
import api from '../../services'
import { model } from '../../utils/model'

const { pathToRegexp } = require('path-to-regexp')

const { dashboard } = api

export default modelExtend(model, {
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
          payload: {
            taskStatusEChart: data.data
          }
        })
      }
    }
  }
})
