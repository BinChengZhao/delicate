import React, { PureComponent } from 'react'
import PropTypes from 'prop-types'
import { connect } from 'umi'
import { t } from '@lingui/macro'
import { Page } from 'components'
import List from './components/List'
import Filter from './components/Filter'
import Modal from './components/Modal'

@connect(({ user, loading }) => ({ user, loading }))
class User extends PureComponent {
  handleRefresh = (newQuery) => {
    const { user, dispatch } = this.props
    const queryWhere = user.queryWhere
    const payload = { ...queryWhere, ...newQuery }
    dispatch({ type: `user/query`, payload: payload })
  }

  get modalProps() {
    const { dispatch, user, loading } = this.props
    const { currentItem, modalVisible, modalType } = user

    return {
      item: modalType === 'create' ? {} : currentItem,
      visible: modalVisible,
      destroyOnClose: true,
      maskClosable: false,
      confirmLoading: loading.effects[`user/${modalType}`],
      title: `${modalType === 'create' ? t`Create Task` : t`Update Task`}`,
      centered: true,
      onOk: (data) => dispatch({ type: `user/${modalType}`, payload: data }).then(() => this.handleRefresh()),
      onCancel: () => dispatch({ type: 'user/hideModal' })
    }
  }

  get listProps() {
    const { dispatch, user, loading } = this.props
    const { dataSource, pagination } = user

    return {
      dataSource,
      loading: loading.effects['user/query'],
      pagination,
      onChange: (page) => {
        this.handleRefresh({
          page: page.current,
          per_page: page.pageSize
        })
      },
      onDeleteItem: (id) =>
        dispatch({ type: 'user/delete', payload: { user_id: id } }).then(() => this.handleRefresh()),
      onEditItem: (item) => dispatch({ type: 'user/showModal', payload: { modalType: 'update', currentItem: item } })
    }
  }

  get filterProps() {
    const { location, dispatch } = this.props
    const { query } = location

    return {
      filter: { ...query },
      onFilterChange: (value) => dispatch({ type: 'user/query', payload: value }),
      onAdd: () => dispatch({ type: 'user/showModal', payload: { modalType: 'create' } })
    }
  }

  render() {
    return (
      <Page inner>
        <Filter {...this.filterProps} />
        <List {...this.listProps} />
        <Modal {...this.modalProps} />
      </Page>
    )
  }
}

User.propTypes = {
  user: PropTypes.object,
  location: PropTypes.object,
  dispatch: PropTypes.func,
  loading: PropTypes.object
}

export default User
