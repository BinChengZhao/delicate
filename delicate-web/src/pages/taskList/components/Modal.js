import React, { PureComponent } from 'react'
import PropTypes from 'prop-types'
import { Form, Input, InputNumber, Modal, Select, Switch } from 'antd'
import { t } from '@lingui/macro'
import { InputCron } from 'react-crons'

const FormItem = Form.Item
const Option = Select.Option
const formItemLayout = {
  labelCol: {
    span: 6
  },
  wrapperCol: {
    span: 14
  }
}

class TaskModal extends PureComponent {
  formRef = React.createRef()

  constructor(props) {
    super(props)
    this.state = {
      forms: this.initForms()
    }
  }

  initForms() {
    return {
      name: '',
      description: '',
      command: '',
      frequency: '{modal:2}',
      modal: 1,
      cron_expression: '* * * * * ?',
      timeout: '180',
      retry_times: '180',
      retry_interval: '3',
      maximum_parallel_runnable_num: '5',
      tag: [],
      bind_id: '',
      status: true // true: 1 启用 ｜ false： 2 未启用
    }
  }

  optionAttr(value) {
    return { key: value, value }
  }

  handleOk = () => {
    const { item = {}, onOk } = this.props
    this.formRef.current
      .validateFields()
      .then((values) => {
        const data = {
          ...values,
          tag: values.tag.join(',')
        }
        onOk(data)
      })
      .catch((errorInfo) => {
        console.log(errorInfo)
      })
  }

  render() {
    const { onOk, form, ...modalProps } = this.props
    const { forms } = this.state
    const initValues = modalProps.item ? modalProps.item : forms
    console.log(initValues)
    return (
      <Modal {...modalProps} onOk={this.handleOk}>
        <Form ref={this.formRef} name="control-ref" layout="horizontal" initialValues={initValues}>
          <FormItem name="name" label={t`Task Name`} rules={[{ required: true }]} hasFeedback {...formItemLayout}>
            <Input placeholder="查看PHP版本" />
          </FormItem>
          <FormItem
            name="description"
            rules={[{ required: true }]}
            label={t`Description`}
            hasFeedback
            {...formItemLayout}
          >
            <Input placeholder="这是一条【查看PHP版本号】的命令" />
          </FormItem>
          <FormItem name="command" label="命令行" rules={[{ required: true }]} hasFeedback {...formItemLayout}>
            <Input placeholder="php -v" />
          </FormItem>
          <FormItem
            name="frequency"
            label="频率"
            rules={[{ required: true, message: '扩展字段示例：{count:3}' }]}
            hasFeedback
            {...formItemLayout}
          >
            <Input
              addonBefore={
                <Select placeholder="模式" defaultValue={forms.modal}>
                  <Option {...this.optionAttr(1)}>模式1</Option>
                  <Option {...this.optionAttr(2)}>模式2</Option>
                </Select>
              }
              placeholder="扩展字段示例：{count:3}"
            />
          </FormItem>

          <FormItem
            name="cron_expression"
            label="Cron 表达式"
            rules={[{ required: true }]}
            hasFeedback
            {...formItemLayout}
          >
            <InputCron
              onChange={(v) => console.log(v)}
              type={['second', 'minute', 'hour', 'day', 'month', 'week', 'year']}
            />
          </FormItem>

          <Form.Item label="时间调度" style={{ marginBottom: 0 }} hasFeedback {...formItemLayout}>
            <Form.Item
              name="timeout"
              label={'超时时间'}
              rules={[{ required: true }]}
              style={{ display: 'inline-block', width: 'calc(32% - 8px)' }}
            >
              <InputNumber placeholder="单位：秒" min={10} max={10000} />
            </Form.Item>
            <Form.Item
              name="retry_times"
              label={'重试时间'}
              rules={[{ required: true }]}
              style={{
                display: 'inline-block',
                width: 'calc(32% - 8px)',
                margin: '0 8px'
              }}
            >
              <InputNumber placeholder="单位：秒" min={10} max={10000} />
            </Form.Item>
            <Form.Item
              name="retry_interval"
              label={'重试间隔'}
              rules={[{ required: true }]}
              style={{
                display: 'inline-block',
                width: 'calc(32% - 8px)',
                margin: '0 8px'
              }}
            >
              <InputNumber placeholder="单位：秒" min={1} max={10} />
            </Form.Item>
          </Form.Item>
          <FormItem
            name="maximum_parallel_runnable_num"
            label="单节点最大并行"
            rules={[{ required: true }]}
            hasFeedback
            {...formItemLayout}
          >
            <InputNumber min={1} />
          </FormItem>
          <FormItem name="tag" label="任务标签" hasFeedback {...formItemLayout}>
            <Select mode="tags" allowClear style={{ width: '100%' }} placeholder="支持自定义标签" />
          </FormItem>
          <FormItem name="bind_id" label="组ID" hasFeedback {...formItemLayout}>
            <Input placeholder="Please input" />
          </FormItem>
          <FormItem name="status" label="是否启用" valuePropName="checked" hasFeedback {...formItemLayout}>
            <Switch checkedChildren="Yes" unCheckedChildren="No" defaultChecked />
          </FormItem>
        </Form>
      </Modal>
    )
  }
}

TaskModal.propTypes = {
  type: PropTypes.string,
  item: PropTypes.object,
  onOk: PropTypes.func
}

export default TaskModal
