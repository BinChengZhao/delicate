import React, { PureComponent } from 'react'
import { connect } from 'dva'
import { Page } from '../../components'
import { Table } from 'antd'
import { t } from '@lingui/macro'

const NAMESPACE = 'loginLogs'

@connect(({ loginLogs, loading }) => ({ loginLogs, loading }))
class loginLogs extends PureComponent {
  fetchList = (extra) => {
    const { dispatch } = this.props
    const condition = { page: 1, per_page: 10 }
    const payload = { ...condition, ...extra }
    dispatch({ type: `${NAMESPACE}/getLoginLogs`, payload }).then((ret) => {
      const { dataSource, pagination, stateDesc } = ret.data
      this.setState({ dataSource, pagination, stateDesc })
    })
  }

  constructor(props) {
    super(props)
    this.state = {
      dataSource: [],
      pagination: { total: 0, pageSize: 10 }
    }
  }

  get FilterProps() {
    return {
      onFilterChange: (payload) => this.fetchList(payload)
    }
  }

  componentDidMount() {
    this.fetchList()
  }

  get columns() {
    return [
      {
        title: t`Sn`,
        dataIndex: 'id'
      },
      {
        title: t`User Id`,
        dataIndex: 'user_id'
      },
      {
        title: t`Login Type`,
        dataIndex: 'login_type'
      },
      {
        title: t`Login Status`,
        dataIndex: 'command'
      },
      {
        title: t`Last Ip`,
        dataIndex: 'lastip'
      },
      {
        title: t`Username`,
        dataIndex: 'user_name'
      },
      {
        title: t`CreateTime`,
        dataIndex: 'created_time'
      }
    ]
  }

  render() {
    const { dataSource, pagination } = this.state
    const { loading } = this.props
    return (
      <Page inner>
        <Table
          loading={loading.effects[`${NAMESPACE}/getLoginLogs`]}
          pagination={{
            ...pagination,
            onChange: (page) => this.fetchList({ page })
          }}
          columns={this.columns}
          dataSource={dataSource}
          simple
          rowKey={(record) => record.id}
        />
      </Page>
    )
  }
}

export default loginLogs
