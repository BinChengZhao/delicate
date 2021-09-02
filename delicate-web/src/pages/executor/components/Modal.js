import React, { PureComponent } from 'react'
import PropTypes from 'prop-types'
import { Form, Input, InputNumber, Modal, Select } from 'antd'
import { t } from '@lingui/macro'

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

class ExecutorModal extends PureComponent {
  formRef = React.createRef()

  constructor(props) {
    super(props)
    this.state = {
      forms: this.initForms()
    }
  }

  initForms() {
    return {
      id: null,
      name: '',
      tag: [],
      host: '',
      description: '',
      machine_id: ''
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
        const data = { ...values, id: item.id }

        data.tag = data.tag.join(',')
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
    return (
      <Modal {...modalProps} onOk={this.handleOk}>
        <Form ref={this.formRef} name="control-ref" layout="horizontal" initialValues={initValues}>
          <FormItem name="name" label={t`Node Name`} rules={[{ required: true }]} hasFeedback {...formItemLayout}>
            <Input placeholder={t`Node Name`} />
          </FormItem>
          <FormItem
            name="description"
            rules={[{ required: true }]}
            label={t`Description`}
            hasFeedback
            {...formItemLayout}
          >
            <Input placeholder={t`Description`} />
          </FormItem>
          <FormItem name="host" label="Host" rules={[{ required: true }]} hasFeedback {...formItemLayout}>
            <Input placeholder="127.0.0.1:8080" />
          </FormItem>

          <FormItem
            name="machine_id"
            label={t`Machine Id`}
            rules={[{ required: true }]}
            hasFeedback
            {...formItemLayout}
          >
            <InputNumber max={1024} min={0} />
          </FormItem>

          <FormItem name="tag" label={t`Task Tag`} hasFeedback {...formItemLayout}>
            <Select mode="tags" allowClear style={{ width: '100%' }} placeholder={t`Support custom labels`} />
          </FormItem>
        </Form>
      </Modal>
    )
  }
}

ExecutorModal.propTypes = {
  type: PropTypes.string,
  item: PropTypes.object
}

export default ExecutorModal
