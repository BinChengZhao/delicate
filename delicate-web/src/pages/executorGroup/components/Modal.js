import React, { PureComponent } from 'react'
import PropTypes from 'prop-types'
import { Form, Input, Modal, Select } from 'antd'
import { t } from '@lingui/macro'

const FormItem = Form.Item
const formItemLayout = {
  labelCol: {
    span: 6
  },
  wrapperCol: {
    span: 14
  }
}

class ExecutorGroupModal extends PureComponent {
  formRef = React.createRef()

  constructor(props) {
    super(props)
    this.state = {
      forms: this.initForms(),
      bindList: []
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

  getGroupBindList() {
    const { getGroupBindList } = this.props
    getGroupBindList().then((bindList) => this.setState({ bindList }))
  }

  handleOk = () => {
    const { item = {}, modalType, onOk, onGroupBindExecutor } = this.props

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
    const { onOk, form, modalType, ...modalProps } = this.props
    const { forms, bindList } = this.state
    const initValues = modalProps.item ? modalProps.item : forms
    return (
      <Modal {...modalProps} onOk={this.handleOk}>
        <Form ref={this.formRef} name="control-ref" layout="horizontal" initialValues={initValues}>
          <FormItem name="name" label="执行器名称" rules={[{ required: true }]} hasFeedback {...formItemLayout}>
            <Input placeholder="执行器名称" />
          </FormItem>
          <FormItem
            name="description"
            rules={[{ required: true }]}
            label={t`Description`}
            hasFeedback
            {...formItemLayout}
          >
            <Input placeholder="这个执行器的描述" />
          </FormItem>

          <FormItem name="tag" label="任务标签" hasFeedback {...formItemLayout}>
            <Select mode="tags" allowClear style={{ width: '100%' }} placeholder="支持自定义标签" />
          </FormItem>
        </Form>
      </Modal>
    )
  }
}
ExecutorGroupModal.propTypes = {
  type: PropTypes.string,
  onOk: PropTypes.func,
  getGroupBindList: PropTypes.func,
  onGroupBindExecutor: PropTypes.func,
  groupUsedExecutor: PropTypes.func,
  item: PropTypes.object
}

export default ExecutorGroupModal
