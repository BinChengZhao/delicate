import React, { PureComponent } from 'react'
import { Button, Col, DatePicker, Form, Input, Row, Select } from 'antd'
import { t, Trans } from '@lingui/macro'
import * as u from '../../../utils/data'
import PropTypes from 'prop-types'

class Filter extends PureComponent {
  formRef = React.createRef()

  handleSubmit = () => {
    const { onFilterChange } = this.props
    const values = this.formRef.current.getFieldsValue()
    const datePikerRange = u.getDatePikerRange(values.time_range)
    delete values.time_range
    const initFlitter = this.initFilter()
    onFilterChange({ ...initFlitter, ...values, ...datePikerRange })
  }

  handleReset = () => {
    const fields = this.formRef.current.getFieldsValue()
    for (const item in fields) {
      if ({}.hasOwnProperty.call(fields, item)) fields[item] = undefined
    }
    this.formRef.current.setFieldsValue(fields)
    this.handleSubmit()
  }

  initFilter() {
    return {
      name: null,
      operation_type: null,
      user_name: null,
      time_range: null,
      page: 1,
      per_page: 10
    }
  }

  componentDidMount() {
    this.handleSubmit()
  }

  render() {
    return (
      <Form ref={this.formRef} name="operateLogs-form" initialValues={this.initFilter()}>
        <Row gutter={24}>
          <Col xl={{ span: 4 }} md={{ span: 8 }}>
            <Form.Item name="name">
              <Input placeholder={t`Option Name`} />
            </Form.Item>
          </Col>
          <Col xl={{ span: 4 }} md={{ span: 8 }}>
            <Form.Item name="operation_type">
              <Select allowClear placeholder={t`Operate Type`}>
                <Select.Option value={1}>
                  <Trans>Addition</Trans>
                </Select.Option>
                <Select.Option value={2}>
                  <Trans>Modify</Trans>
                </Select.Option>
                <Select.Option value={3}>
                  <Trans>Delete</Trans>
                </Select.Option>
              </Select>
            </Form.Item>
          </Col>
          <Col xl={{ span: 4 }} md={{ span: 8 }}>
            <Form.Item name="user_name">
              <Input placeholder={t`Username`} />
            </Form.Item>
          </Col>
          <Col xl={{ span: 8 }} md={{ span: 8 }}>
            <Form.Item name="time_range">
              <DatePicker.RangePicker showTime format="YYYY-MM-DD HH:mm:ss" />
            </Form.Item>
          </Col>
          <Button type="primary" htmlType="submit" className="margin-right" onClick={() => this.handleSubmit()}>
            <Trans>Search</Trans>
          </Button>
          <Button className="margin-right" onClick={this.handleReset.bind(this)}>
            <Trans>Reset</Trans>
          </Button>
        </Row>
      </Form>
    )
  }
}

Filter.propTypes = {
  onFilterChange: PropTypes.func
}

export default Filter
