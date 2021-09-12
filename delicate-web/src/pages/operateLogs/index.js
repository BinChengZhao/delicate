import React, { PureComponent } from 'react'
import { connect } from 'dva'
import { Page } from '../../components'
import Filter from './components/Filter'
import { Table } from 'antd'
import { t } from '@lingui/macro'
import { Link } from 'umi'

const NAMESPACE = 'operateLogs'

@connect(({ operateLogs, loading }) => ({ operateLogs, loading }))
class operateLogs extends PureComponent {
  fetchList = (extra) => {
    const { operateLogs, dispatch } = this.props
    const condition = operateLogs.queryWhere
    const payload = { ...condition, ...extra }
    dispatch({ type: `${NAMESPACE}/getOperateLogs`, payload }).then((ret) => {
      const { dataSource, pagination } = ret.data
      this.setState({ dataSource, pagination })
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

  get columns() {
    return [
      {
        title: t`Sn`,
        dataIndex: 'id'
      },
      {
        title: t`Operate Name`,
        dataIndex: 'name'
      },
      {
        title: t`Operate Type`,
        dataIndex: 'operation_type_desc'
      },
      {
        title: t`Table Id`,
        dataIndex: 'table_id'
      },
      {
        title: t`Username`,
        dataIndex: 'user_name'
      },
      {
        title: t`Operate Time`,
        dataIndex: 'operation_time'
      },
      {
        title: t`Operation`,
        render: (text, row) => <Link to={{ pathname: `operateLogs/${row.id}` }}>{t`Detail`}</Link>
      }
    ]
  }

  render() {
    const { dataSource, pagination } = this.state
    const { loading } = this.props
    return (
      <Page inner>
        <Filter {...this.FilterProps} />
        <Table
          loading={loading.effects[`${NAMESPACE}/getOperateLogs`]}
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

export default operateLogs
