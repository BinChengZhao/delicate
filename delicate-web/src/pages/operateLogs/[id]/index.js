import React, { Component } from 'react'
import { connect } from 'dva'
import { Page } from '../../../components'
import ReactJson from 'react-json-view'

const NAMESPACE = 'operateLogs'

@connect(({ operateLogs }) => ({ operateLogs }))
class LogDetails extends Component {
  constructor(props) {
    super(props)
    this.state = {
      json: {}
    }
  }

  componentDidMount() {
    const id = this.props.match.params.id
    const { dispatch } = this.props
    const payload = { operation_log_id: parseInt(id) }
    dispatch({ type: `${NAMESPACE}/getOperateLogDetail`, payload }).then((ret) => {
      const json = []
      // eslint-disable-next-line array-callback-return
      ret.data.map((item) => {
        json.push(JSON.parse(item.values))
      })
      this.setState({ json })
    })
  }

  render() {
    const { json } = this.state
    return (
      <Page inner>
        <ReactJson name={null} displayDataTypes={false} src={json} />
      </Page>
    )
  }
}

export default LogDetails
