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
    const { item = {}, onOk, modalType, onGroupBindExecutor, onRefresh } = this.props

    this.formRef.current
      .validateFields()
      .then((values) => {
        const data = { ...values, id: item.id }
        data.tag = data.tag && data.tag.join(',')
        onOk(data).then((ret) => {
          if (modalType === 'create') {
            // 绑定
            const bindParams = {
              group_id: ret.data,
              executor_ids: data.executor_ids,
              name: data.name + '绑定执行器',
              weight: 0
            }
            onGroupBindExecutor(bindParams)
          } else {
            onRefresh()
          }
        })
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
          <FormItem name="name" label={t`Group Name`} rules={[{ required: true }]} hasFeedback {...formItemLayout}>
            <Input placeholder={t`Group Name`} />
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

          <FormItem name="tag" label={t`Task Tag`} rules={[{ required: true }]} hasFeedback {...formItemLayout}>
            <Select mode="tags" allowClear style={{ width: '100%' }} placeholder={t`Support custom labels`} />
          </FormItem>
          {modalType === 'create' ? (
            <FormItem name="executor_ids" label="绑定执行器" hasFeedback {...formItemLayout}>
              <Select mode={'multiple'} placeholder="请选择执行器ID" onFocus={() => this.getGroupBindList()}>
                {bindList.map((point, i) => {
                  return (
                    <Select.Option key={parseInt(point.id)} value={parseInt(point.id)}>
                      {point.title}
                    </Select.Option>
                  )
                })}
              </Select>
            </FormItem>
          ) : null}
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
  onRefresh: PropTypes.func,
  item: PropTypes.object
}

export default ExecutorGroupModal
