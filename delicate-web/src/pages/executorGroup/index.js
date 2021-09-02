import React, { PureComponent } from 'react'
import { connect } from 'dva'
import { Page } from '../../components'
import PropTypes from 'prop-types'
import ExecutorGroupModal from './components/Modal'
import ExecutorGroupList from './components/List'
import ExecutorGroupFilter from './components/Filter'
import { t } from '@lingui/macro'

export const NAMESPACE = 'executorGroupModel'

@connect(({ executorGroupModel, loading }) => ({ executorGroupModel, loading }))
class ExecutorGroup extends PureComponent {
  formRef = React.createRef()

  handleRefresh = (newQuery) => {
    const { executorGroupModel, dispatch } = this.props
    const queryWhere = executorGroupModel.queryWhere
    const payload = { ...queryWhere, ...newQuery }
    dispatch({ type: `${NAMESPACE}/query`, payload: payload })
  }

  get filterProps() {
    const { dispatch } = this.props

    return {
      openModal: () => {
        dispatch({
          type: `${NAMESPACE}/showGroupModal`,
          payload: { modalType: 'create', currentItem: {} }
        })
      },
      query: (payload) => {
        dispatch({ type: `${NAMESPACE}/query`, payload: payload })
      }
    }
  }

  get modalProps() {
    const { dispatch, executorGroupModel, loading } = this.props
    let { currentItem, modalVisible, modalType } = executorGroupModel

    let item = {}
    let title = ''
    switch (modalType) {
      case 'create':
        title = t`Create`
        break
      case 'update':
        title = t`Update`
        item = item = { ...currentItem, tag: currentItem.tag.split(',').filter((e) => e !== '') }
        break
    }
    modalType = modalType === 'copy' ? 'create' : modalType
    return {
      modalType: modalType,
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
      onOk: (data) => dispatch({ type: `${NAMESPACE}/${modalType}`, payload: data }),
      onCancel: () => dispatch({ type: `${NAMESPACE}/hideGroupModal` }),
      getGroupBindList: () => dispatch({ type: `${NAMESPACE}/groupBindList` }),
      onGroupBindExecutor: (data) =>
        dispatch({ type: `${NAMESPACE}/onGroupBindExecutor`, payload: data }).then(() => this.handleRefresh()),
      groupUsedExecutor: (data) => dispatch({ type: `${NAMESPACE}/groupUsedExecutor`, payload: data }),
      onRefresh: (newQuery) => this.handleRefresh(newQuery)
    }
  }

  get listProps() {
    const { dispatch, executorGroupModel, loading } = this.props
    const { dataSource, pagination } = executorGroupModel
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
          payload: { executor_group_id: id }
        }).then(() => {
          this.handleRefresh()
        })
      },
      onEditItem(item) {
        dispatch({
          type: `${NAMESPACE}/showGroupModal`,
          payload: { modalType: 'update', currentItem: item }
        })
      },
      onCopy(item) {
        console.log(item)
        dispatch({
          type: `${NAMESPACE}/showGroupModal`,
          payload: { modalType: 'copy', currentItem: item }
        })
      }
    }
  }

  render() {
    return (
      <Page inner>
        <ExecutorGroupFilter {...this.filterProps} />
        <ExecutorGroupList {...this.listProps} />
        <ExecutorGroupModal {...this.modalProps} />
      </Page>
    )
  }
}

ExecutorGroup.propTypes = {
  executorGroupModel: PropTypes.object
}
export default ExecutorGroup
