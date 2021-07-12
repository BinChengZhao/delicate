import { t, Trans } from '@lingui/macro'
import { Dropdown, Menu, Space, Table, Tooltip, Popconfirm, message } from 'antd'
import { DownOutlined } from '@ant-design/icons'
import React, { PureComponent } from 'react'
import styles from '../../taskList/components/List.less'
import PropTypes from 'prop-types'

class ExecutorList extends PureComponent {
  confirm(id) {
    const { onDeleteItem } = this.props
    onDeleteItem(id)
  }

  cancel() {
    message.info('取消删除')
  }

  render() {
    const { onEditItem, onDeleteItem, onActivation, onCopy, ...tableProps } = this.props

    const MENU_ITEM_MAP = [
      { title: '复制节点', method: (r) => onCopy(r) },
      { title: '激活节点', method: (r) => onActivation(r.id) }
    ]

    const columns = [
      {
        title: <Trans>Sn</Trans>,
        dataIndex: 'id',
        key: 'id',
        fixed: 'left'
      },
      {
        title: '节点名称',
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
        title: '状态',
        dataIndex: 'status',
        key: 'status'
      },
      {
        title: 'Host',
        dataIndex: 'host',
        key: 'host'
      },
      {
        title: '端口号',
        dataIndex: 'port',
        key: 'port'
      },
      {
        title: '机器节点id',
        dataIndex: 'machine_id',
        key: 'machine_id'
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
                编辑
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
                      title={`去定要删除执行节点【${row.name}】吗？`}
                      onConfirm={() => this.confirm(row.id)}
                      onCancel={() => this.cancel()}
                      okText="Yes"
                      cancelText="No"
                    >
                      <Menu.Item danger>删除节点</Menu.Item>
                    </Popconfirm>
                  </Menu>
                }
              >
                <a className="ant-dropdown-link" onClick={(e) => e.preventDefault()}>
                  更多 <DownOutlined />
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
        pagination={{
          ...tableProps.pagination,
          showTotal: (total) => t`Total ${total} Items`
        }}
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
