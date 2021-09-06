import React, { PureComponent } from 'react'
import { connect } from 'dva'
import { Page } from '../../components'
import PropTypes from 'prop-types'
import _ from 'lodash'

import Filter from './components/Filter'
import List from './components/List'
import TaskModal from './components/Modal'
import { t } from '@lingui/macro'

const NAMESPACE = 'taskModel'

@connect(({ taskModel, loading }) => ({ taskModel, loading }))
class Task extends PureComponent {
  handleRefresh = (newQuery) => {
    const { taskModel, dispatch } = this.props
    const queryWhere = taskModel.queryWhere
    const payload = { ...queryWhere, ...newQuery }
    dispatch({ type: `${NAMESPACE}/query`, payload: payload })
  }

  get filterProps() {
    const { dispatch } = this.props

    return {
      openModal: () => {
        dispatch({
          type: `${NAMESPACE}/showModal`,
          payload: { modalType: 'create', currentItem: {} }
        })
      },
      query: (payload) => {
        dispatch({ type: `${NAMESPACE}/query`, payload: payload })
      }
    }
  }

  get modalProps() {
    const { dispatch, taskModel, loading } = this.props
    let { currentItem, modalVisible, modalType } = taskModel

    let item = {}
    let title = ''
    switch (modalType) {
      case 'create':
        title = t`Create`
        break
      case 'copy':
        title = t`Copy`
        break
      case 'update':
        title = t`Update`
        item = currentItem
        break
    }
    modalType = modalType === 'copy' ? 'create' : modalType
    return {
      item: item,
      visible: modalVisible,
      destroyOnClose: true,
      maskClosable: false,
      cancelText: t`Cancel`,
      okText: t`Save`,
      confirmLoading: loading.effects[`${NAMESPACE}/${modalType}`],
      title: title,
      centered: true,
      width: 800,
      onOk: (data) => dispatch({ type: `${NAMESPACE}/${modalType}`, payload: data }).then(() => this.handleRefresh()),
      onCancel: () => dispatch({ type: `${NAMESPACE}/hideModal` }),
      getBindList: () => dispatch({ type: `${NAMESPACE}/taskBindList`, payload: {} })
    }
  }

  get listProps() {
    const { dispatch, taskModel, loading } = this.props
    const { dataSource, pagination } = taskModel
    return {
      dataSource,
      loading: loading.effects[`${NAMESPACE}/query`],
      pagination,
      onChange: (page) => {
        this.handleRefresh({
          page: page.current,
          per_page: page.pageSize
        })
      },
      onDeleteItem: (id) => {
        dispatch({
          type: `${NAMESPACE}/delete`,
          payload: { task_id: id }
        }).then(() => this.handleRefresh())
      },
      onEditItem: (item) => {
        const frequency = JSON.parse(item.frequency)
        const status = item.status ? 2 : 1
        const tag = _.isEmpty(item.tag) ? [] : item.tag.split(',')
        dispatch({
          type: `${NAMESPACE}/showModal`,
          payload: { modalType: 'update', currentItem: { ...item, frequency, status, tag } }
        })
      },
      onTaskRun: (id) => {
        dispatch({
          type: `${NAMESPACE}/onTaskRun`,
          payload: { task_id: id }
        }).then(() => this.handleRefresh())
      },
      onTaskSuspend: (id) => {
        dispatch({
          type: `${NAMESPACE}/onTaskSuspend`,
          payload: { task_id: id }
        }).then(() => this.handleRefresh())
      },
      onTaskAdvance: (id) => {
        dispatch({
          type: `${NAMESPACE}/onTaskAdvance`,
          payload: { task_id: id }
        }).then(() => this.handleRefresh())
      }
    }
  }

  render() {
    return (
      <Page inner>
        <Filter {...this.filterProps} />
        <List {...this.listProps} />
        <TaskModal {...this.modalProps} />
      </Page>
    )
  }
}

Task.propTypes = {
  taskModel: PropTypes.object
}
export default Task
