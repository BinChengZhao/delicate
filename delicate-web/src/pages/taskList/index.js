import React, { PureComponent } from 'react'
import PropTypes from 'prop-types'
import { connect } from 'umi'
import { t } from '@lingui/macro'
import { Page } from '../../components'
import List from './components/List'
import Filter from './components/Filter'
import TaskModal from './components/Modal'

@connect(({ taskList, loading }) => ({ taskList, loading }))
class Task extends PureComponent {
  get modalProps() {
    const { dispatch, taskList, loading } = this.props
    const { currentItem, modalVisible, modalType } = taskList

    return {
      item: modalType === 'create' ? {} : currentItem,
      visible: modalVisible,
      destroyOnClose: true,
      maskClosable: false,
      cancelText: '取消',
      okText: '保存',
      confirmLoading: loading.effects[`taskList/${modalType}`],
      title: `${modalType === 'create' ? t`Create Task` : t`Update User`}`,
      centered: true,
      width: 800,
      onOk: (data) => {
        dispatch({
          type: `taskList/${modalType}`,
          payload: data
        }).then(() => {
          this.handleRefresh()
        })
      },
      onCancel() {
        dispatch({
          type: 'taskList/hideModal'
        })
      }
    }
  }

  get listProps() {
    const { dispatch, taskList, loading } = this.props
    const { dataSource, pagination } = taskList

    return {
      dataSource,
      loading: loading.effects['taskList/query'],
      pagination,
      onChange: (page) => {
        this.handleRefresh({
          page: page.current,
          pageSize: page.pageSize
        })
      },
      onDeleteItem: (id) => {
        dispatch({
          type: 'taskList/delete',
          payload: id
        }).then(() => {
          this.handleRefresh({
            page: list.length === 1 && pagination.current > 1 ? pagination.current - 1 : pagination.current
          })
        })
      },
      onEditItem(item) {
        dispatch({
          type: 'taskList/showModal',
          payload: {
            modalType: 'update',
            currentItem: item
          }
        })
      }
    }
  }

  get filterProps() {
    const { location, dispatch } = this.props
    const { query } = location

    return {
      filter: { ...query },
      onFilterChange: (value) => dispatch({ type: 'taskList/query', payload: value }),
      onAdd() {
        dispatch({
          type: 'taskList/showModal',
          payload: {
            modalType: 'create'
          }
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
  taskList: PropTypes.object,
  location: PropTypes.object,
  dispatch: PropTypes.func,
  loading: PropTypes.object
}

export default Task
