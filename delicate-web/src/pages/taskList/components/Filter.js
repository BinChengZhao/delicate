import React, { Component } from 'react'
import PropTypes from 'prop-types'
import moment from 'moment'
import { t, Trans } from '@lingui/macro'
import { Button, Col, Form, Input, Row } from 'antd'

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
    const { onFilterChange } = this.props
    const values = this.formRef.current.getFieldsValue()
    for (const i in values) {
      values[i] = values[i] === '' ? null : values[i]
    }
    const initFlitter = this.initFilter()
    onFilterChange({ ...initFlitter, ...values })
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
    const { onAdd, filter } = this.props
    const initialCreateTime = []
    if (filter.createTime && filter.createTime[0]) {
      initialCreateTime[0] = moment(filter.createTime[0])
    }
    if (filter.createTime && filter.createTime[1]) {
      initialCreateTime[1] = moment(filter.createTime[1])
    }

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
              <Input placeholder={t`Status`} />
            </Form.Item>
          </Col>
          <Col xl={{ span: 4 }} md={{ span: 8 }}>
            <Form.Item name="tag">
              <Input placeholder={t`Tag`} />
            </Form.Item>
          </Col>
          <Button type="primary" htmlType="submit" className="margin-right" onClick={this.handleSubmit.bind(this)}>
            <Trans>Search</Trans>
          </Button>
          <Button className="margin-right" onClick={this.handleReset.bind(this)}>
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
