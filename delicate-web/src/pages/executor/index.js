import React, { PureComponent } from 'react'
import { connect } from 'dva'
import { Page } from '../../components'
import { t, Trans } from '@lingui/macro'
import { Button, Col, Dropdown, Form, Input, Menu, Row, Select, Space, Table, Tooltip } from 'antd'
import { DownOutlined } from '@ant-design/icons'
import PropTypes from 'prop-types'

function mapStateToProps(state) {
  return {}
}
const FormItem = Form.Item
const Option = Select.Option
@connect(({ executorModel, loading }) => ({ executorModel, loading }))
class Executor extends PureComponent {
  formRef = React.createRef()

  get modalProps() {}

  render() {
    console.log(this.props)

    const { location } = this.props
    const Data = [{}, {}]
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
        render: (text) => {
          return (
            <Space split={'|'}>
              <a type={'link'}>编辑</a>
              <Dropdown
                overlay={
                  <Menu>
                    <Menu.Item>复制节点</Menu.Item>
                    <Menu.Item>激活节点</Menu.Item>
                    <Menu.Item>冻结节点</Menu.Item>
                    <Menu.Item danger>删除节点</Menu.Item>
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
      <Page inner>
        <Form ref={this.formRef} name="control-ref">
          <Row gutter={24}>
            <Col xl={{ span: 4 }} md={{ span: 8 }}>
              <Form.Item name="tag">
                <Input placeholder={t`Tag`} />
              </Form.Item>
            </Col>
            <Col xl={{ span: 4 }} md={{ span: 8 }}>
              <Form.Item name="name">
                <Input placeholder="节点名称" />
              </Form.Item>
            </Col>
            <Col xl={{ span: 4 }} md={{ span: 8 }}>
              <Form.Item name="machine_id">
                <Input placeholder="机器 ID" />
              </Form.Item>
            </Col>
            <Button type="primary" htmlType="submit" className="margin-right">
              <Trans>Search</Trans>
            </Button>
            <Button className="margin-right">
              <Trans>Reset</Trans>
            </Button>
            <Button type="ghost">
              <Trans>Create</Trans>
            </Button>
          </Row>
        </Form>

        <Table columns={columns} dataSource={Data} simple rowKey={(record) => record.id} />
      </Page>
    )
  }
}
Executor.propTypes = {
  executorModel: PropTypes.object
}
export default connect(mapStateToProps)(Executor)
