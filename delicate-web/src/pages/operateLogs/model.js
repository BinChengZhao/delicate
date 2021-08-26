import api from '../../services'

const { optionLogList, optionLogDetail, loginLogList } = api

export default {
  namespace: 'logs',

  state: {
    queryWhere: {}
  },

  effects: {
    *getOperateLogs({ payload }, { call, put }) {
      yield put({ type: 'saveQueryWhere', payload })
      try {
        return yield call(optionLogList, payload)
      } catch (e) {
        console.log(e)
      }
    },
    *getOperateLogDetail({ payload }, { call }) {
      try {
        return yield call(optionLogDetail, payload)
      } catch (e) {
        console.log(e)
      }
    }
  },

  reducers: {
    saveQueryWhere(state, { payload }) {
      return { ...state, queryWhere: payload }
    }
  }
}
