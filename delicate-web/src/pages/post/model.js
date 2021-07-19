import modelExtend from 'dva-model-extend'
import api from 'api'
import { pageModel } from 'utils/model'
const { pathToRegexp } = require('path-to-regexp')

const { queryPostList } = api

export default modelExtend(pageModel, {
  namespace: 'post',

  subscriptions: {
    setup({ dispatch, history }) {
      history.listen((location) => {
        if (pathToRegexp('/post').exec(location.pathname)) {
          dispatch({
            type: 'query',
            payload: {
              status: 2,
              ...location.query
            }
          })
        }
      })
    }
  },

  effects: {
    *query({ payload }, { call, put }) {
      const data = yield call(queryPostList, payload)
      if (data.success) {
        yield put({
          type: 'querySuccess',
          payload: {
            list: data.data,
            pagination: {
              current: Number(payload.page) || 1,
              pageSize: Number(payload.pageSize) || 10,
              total: data.total
            }
          }
        })
      }
    }
  }
})
