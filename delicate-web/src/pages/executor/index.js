import React, { PureComponent } from 'react'
import { connect } from 'dva'
import { Page } from '../../components'
import PropTypes from 'prop-types'
import ExecutorModal from './components/Modal'
import ExecutorList from './components/List'
import ExecutorFilter from './components/Filter'

const NAMESPACE = 'executorModel'

@connect(({ executorModel, loading }) => ({ executorModel, loading }))
class Executor extends PureComponent {
  formRef = React.createRef()

  handleRefresh = (newQuery) => {
    const { executorModel, dispatch } = this.props
    const queryWhere = executorModel.queryWhere
    const payload = { ...queryWhere, ...newQuery }
    dispatch({ type: `${NAMESPACE}/query`, payload: payload })
  }

  get filterProps() {
    const { dispatch } = this.props

    return {
      openModal: () => {
        dispatch({
          type: `${NAMESPACE}/showExecutorModal`,
          payload: { modalType: 'create', currentItem: {} }
        })
      },
      query: (payload) => {
        dispatch({ type: `${NAMESPACE}/query`, payload: payload })
      }
    }
  }

  get modalProps() {
    const { dispatch, executorModel, loading } = this.props
    let { currentItem, modalVisible, modalType } = executorModel

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
        dispatch({ type: `${NAMESPACE}/${modalType}`, payload: data }).then(() => this.handleRefresh())
      },
      onCancel() {
        dispatch({
          type: `${NAMESPACE}/hideExecutorModal`
        })
      }
    }
  }

  get listProps() {
    const { dispatch, executorModel, loading } = this.props
    const { dataSource, pagination } = executorModel
    return {
      dataSource,
      loading: loading.effects['executorModel/query'],
      pagination,
      onChange: (page) => {
        this.handleRefresh({
          page: page.current,
          per_page: page.pageSize
        })
      },
      onDeleteItem: (id) => {
        dispatch({
          type: 'executorModel/delete',
          payload: { executor_processor_id: id }
        }).then(() => {
          this.handleRefresh()
        })
      },
      onEditItem(item) {
        dispatch({
          type: 'executorModel/showExecutorModal',
          payload: { modalType: 'update', currentItem: item }
        })
      },
      onCopy(item) {
        console.log(item)
        dispatch({
          type: 'executorModel/showExecutorModal',
          payload: { modalType: 'copy', currentItem: item }
        })
      },
      onActivation(id) {
        dispatch({
          type: 'executorModel/activation',
          payload: { executor_processor_id: id }
        }).then(() => {
          this.handleRefresh()
        })
      }
    }
  }

  render() {
    return (
      <Page inner>
        <ExecutorFilter {...this.filterProps} />
        <ExecutorList {...this.listProps} />
        <ExecutorModal {...this.modalProps} />
      </Page>
    )
  }
}

Executor.propTypes = {
  executorModel: PropTypes.object
}
export default Executor
