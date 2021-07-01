import React from 'react'
import PropTypes from 'prop-types'
import { Table, Tag } from 'antd'
import { Color } from 'utils'
import styles from './comments.less'

const status = {
  1: {
    color: Color.green,
    text: 'APPROVED',
  },
  2: {
    color: Color.yellow,
    text: 'PENDING',
  },
  3: {
    color: Color.red,
    text: 'REJECTED',
  },
}

function Comments({ data }) {
  const columns = [
    {
      title: 'avatar',
      dataIndex: 'avatar',
      width: 48,
      className: styles.avatarcolumn,
      render: (text) => (
        <span
          style={{ backgroundImage: `url(${text})` }}
          className={styles.avatar}
        />
      ),
    },
    {
      title: 'content',
      dataIndex: 'content',
      render: (text, it) => (
        <div>
          <h5 className={styles.name}>{it.name}</h5>
          <p className={styles.content}>{it.content}</p>
          <div className={styles.daterow}>
            <Tag color={status[it.status].color}>{status[it.status].text}</Tag>
            <span className={styles.date}>{it.date}</span>
          </div>
        </div>
      ),
    },
  ]
  return (
    <div className={styles.comments}>
      <Table
        pagination={false}
        showHeader={false}
        columns={columns}
        rowKey="avatar"
        dataSource={data.filter((item, key) => key < 3)}
      />
    </div>
  )
}

Comments.propTypes = {
  data: PropTypes.array,
}

export default Comments
