import React, { Component } from 'react'
import PropTypes from 'prop-types'
import { Trans } from '@lingui/macro'

import { Button, Col, Form, Input, Row } from 'antd'

const { Search } = Input

const ColProps = {
  xs: 24,
  sm: 12,
  style: {
    marginBottom: 16
  }
}

class Filter extends Component {
  formRef = React.createRef()

  constructor(props) {
    super(props)
    this.state = {
      forms: this.initForms()
    }
  }

  initForms() {
    return {
      user_name: null,
      nick_name: null,
      mobile: null,
      email: null,
      id: null,
      status: null,
      per_page: 10,
      page: 1
    }
  }

  handleSubmit = () => {
    const { onFilterChange } = this.props
    const values = this.formRef.current.getFieldsValue()
    const initFlitter = this.initForms()
    onFilterChange({ ...initFlitter, ...values })
  }

  handleReset = () => {
    const fields = this.formRef.current.getFieldsValue()
    for (const item in fields) {
      if ({}.hasOwnProperty.call(fields, item)) fields[item] = undefined
    }
    this.formRef.current.setFieldsValue(fields)
    this.handleSubmit()
  }

  componentDidMount() {
    this.handleSubmit()
  }

  render() {
    const { onAdd } = this.props

    return (
      <Form ref={this.formRef} name="control-ref">
        <Row gutter={24}>
          <Col {...ColProps} xl={{ span: 4 }} md={{ span: 8 }}>
            <Form.Item name="nick_name">
              <Input placeholder={'昵称'} />
            </Form.Item>
          </Col>
          <Col {...ColProps} xl={{ span: 4 }} md={{ span: 8 }}>
            <Form.Item name="email">
              <Input placeholder={'个人邮箱'} />
            </Form.Item>
          </Col>
          <Button type="primary" htmlType="submit" className="margin-right" onClick={() => this.handleSubmit()}>
            <Trans>Search</Trans>
          </Button>
          <Button className="margin-right" onClick={() => this.handleReset()}>
            <Trans>Reset</Trans>
          </Button>
          <Button type="ghost" onClick={onAdd}>
            <Trans>Create</Trans>
          </Button>
        </Row>
      </Form>
    )
  }
}

Filter.propTypes = {
  onAdd: PropTypes.func,
  filter: PropTypes.object,
  onFilterChange: PropTypes.func
}

export default Filter
