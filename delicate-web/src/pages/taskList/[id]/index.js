import React, { PureComponent } from 'react'
import { Page } from '../../../components'
import { Button, Col, Descriptions, Dropdown, Form, Input, Row, Select, Space, Table, Tooltip } from 'antd'
import { t, Trans } from '@lingui/macro'
import { connect } from 'umi'
import PropTypes from 'prop-types'
import { DownOutlined } from '@ant-design/icons'

const { Option } = Select

@connect(({ taskList }) => ({ taskList }))
class TaskLog extends PureComponent {
  constructor(props) {
    super(props)
  }

  formRef = React.createRef()

  optionAttr(value) {
    return { key: value, value }
  }

  initFlitter() {
    return {
      name: null,
      description: null,
      command: null,
      tag: null,
      task_id: null,
      id: null,
      status: null,
      executor_processor_id: null,
      per_page: 10,
      page: 1
    }
  }

  render() {
    const { location } = this.props
    const { state } = location

    const columns = [
      {
        title: <Trans>Sn</Trans>,
        dataIndex: 'id',
        key: 'id',
        width: 70,
        fixed: 'left'
      },
      {
        title: '执行时-名称',
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
        title: <Trans>Command</Trans>,
        dataIndex: 'command',
        width: 200,
        key: 'command'
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
        title: '机器节点id',
        dataIndex: 'executor_processor_id',
        key: 'executor_processor_id'
      },
      {
        title: '节点执行ID',
        dataIndex: 'record_id',
        key: 'record_id'
      }
    ]

    return (
      <Page inner>
        <Descriptions title={state.name}>
          <Descriptions.Item label="描述" span={3}>
            {state.description}
          </Descriptions.Item>
        </Descriptions>

        <Form ref={this.formRef} name="control-ref" initialValues={this.initFlitter()}>
          <Row gutter={24}>
            <Col xl={{ span: 4 }} md={{ span: 8 }}>
              <Form.Item name="tag">
                <Input placeholder={t`Tag`} />
              </Form.Item>
            </Col>
            <Col xl={{ span: 4 }} md={{ span: 8 }}>
              <Form.Item name="status">
                <Select placeholder={'状态'}>
                  <Option {...this.optionAttr(1)}>{'运行中'}</Option>
                  <Option {...this.optionAttr(2)}>{'正常结束'}</Option>
                  <Option {...this.optionAttr(3)}>{'异常结束'}</Option>
                  <Option {...this.optionAttr(4)}>{'超时'}</Option>
                  <Option {...this.optionAttr(5)}>{'手动取消'}</Option>
                  <Option {...this.optionAttr(81)}>{'未知'}</Option>
                </Select>
              </Form.Item>
            </Col>
            <Col xl={{ span: 4 }} md={{ span: 8 }}>
              <Form.Item name="executor_processor_id">
                <Input placeholder="执行器处理器 ID" />
              </Form.Item>
            </Col>
            <Button type="primary" htmlType="submit" className="margin-right">
              <Trans>Search</Trans>
            </Button>
            <Button className="margin-right">
              <Trans>Reset</Trans>
            </Button>
          </Row>
        </Form>

        <Table columns={columns} simple rowKey={(record) => record.id} />
      </Page>
    )
  }
}

TaskLog.propTypes = {
  taskList: PropTypes.object
}

export default TaskLog
