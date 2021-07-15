import React, { PureComponent } from 'react'
import { Menu, Table, Tooltip, Space, Dropdown } from 'antd'
import { t, Trans } from '@lingui/macro'
import styles from './List.less'
import { CheckCircleOutlined, DownOutlined, StopOutlined } from '@ant-design/icons'
import PropTypes from 'prop-types'
import { Link } from 'umi'

const STATUS_ENABLE = 1 // 启用

class List extends PureComponent {
  menu(row) {
    return (
      <Menu>
        <Menu.Item disabled={row.status === STATUS_ENABLE}>启用任务</Menu.Item>
        <Menu.Item disabled={row.status !== STATUS_ENABLE}>立即执行</Menu.Item>
        <Menu.Item>
          <Link
            to={{
              pathname: `taskList/${row.id}`,
              state: row
            }}
          >
            查看日志
          </Link>
        </Menu.Item>
        <Menu.Item>复制任务</Menu.Item>
        <Menu.Item danger>删除任务</Menu.Item>
      </Menu>
    )
  }

  render() {
    const { onEditItem, ...tableProps } = this.props
    const columns = [
      {
        title: <Trans>Sn</Trans>,
        dataIndex: 'id',
        key: 'id',
        width: 70,
        fixed: 'left'
      },
      {
        title: <Trans>Task Name</Trans>,
        dataIndex: 'name',
        key: 'name',
        width: 120,
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
        title: '是否启用',
        dataIndex: 'status',
        width: 100,
        key: 'status',
        render: (text) => {
          return text === STATUS_ENABLE ? (
            <CheckCircleOutlined style={{ color: 'green', fontSize: '18px' }} />
          ) : (
            <StopOutlined style={{ color: 'red', fontSize: '18px' }} />
          )
        }
      },
      {
        title: <Trans>Command</Trans>,
        dataIndex: 'command',
        width: 200,
        key: 'command',
        ellipsis: true,
        render: (text, row) => {
          return <Tooltip title={row.command}>{text}</Tooltip>
        }
      },
      {
        title: <Trans>Frequency</Trans>,
        dataIndex: 'frequency',
        width: 300,
        key: 'frequency',
        ellipsis: true,
        render: (text, row) => {
          return <Tooltip title={row.frequency}>{text} </Tooltip>
        }
      },
      {
        title: <Trans>Cron Expression</Trans>,
        dataIndex: 'cron_expression',
        width: 120,
        key: 'cron_expression'
      },
      {
        title: <Trans>Timeout</Trans>,
        dataIndex: 'timeout',
        key: 'timeout',
        width: 100
      },
      {
        title: <Trans>Retry Times</Trans>,
        dataIndex: 'retry_times',
        key: 'retry_times',
        width: 130
      },
      {
        title: <Trans>Retry Interval</Trans>,
        dataIndex: 'retry_interval',
        key: 'retry_interval',
        width: 130
      },
      {
        title: <Trans>Max Parallel Num</Trans>,
        dataIndex: 'maximum_parallel_runnable_num',
        key: 'maximum_parallel_runnable_num',
        width: 120
      },
      {
        title: <Trans>Task Id</Trans>,
        dataIndex: 'task_id',
        key: 'task_id',
        width: 100
      },
      {
        title: <Trans>Tag</Trans>,
        dataIndex: 'tag',
        key: 'tag'
      },
      {
        title: <Trans>Bind Id</Trans>,
        dataIndex: 'bind_id',
        key: 'bind_id',
        width: 100
      },
      {
        title: <Trans>Operation</Trans>,
        key: 'Operation',
        fixed: 'right',
        render: (text, row) => (
          <Space split={'|'}>
            <a type={'link'} onClick={() => onEditItem(row)}>
              编辑
            </a>
            <Dropdown overlay={this.menu(row)}>
              <a className="ant-dropdown-link" onClick={(e) => e.preventDefault()}>
                更多 <DownOutlined />
              </a>
            </Dropdown>
          </Space>
        )
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
        scroll={{ x: 1800 }}
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
