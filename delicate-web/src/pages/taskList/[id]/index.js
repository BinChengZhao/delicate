import React, { PureComponent } from 'react'
import { Page } from '../../../components'
import {
  Button,
  Col,
  DatePicker,
  Descriptions,
  Form,
  Input,
  message,
  Modal,
  Popconfirm,
  Row,
  Select,
  Table,
  Tooltip
} from 'antd'
import { t, Trans } from '@lingui/macro'
import { connect } from 'umi'
import PropTypes from 'prop-types'
import moment from 'moment'

const { Option } = Select
const STATUS_RUN_ING = 1
const STATUS_LIST = [
  { key: 1, value: 1, title: '运行中' },
  { key: 2, value: 2, title: '正常结束' },
  { key: 3, value: 3, title: '异常结束' },
  { key: 4, value: 4, title: '超时' },
  { key: 5, value: 5, title: '手动取消' },
  { key: 81, value: 81, title: '未知' }
]

@connect(({ taskModel, loading }) => ({ taskModel, loading }))
class TaskLog extends PureComponent {
  constructor(props) {
    super(props)
    this.state = {
      visible: false,
      logDetail: this.initLogDetail()
    }
  }

  formRef = React.createRef()

  handleRefresh = (newQuery) => {
    const { taskModel, dispatch } = this.props
    const queryWhere = taskModel.logQueryWhere
    const payload = { ...queryWhere, ...newQuery }
    dispatch({ type: `taskModel/taskLogList`, payload: payload })
  }

  handleSubmit() {
    const { dispatch, location } = this.props
    const values = this.formRef.current.getFieldsValue()
    for (const i in values) {
      values[i] = values[i] === '' ? null : values[i]
    }
    const initFlitter = this.initFilter()
    const taskId = location.state.id || null
    const timeRange = {
      start_time: null,
      end_time: null
    }
    if (values.time_range) {
      timeRange.start_time = parseInt(moment(values.time_range[0]._d).valueOf() / 1000)
      timeRange.end_time = parseInt(moment(values.time_range[1]._d).valueOf() / 1000)
    }
    delete values.time_range
    dispatch({
      type: `taskModel/taskLogList`,
      payload: { ...initFlitter, ...values, task_id: taskId, ...timeRange }
    })
  }

  handleReset() {
    const fields = this.formRef.current.getFieldsValue()
    for (const item in fields) {
      if ({}.hasOwnProperty.call(fields, item)) fields[item] = undefined
    }
    this.formRef.current.setFieldsValue(fields)
    this.handleSubmit()
  }

  confirm(row) {
    const { dispatch } = this.props
    const params = {
      task_id: row.task_id,
      record_id: row.id,
      executor_processor_id: row.executor_processor_id
    }
    dispatch({ type: 'taskModel/onTaskKill', payload: params }).then((ret) => {
      if (!ret.code) this.handleSubmit()
    })
  }

  cancel() {
    message.info(t`Cancel`)
  }

  taskLogDetail(recordId) {
    const { dispatch } = this.props
    dispatch({ type: 'taskModel/taskLogDetail', payload: { record_id: recordId } }).then((ret) => {
      if (!ret.code) this.setState({ logDetail: ret.data }, () => this.toggleVisible())
    })
  }

  toggleVisible() {
    const { visible } = this.state
    this.setState({ visible: !visible })
  }

  cancelModal() {
    this.setState({ logDetail: this.initLogDetail() }, () => this.toggleVisible())
  }

  componentDidMount() {
    this.handleSubmit()
  }

  initFilter() {
    return {
      id: null,
      name: null,
      description: null,
      command: null,
      tag: null,
      status: null,
      executor_processor_id: null,
      time_range: null,
      start_time: null,
      end_time: null,
      per_page: 10,
      page: 1
    }
  }

  initLogDetail() {
    return { id: '', task_id: '', stdout: '', stderr: '' }
  }

  render() {
    const { location, taskModel, loading } = this.props
    const { state } = location
    const { logSource, logPagination } = taskModel
    const { visible, logDetail } = this.state

    const columns = [
      {
        title: <Trans>Sn</Trans>,
        dataIndex: 'id',
        key: 'id',
        width: 200,
        fixed: 'left'
      },
      {
        title: '执行时-名称',
        dataIndex: 'name',
        key: 'name',
        width: 120,
        fixed: 'left',
        ellipsis: true,
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
        title: '状态',
        dataIndex: 'status',
        key: 'status',
        width: 100,
        render: (text) => {
          const item = STATUS_LIST.find((i) => i.key === text)
          return `${text}: ${item.title}`
        }
      },
      {
        title: '节点执行ID',
        dataIndex: 'record_id',
        key: 'record_id'
      },
      {
        title: 'Host',
        dataIndex: 'executor_processor_host',
        key: 'executor_processor_host'
      },
      {
        title: '表达式',
        dataIndex: 'cron_expression',
        width: 200,
        key: 'cron_expression'
      },
      {
        title: '进程开始时间',
        dataIndex: 'created_time',
        width: 200
      },
      {
        title: '最大并行运行数',
        dataIndex: 'maximum_parallel_runnable_num',
        width: 200,
        key: 'maximum_parallel_runnable_num'
      },
      {
        title: <Trans>Frequency</Trans>,
        dataIndex: 'frequency',
        width: 200,
        key: 'frequency'
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
        title: t`Operation`,
        fixed: 'right',
        key: 'operating',
        render: (text, row) => {
          return (
            <div>
              <Button type={'link'} onClick={() => this.taskLogDetail(row.id)}>
                查看详情
              </Button>

              <Popconfirm
                title={`确定要中断进程【${row.id}】吗？`}
                onConfirm={() => this.confirm(row)}
                onCancel={() => this.cancel()}
                okText="Yes"
                cancelText="No"
                disabled={row.status !== STATUS_RUN_ING}
              >
                <Button disabled={row.status !== STATUS_RUN_ING} type={'link'} danger>
                  中断进程
                </Button>
              </Popconfirm>
            </div>
          )
        }
      }
    ]

    return (
      <Page inner>
        <Descriptions title={state.name}>
          <Descriptions.Item label={t`Description`} span={3}>
            {state.description}
          </Descriptions.Item>
        </Descriptions>

        <Form ref={this.formRef} name="control-ref" initialValues={this.initFilter()}>
          <Row gutter={24}>
            <Col xl={{ span: 4 }} md={{ span: 8 }}>
              <Form.Item name="status">
                <Select allowClear placeholder={'状态'}>
                  {STATUS_LIST.map((item) => {
                    // eslint-disable-next-line react/jsx-key
                    return <Option {...item}>{item.title}</Option>
                  })}
                </Select>
              </Form.Item>
            </Col>
            <Col xl={{ span: 4 }} md={{ span: 8 }}>
              <Form.Item name="executor_processor_id">
                <Input placeholder={t`Machine Id`} />
              </Form.Item>
            </Col>
            <Col xl={{ span: 8 }} md={{ span: 8 }}>
              <Form.Item name="time_range">
                <DatePicker.RangePicker showTime format="YYYY-MM-DD HH:mm:ss" />
              </Form.Item>
            </Col>

            <Button type="primary" htmlType="submit" className="margin-right" onClick={() => this.handleSubmit()}>
              <Trans>Search</Trans>
            </Button>
            <Button className="margin-right">
              <Trans>Reset</Trans>
            </Button>
          </Row>
        </Form>

        <Table
          size={'small'}
          columns={columns}
          dataSource={logSource}
          scroll={{ x: 2000 }}
          loading={loading.effects[`taskModel/taskLogList`]}
          pagination={{
            ...logPagination,
            onChange: (page, pageSize) => {
              this.handleRefresh({
                page: page,
                per_page: pageSize
              })
            }
          }}
          simple
          rowKey={(record) => record.id}
        />
        <Modal visible={visible} title={'日志详情'} onCancel={() => this.cancelModal()} width={800} footer={null}>
          <p>
            <b>任务ID</b>:{logDetail.task_id}
          </p>
          <p>
            <b>日志ID</b>:{logDetail.id}
          </p>
          <br />
          <p>
            <b>日志详情</b>:{logDetail.stdout}
          </p>
          <hr />
          <p>
            <b>日志错误</b>:{logDetail.stderr}
          </p>
        </Modal>
      </Page>
    )
  }
}

TaskLog.propTypes = {
  taskList: PropTypes.object
}

export default TaskLog
