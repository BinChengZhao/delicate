import React, { PureComponent } from 'react'
import PropTypes from 'prop-types'
import { Form, Input, InputNumber, Modal, Select, Switch } from 'antd'
import { t } from '@lingui/macro'
import { InputCron } from 'react-crons'
import * as u from '../../../utils/data'

const FormItem = Form.Item
const Option = Select.Option
const formItemLayout = {
  labelCol: {
    span: 4
  },
  wrapperCol: {
    span: 18
  }
}

class TaskModal extends PureComponent {
  formRef = React.createRef()

  constructor(props) {
    super(props)
    this.state = {
      forms: this.initForms(),
      bindList: [],
      modeVale: 1
    }
  }

  initForms() {
    return {
      id: null,
      name: '',
      description: '',
      command: '";',
      frequency: { extend: { count: 1 }, mode: 1, time_zone: 1 },
      cron_expression: '* * * * * ?',
      timeout: 300,
      retry_times: 1,
      retry_interval: 10,
      maximum_parallel_runnable_num: 2,
      tag: [],
      binding_ids: [],
      status: 2 // true: 2 启用 ｜ false： 1 未启用
    }
  }

  setParams(data) {
    const params = { task: data, binding_ids: data.binding_ids }
    delete data.binding_ids
    console.log(params)
    return params
  }

  handleOk = () => {
    const { item = {}, onOk } = this.props
    this.formRef.current
      .validateFields()
      .then((values) => {
        const data = {
          ...item,
          ...values,
          tag: values.tag.join(','),
          frequency: JSON.stringify(values.frequency),
          cron_expression: values.cron_expression.replaceAll('?', '*') + ' *'
        }
        const params = this.setParams(data)
        onOk(params)
      })
      .catch((errorInfo) => {
        console.log(errorInfo)
      })
  }

  bindList() {
    const { getBindList } = this.props
    getBindList().then((bindList) => this.setState({ bindList }))
  }

  componentDidMount() {
    // 页面渲染完毕后立即获取bindList
    this.bindList()
  }

  render() {
    const { onOk, form, ...modalProps } = this.props
    const { forms, bindList } = this.state
    const initValues = !u.isEmpty(modalProps.item) ? modalProps.item : forms
    return (
      <Modal {...modalProps} onOk={this.handleOk}>
        <Form ref={this.formRef} name="control-ref" layout="horizontal" initialValues={initValues}>
          <FormItem
            name="name"
            label={t`Task Name`}
            rules={[{ required: true, message: '任务名称必须填写' }]}
            hasFeedback
            {...formItemLayout}
          >
            <Input placeholder="demo" />
          </FormItem>
          <FormItem
            name="description"
            rules={[{ required: true, message: '描述必须填写' }]}
            label={t`Description`}
            hasFeedback
            {...formItemLayout}
          >
            <Input placeholder="这是一条【demo】的命令" />
          </FormItem>
          <FormItem
            name="command"
            label="命令行"
            rules={[{ required: true, message: '命令行必须填写' }]}
            hasFeedback
            {...formItemLayout}
          >
            <Input placeholder="echo 'hello word';" />
          </FormItem>

          <FormItem label="频率" hasFeedback {...formItemLayout}>
            <Input.Group compact {...formItemLayout}>
              <Form.Item name={['frequency', 'mode']} noStyle rules={[{ required: true, message: '模式必须选择' }]}>
                <Select placeholder="模式" style={{ width: '25%' }}>
                  <Option key={1} value={1}>
                    Once
                  </Option>
                  <Option key={2} value={2}>
                    CountDown
                  </Option>
                  <Option key={3} value={3}>
                    Repeat
                  </Option>
                </Select>
              </Form.Item>
              <Form.Item name={['frequency', 'extend', 'count']} noStyle>
                <Input style={{ width: '50%' }} placeholder="运行次数,CountDown模式下有效" />
              </Form.Item>
              <Form.Item
                name={['frequency', 'time_zone']}
                noStyle
                rules={[{ required: true, message: '时区必须选择' }]}
              >
                <Select placeholder="时区" style={{ width: '25%' }}>
                  <Option key={1} value={1}>
                    Local
                  </Option>
                  <Option key={2} value={2}>
                    Utc
                  </Option>
                </Select>
              </Form.Item>
            </Input.Group>
          </FormItem>
          <FormItem
            name="cron_expression"
            label="Cron 表达式"
            rules={[{ required: true, message: '表达式不能为空' }]}
            hasFeedback
            {...formItemLayout}
          >
            <InputCron lang={'zh-Hans-CN'} type={['second', 'minute', 'hour', 'day', 'month', 'week']} />
          </FormItem>

          <Form.Item label="时间调度" style={{ marginBottom: 0 }} hasFeedback {...formItemLayout}>
            <Form.Item
              name="timeout"
              label={'超时时间'}
              rules={[{ required: true, message: '未设置超时时间' }]}
              style={{ display: 'inline-block', width: 'calc(32% - 8px)' }}
            >
              <InputNumber placeholder="单位：秒" min={10} max={10000} />
            </Form.Item>
            <Form.Item
              name="retry_times"
              label={'重试次数'}
              rules={[{ required: true, message: '未设置重试次数' }]}
              style={{
                display: 'inline-block',
                width: 'calc(32% - 8px)',
                margin: '0 8px'
              }}
            >
              <InputNumber placeholder="单位：次" min={1} max={100} />
            </Form.Item>
            <Form.Item
              name="retry_interval"
              label={'重试间隔'}
              rules={[{ required: true, message: '未设置重置间隔' }]}
              style={{
                display: 'inline-block',
                width: 'calc(32% - 8px)',
                margin: '0 8px'
              }}
            >
              <InputNumber placeholder="单位：秒" min={10} max={10000} />
            </Form.Item>
          </Form.Item>
          <FormItem
            name="maximum_parallel_runnable_num"
            label="单节点最大并行"
            rules={[{ required: true, message: '未设置最大并行' }]}
            hasFeedback
            {...formItemLayout}
          >
            <InputNumber min={1} max={10000} />
          </FormItem>
          <FormItem name="tag" label="任务标签" hasFeedback {...formItemLayout}>
            <Select mode="tags" allowClear style={{ width: '100%' }} placeholder="支持自定义标签" />
          </FormItem>
          <FormItem name="binding_ids" label="机器绑定" hasFeedback {...formItemLayout}>
            <Select mode={'multiple'} placeholder="请选择" onFocus={() => this.bindList()}>
              {bindList.map((point, i) => {
                return (
                  <Select.Option key={parseInt(point.id)} value={parseInt(point.id)}>
                    {point.title}
                  </Select.Option>
                )
              })}
            </Select>
          </FormItem>
        </Form>
      </Modal>
    )
  }
}

TaskModal.propTypes = {
  type: PropTypes.string,
  item: PropTypes.object,
  onOk: PropTypes.func,
  getBindList: PropTypes.func
}

export default TaskModal
