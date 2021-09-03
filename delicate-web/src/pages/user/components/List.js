import React, { PureComponent } from 'react'
import PropTypes from 'prop-types'
import { Modal, Table } from 'antd'
import { DropOption } from 'components'
import { t, Trans } from '@lingui/macro'
import styles from './List.less'

const { confirm } = Modal

class List extends PureComponent {
  handleMenuClick = (record, e) => {
    const { onDeleteItem, onEditItem } = this.props

    if (e.key === '1') {
      onEditItem(record)
    } else if (e.key === '2') {
      confirm({
        title: t`Are you sure delete this record?`,
        onOk() {
          onDeleteItem(record.id)
        }
      })
    }
  }

  render() {
    const { onDeleteItem, onEditItem, ...tableProps } = this.props

    const columns = [
      {
        title: '用户名',
        dataIndex: 'user_name',
        key: 'user_name'
      },
      {
        title: <Trans>NickName</Trans>,
        dataIndex: 'nick_name',
        key: 'nick_name'
      },
      {
        title: '手机号',
        dataIndex: 'mobile',
        key: 'mobile'
      },
      {
        title: <Trans>Email</Trans>,
        dataIndex: 'email',
        key: 'email'
      },
      {
        title: <Trans>CreateTime</Trans>,
        dataIndex: 'created_time',
        key: 'createTime'
      },
      {
        title: <Trans>Operation</Trans>,
        key: 'operation',
        fixed: 'right',
        width: '8%',
        render: (text, record) => {
          return (
            <DropOption
              onMenuClick={(e) => this.handleMenuClick(record, e)}
              menuOptions={[
                { key: '1', name: t`Update` },
                { key: '2', name: t`Delete` }
              ]}
            />
          )
        }
      }
    ]

    return (
      <Table
        {...tableProps}
        pagination={{ ...tableProps.pagination }}
        className={styles.table}
        columns={columns}
        simple
        rowKey={(record) => record.id}
      />
    )
  }
}

List.propTypes = {
  onDeleteItem: PropTypes.func,
  onEditItem: PropTypes.func,
  location: PropTypes.object
}

export default List
