import { t, Trans } from '@lingui/macro'
import { Dropdown, Menu, Space, Table, Tooltip, Popconfirm, message } from 'antd'
import { CheckCircleOutlined, DownOutlined, StopOutlined, WarningOutlined } from '@ant-design/icons'
import React, { PureComponent } from 'react'
import styles from '../../taskList/components/List.less'
import PropTypes from 'prop-types'

class ExecutorList extends PureComponent {
  confirm(id) {
    const { onDeleteItem } = this.props
    onDeleteItem(id)
  }

  cancel() {
    message.info(t`Cancel Delete`)
  }

  render() {
    const { onEditItem, onDeleteItem, onActivation, onCopy, ...tableProps } = this.props

    const MENU_ITEM_MAP = [
      { title: t`Copy Node`, method: (r) => onCopy(r) },
      { title: t`Activate Node`, method: (r) => onActivation(r.id) }
    ]

    const columns = [
      {
        title: t`Sn`,
        dataIndex: 'id',
        key: 'id',
        fixed: 'left'
      },
      {
        title: t`Node Name`,
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
        title: t`Status`,
        dataIndex: 'status',
        key: 'status',
        render: (text) => {
          switch (text) {
            case 1:
              return <StopOutlined style={{ color: 'red', fontSize: '18px' }} />
            case 2:
              return <CheckCircleOutlined style={{ color: 'green', fontSize: '18px' }} />
            case 3:
              return <WarningOutlined style={{ color: 'orange', fontSize: '18px' }} />
          }
        }
      },
      {
        title: t`Host`,
        dataIndex: 'host',
        key: 'host'
      },
      {
        title: t`Machine Id`,
        dataIndex: 'machine_id',
        key: 'machine_id'
      },
      {
        title: t`Tag`,
        dataIndex: 'tag',
        key: 'tag'
      },
      {
        title: t`Operation`,
        key: 'operation',
        render: (text, row) => {
          return (
            <Space split={'|'}>
              <a type={'link'} onClick={() => onEditItem(row)}>
                {t`Update`}
              </a>
              <Dropdown
                overlay={
                  <Menu>
                    {MENU_ITEM_MAP.map((item, index) => {
                      return (
                        <Menu.Item key={index} {...item} onClick={() => item.method(row)}>
                          {item.title}
                        </Menu.Item>
                      )
                    })}
                    <Popconfirm
                      title={t`Are you sure you want to delete the execution node ${row.name}?`}
                      onConfirm={() => this.confirm(row.id)}
                      onCancel={() => this.cancel()}
                      okText="Yes"
                      cancelText="No"
                    >
                      <Menu.Item danger>{t`Delete Node`}</Menu.Item>
                    </Popconfirm>
                  </Menu>
                }
              >
                <a className="ant-dropdown-link" onClick={(e) => e.preventDefault()}>
                  {t`More`} <DownOutlined />
                </a>
              </Dropdown>
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

ExecutorList.propTypes = {
  onEditItem: PropTypes.func,
  onDeleteItem: PropTypes.func,
  onCopy: PropTypes.func,
  onActivation: PropTypes.func
}

export default ExecutorList
