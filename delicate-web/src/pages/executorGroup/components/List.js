import { t, Trans } from '@lingui/macro'
import { Button, message, Popconfirm, Space, Table, Tooltip } from 'antd'
import React, { PureComponent } from 'react'
import styles from '../../taskList/components/List.less'
import PropTypes from 'prop-types'
import { Link } from 'umi'

class ExecutorGroupList extends PureComponent {
  confirm(id) {
    const { onDeleteItem } = this.props
    onDeleteItem(id)
  }

  cancel() {
    message.info(t`Cancel Delete`)
  }

  render() {
    const { onEditItem, onDeleteItem, onCopy, ...tableProps } = this.props

    const columns = [
      {
        title: <Trans>Sn</Trans>,
        dataIndex: 'id',
        key: 'id',
        fixed: 'left'
      },
      {
        title: t`Group Name`,
        dataIndex: 'name',
        key: 'name',
        fixed: 'left',
        render: (text, row) => {
          return (
            <Tooltip title={t`Description` + ':' + row.description}>
              <a>{text}</a>
            </Tooltip>
          )
        }
      },
      {
        title: <Trans>Tag</Trans>,
        dataIndex: 'tag',
        key: 'tag'
      },
      {
        title: <Trans>Operation</Trans>,
        key: 'operation',
        render: (text, row) => {
          return (
            <Space split={'|'}>
              <a type={'link'} onClick={() => onEditItem(row)}>
                {t`Update`}
              </a>
              <Link to={{ pathname: `executorGroup/${row.id}` }}>{t`Group Detail`}</Link>
              <Popconfirm
                title={t`Are you sure you want to delete the execution group ${row.name}?`}
                onConfirm={() => this.confirm(row.id)}
                onCancel={() => this.cancel()}
                okText="Yes"
                cancelText="No"
              >
                <a type={'link'} style={{ color: 'red' }}>
                  {t`Delete`}
                </a>
              </Popconfirm>
            </Space>
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

ExecutorGroupList.propTypes = {
  onEditItem: PropTypes.func,
  onDeleteItem: PropTypes.func,
  onCopy: PropTypes.func
}

export default ExecutorGroupList
