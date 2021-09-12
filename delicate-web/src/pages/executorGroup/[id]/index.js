import React, { Component } from 'react'
import { connect } from 'dva'
import { Page } from '../../../components'
import { Button, Descriptions, Form, Input, message, Modal, Popconfirm, Select, Table } from 'antd'
import styles from '../../taskList/components/List.less'
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
@connect(({ executorGroupModel, loading }) => ({ executorGroupModel, loading }))
class groupDetail extends Component {
  formRef = React.createRef()
  editFormRef = React.createRef()

  constructor(props) {
    super(props)
    this.state = {
      executorIds: [],
      inner: { name: '', description: '' },
      bindings: [],
      bindList: [],
      visible: false,
      modalProps: this.updateModalProps
    }
  }

  getDetail() {
    const { dispatch, match } = this.props
    const groupId = match.params.id || 0
    dispatch({ type: 'executorGroupModel/groupUsedExecutor', payload: { executor_group_id: parseInt(groupId) } }).then(
      (ret) => this.setState({ ...ret })
    )
  }

  getGroupBindList() {
    const { dispatch } = this.props
    dispatch({ type: `executorGroupModel/groupBindList` }).then((bindList) => this.setState({ bindList }))
  }

  confirm(bingdingId) {
    const { dispatch } = this.props
    dispatch({
      type: 'executorGroupModel/groupDeleteExecutor',
      payload: { executor_processor_bind_id: parseInt(bingdingId) }
    }).then(() => this.getDetail())
  }

  cancel() {
    message.info(t`Cancel Delete`)
  }

  handleOk = () => {
    const { dispatch, match } = this.props
    const { modalProps } = this.state

    const groupId = match.params.id || 0
    this.formRef.current
      .validateFields()
      .then((values) => {
        const bindParams = {
          group_id: parseInt(groupId),
          name: this.state.inner.name + t`Binding Node`,
          weight: 0,
          ...values
        }
        if (modalProps.type === 'create') {
          dispatch({ type: `executorGroupModel/onGroupBindExecutor`, payload: bindParams }).then(() => this.getDetail())
        } else {
          bindParams.id = this.state.bingdingId
          dispatch({ type: `executorGroupModel/onGroupUpdateExecutor`, payload: bindParams }).then(() =>
            this.getDetail()
          )
        }
        if (this.state.visible === true) this.toggleBindModal()
      })
      .catch((errorInfo) => {
        console.log(errorInfo)
      })
  }

  toggleBindModal() {
    this.setState({ visible: !this.state.visible })
  }

  get createModalProps() {
    return {
      type: 'create',
      title: t`Create`,
      mode: 'multiple',
      name: 'executor_ids'
    }
  }

  get updateModalProps() {
    return {
      type: 'update',
      title: t`Update`,
      mode: null,
      name: 'executor_id'
    }
  }

  componentDidMount() {
    this.getDetail()
    this.getGroupBindList()
  }

  render() {
    const { inner, bindings, bindList, visible, modalProps } = this.state

    const columns = [
      {
        title: t`Binding Node`,
        dataIndex: 'bingding_name',
        key: 'name',
        width: 120,
        fixed: 'left',
        render: (text, row) => <a>{text}</a>
      },
      {
        title: t`Node Id`,
        dataIndex: 'executor_id',
        width: 200,
        key: 'executor_id'
      },
      {
        title: t`Node Name`,
        dataIndex: 'executor_name',
        key: 'executor_name',
        width: 150
      },
      {
        title: t`Host`,
        dataIndex: 'host',
        key: 'host'
      },
      {
        title: t`Machine Id`,
        dataIndex: 'machine_id',
        key: 'machine_id'
      },
      {
        title: t`Operation`,
        key: 'operating',
        render: (text, row) => {
          return (
            <div>
              <a
                type={'link'}
                onClick={() =>
                  this.setState({ modalProps: this.updateModalProps, bingdingId: row.bingding_id }, () =>
                    this.toggleBindModal()
                  )
                }
              >
                {t`Update`}
              </a>
              <Popconfirm
                title={t`Are you sure you want to unbind the node ID ${row.executor_id}?`}
                onConfirm={() => this.confirm(row.bingding_id)}
                onCancel={() => this.cancel()}
                okText="Yes"
                cancelText="No"
              >
                <Button type={'link'} danger>
                  {t`Unbind`}
                </Button>
              </Popconfirm>
            </div>
          )
        }
      }
    ]

    return (
      <Page inner>
        <Descriptions title={inner.name}>
          <Descriptions.Item label={t`Description`} span={3}>
            {inner.description}
          </Descriptions.Item>
        </Descriptions>

        <div style={{ float: 'right', padding: '20px 0' }}>
          <Button
            type="primary"
            htmlType="submit"
            className="margin-right"
            onClick={() => this.setState({ modalProps: this.createModalProps }, () => this.toggleBindModal())}
          >
            {t`Create`}
          </Button>
        </div>

        <Table
          className={styles.table}
          pagination={false}
          columns={columns}
          dataSource={bindings}
          simple
          rowKey={(record) => record.id}
        />

        <Modal title={modalProps.title} visible={visible} onOk={this.handleOk} onCancel={() => this.toggleBindModal()}>
          <Form ref={this.formRef} name="control-ref" layout="horizontal">
            {modalProps.type === 'update' ? (
              <FormItem name="name" label={t`Node Name`} hasFeedback {...formItemLayout}>
                <Input />
              </FormItem>
            ) : null}
            <FormItem name={modalProps.name} label={t`Binding Node`} hasFeedback {...formItemLayout}>
              <Select mode={modalProps.mode} rules={[{ required: true }]} onFocus={() => this.getGroupBindList()}>
                {/* eslint-disable-next-line array-callback-return */}
                {bindList.map((point, i) => {
                  return (
                    <Select.Option key={parseInt(point.id)} value={parseInt(point.id)}>
                      {point.title}
                    </Select.Option>
                  )
                })}
              </Select>
            </FormItem>
          </Form>
        </Modal>
      </Page>
    )
  }
}

export default groupDetail
