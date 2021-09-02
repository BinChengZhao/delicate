import api from '../../services'

const { loginLogList } = api

export default {
  namespace: 'loginLogs',

  state: {
    queryWhere: {}
  },

  effects: {
    *getLoginLogs({ payload }, { call, put }) {
      yield put({ type: 'saveQueryWhere', payload })
      try {
        return yield call(loginLogList, payload)
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
