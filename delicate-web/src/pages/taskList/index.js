import React, { PureComponent } from 'react'
import { connect } from 'dva'
import { Page } from '../../components'
import PropTypes from 'prop-types'

import Filter from './components/Filter'
import List from './components/List'
import TaskModal from './components/Modal'

const NAMESPACE = 'taskModel'

@connect(({ taskModel, loading }) => ({ taskModel, loading }))
class Task extends PureComponent {
  formRef = React.createRef()

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
        title = '创建执行器'
        break
      case 'copy':
        title = '复制执行器'
        item = { ...currentItem, id: null, tag: currentItem.tag.split(',') }
        break
      case 'update':
        title = '编辑执行器'
        item = item = { ...currentItem, tag: currentItem.tag.split(',') }
        break
    }
    modalType = modalType === 'copy' ? 'create' : modalType
    return {
      item: item,
      visible: modalVisible,
      destroyOnClose: true,
      maskClosable: false,
      cancelText: '取消',
      okText: '保存',
      confirmLoading: loading.effects[`${NAMESPACE}/${modalType}`],
      title: title,
      centered: true,
      width: 800,
      onOk: (data) => {
        console.log(`${NAMESPACE}/${modalType}`, data)
        dispatch({ type: `${NAMESPACE}/${modalType}`, payload: data })
      },
      onCancel() {
        dispatch({ type: `${NAMESPACE}/hideModal` })
      }
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
          payload: { executor_processor_id: id }
        }).then(() => {
          this.handleRefresh()
        })
      },
      onEditItem(item) {
        dispatch({
          type: `${NAMESPACE}/showModal`,
          payload: { modalType: 'update', currentItem: item }
        })
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
