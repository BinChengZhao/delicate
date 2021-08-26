// 数据工具类

import moment from 'moment'

export const isEmpty = (val) => {
  // null or undefined
  if (val == null) return true

  if (typeof val === 'boolean') return false

  if (typeof val === 'number') return !val

  if (val instanceof Error) return val.message === ''

  switch (Object.prototype.toString.call(val)) {
    // String or Array
    case '[object String]':
    case '[object Array]':
      return !val.length

    // Map or Set or File
    case '[object File]':
    case '[object Map]':
    case '[object Set]': {
      return !val.size
    }
    // Plain Object
    case '[object Object]': {
      return !Object.keys(val).length
    }
  }

  return false
}

/**
 * DatePiker组件获取时间范围
 * @param range
 * @returns {{start_time: null|number, end_time: null|number}}
 */
export const getDatePikerRange = (range) => {
  const timeRange = { start_time: null, end_time: null }
  if (range) {
    timeRange.start_time = parseInt(moment(range[0]._d).valueOf() / 1000)
    timeRange.end_time = parseInt(moment(range[1]._d).valueOf() / 1000)
  }
  return timeRange
}
