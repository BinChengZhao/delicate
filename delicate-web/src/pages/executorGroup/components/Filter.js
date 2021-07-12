import React, { PureComponent } from 'react'
import { Button, Col, Form, Input, Row } from 'antd'
import { t, Trans } from '@lingui/macro'
import PropTypes from 'prop-types'

class ExeutorGroupFilter extends PureComponent {
  formRef = React.createRef()

  initFilter() {
    return {
      id: null,
      name: null,
      tag: null,
      description: null,
      per_page: 10,
      page: 1
    }
  }

  handleReset() {
    const fields = this.formRef.current.getFieldsValue()
    for (const item in fields) {
      if ({}.hasOwnProperty.call(fields, item)) fields[item] = undefined
    }
    this.formRef.current.setFieldsValue(fields)
    this.handleSubmit()
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

  componentDidMount() {
    this.handleSubmit()
  }

  render() {
    const { openModal } = this.props
    return (
      <Form ref={this.formRef} name="control-ref" initialValues={this.initFilter()}>
        <Row gutter={24}>
          <Col xl={{ span: 4 }} md={{ span: 8 }}>
            <Form.Item name="tag">
              <Input placeholder={t`Tag`} />
            </Form.Item>
          </Col>
          <Col xl={{ span: 4 }} md={{ span: 8 }}>
            <Form.Item name="name">
              <Input placeholder="节点名称" />
            </Form.Item>
          </Col>
          <Button type="primary" htmlType="submit" className="margin-right" onClick={() => this.handleSubmit()}>
            <Trans>Search</Trans>
          </Button>
          <Button className="margin-right" onClick={() => this.handleReset()}>
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

ExeutorGroupFilter.propTypes = {
  openModal: PropTypes.func,
  query: PropTypes.func
}

export default ExeutorGroupFilter
