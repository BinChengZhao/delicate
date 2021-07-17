import React, { Component } from 'react'
import PropTypes from 'prop-types'
import { t, Trans } from '@lingui/macro'
import { Button, Col, Form, Input, Row, Select } from 'antd'

class Filter extends Component {
  formRef = React.createRef()

  initFilter() {
    return {
      name: null,
      description: null,
      command: null,
      id: null,
      bind_id: null,
      status: null,
      tag: null,
      per_page: 10,
      page: 1
    }
  }

  handleSubmit() {
    const { query } = this.props
    const values = this.formRef.current.getFieldsValue()
    for (const i in values) {
      values[i] = values[i] === '' ? null : values[i]
    }
    const initFlitter = this.initFilter()
    query({ ...initFlitter, ...values })
  }

  handleReset() {
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
    const { openModal } = this.props

    return (
      <Form ref={this.formRef} name="control-ref" initialValues={this.initFilter()}>
        <Row gutter={24}>
          <Col xl={{ span: 4 }} md={{ span: 8 }}>
            <Form.Item name="name">
              <Input placeholder={t`Task Name`} />
            </Form.Item>
          </Col>
          <Col xl={{ span: 4 }} md={{ span: 8 }}>
            <Form.Item name="bind_id">
              <Input placeholder={t`Bind Id`} />
            </Form.Item>
          </Col>
          <Col xl={{ span: 4 }} md={{ span: 8 }}>
            <Form.Item name="status">
              <Select allowClear placeholder={'状态'}>
                <Select.Option value={2}>启用</Select.Option>
                <Select.Option value={1}>未启用</Select.Option>
              </Select>
            </Form.Item>
          </Col>
          <Col xl={{ span: 4 }} md={{ span: 8 }}>
            <Form.Item name="tag">
              <Input placeholder={t`Tag`} />
            </Form.Item>
          </Col>
          <Button type="primary" htmlType="submit" className="margin-right" onClick={() => this.handleSubmit()}>
            <Trans>Search</Trans>
          </Button>
          <Button className="margin-right" onClick={this.handleReset.bind(this)}>
            <Trans>Reset</Trans>
          </Button>
          <Button type="ghost" onClick={openModal}>
            <Trans>Create</Trans>
          </Button>
        </Row>
      </Form>
    )
  }
}

Filter.propTypes = {
  openModal: PropTypes.func,
  filter: PropTypes.object,
  query: PropTypes.func
}

export default Filter
