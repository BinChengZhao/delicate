import React, { PureComponent } from 'react'
import { connect } from 'react-redux'
import { Page } from '../../components'
import { Descriptions, Form, Input, Modal, Select } from 'antd'
import store from 'store'
import { t } from '@lingui/macro'

function mapStateToProps(state) {
  return {}
}
const FormItem = Form.Item
const formItemLayout = {
  labelCol: {
    span: 6
  },
  wrapperCol: {
    span: 16
  }
}

@connect(({ user, loading }) => ({ user, loading }))
class personalCenter extends PureComponent {
  formRef = React.createRef()

  constructor(props) {
    super(props)
    this.state = {
      userInfo: store.get('user'),
      isModalVisible: false
    }
  }

  toggleModal() {
    const { isModalVisible } = this.state
    this.setState({ isModalVisible: !isModalVisible })
  }

  onChangePassword() {
    const { dispatch } = this.props
    const values = this.formRef.current.getFieldsValue()
    values.identity_type = 3
    dispatch({ type: `user/updatePassword`, payload: values }).then((ret) => {
      if (!ret.code) this.toggleModal()
    })
  }

  render() {
    const { userInfo, isModalVisible } = this.state
    const { loading } = this.props
    return (
      <Page inner>
        <Descriptions title={userInfo.nick_name}>
          <Descriptions.Item label={t`Username`}>{userInfo.user_name}</Descriptions.Item>
          <Descriptions.Item label={t`MobileNo.`}>{userInfo.mobile}</Descriptions.Item>
          <Descriptions.Item label={t`Email`}>{userInfo.email}</Descriptions.Item>
          <Descriptions.Item label={t`Password`}>
            <a onClick={() => this.toggleModal()}>修改密码</a>
          </Descriptions.Item>
        </Descriptions>

        <Modal
          title={t`Update` + t`Password`}
          visible={isModalVisible}
          confirmLoading={loading.effects[`user/updatePassword`]}
          onOk={() => this.onChangePassword()}
          onCancel={() => this.toggleModal()}
        >
          <Form ref={this.formRef} name="control-ref" layout="horizontal">
            <FormItem
              name="current_password"
              rules={[{ required: true, min: 5, max: 20 }]}
              label={t`Current` + t`Password`}
              hasFeedback
              {...formItemLayout}
            >
              <Input type={'password'} />
            </FormItem>
            <FormItem
              name="modified_password"
              rules={[{ required: true, min: 5, max: 20 }]}
              label={t`New` + t`Password`}
              hasFeedback
              {...formItemLayout}
            >
              <Input type={'password'} />
            </FormItem>
            <FormItem
              name="confirmPassword"
              label={t`Confirm` + t`Password`}
              dependencies={['modified_password']}
              hasFeedback
              rules={[
                {
                  required: true,
                  message: 'Please confirm your password!'
                },
                ({ getFieldValue }) => ({
                  validator(_, value) {
                    if (!value || getFieldValue('modified_password') === value) {
                      return Promise.resolve()
                    }
                    return Promise.reject(new Error('The two passwords that you entered do not match!'))
                  }
                })
              ]}
              {...formItemLayout}
            >
              <Input type={'password'} />
            </FormItem>
          </Form>
        </Modal>
      </Page>
    )
  }
}

export default connect(mapStateToProps)(personalCenter)
