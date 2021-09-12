import React, { Fragment, PureComponent } from 'react'
import PropTypes from 'prop-types'
import { connect } from 'umi'
import { Button, Form, Input, Row } from 'antd'
import { GlobalFooter } from 'components'
import { GithubOutlined, KeyOutlined, UserOutlined } from '@ant-design/icons'
import { t, Trans } from '@lingui/macro'

import { setLocale } from 'utils'
import config from 'utils/config'

import styles from './index.less'

const FormItem = Form.Item

@connect(({ loading, dispatch }) => ({ loading, dispatch }))
class Login extends PureComponent {
  render() {
    const { dispatch, loading } = this.props

    const handleOk = (values) => {
      dispatch({ type: 'login/login', payload: { ...values, login_type: 3 } })
    }
    let footerLinks = [
      {
        key: 'github',
        title: <GithubOutlined />,
        href: 'https://github.com/BinChengZhao/delicate',
        blankTarget: true
      }
    ]

    if (config.i18n) {
      footerLinks = footerLinks.concat(
        config.i18n.languages.map((item) => ({
          key: item.key,
          title: <span onClick={setLocale.bind(null, item.key)}>{item.title}</span>
        }))
      )
    }

    return (
      <Fragment>
        <div className={styles.form}>
          <div className={styles.logo}>
            <img alt="logo" src={config.logoPath} />
            <span>{config.siteName}</span>
          </div>
          <Form onFinish={handleOk}>
            <FormItem name="account" rules={[{ required: true }]} hasFeedback>
              <Input prefix={<UserOutlined />} placeholder={t`Username`} />
            </FormItem>
            <FormItem name="password" rules={[{ required: true }]} hasFeedback>
              <Input prefix={<KeyOutlined />} type="password" placeholder={t`Password`} />
            </FormItem>
            <Row>
              <Button type="primary" htmlType="submit" loading={loading.effects.login}>
                <Trans>Sign in</Trans>
              </Button>
            </Row>
          </Form>
        </div>
        <div className={styles.footer}>
          <GlobalFooter links={footerLinks} copyright={config.copyright} />
        </div>
      </Fragment>
    )
  }
}

Login.propTypes = {
  form: PropTypes.object,
  dispatch: PropTypes.func,
  loading: PropTypes.object
}

export default Login
