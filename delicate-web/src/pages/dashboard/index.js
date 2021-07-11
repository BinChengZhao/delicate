import React, { PureComponent } from 'react'
import * as echarts from 'echarts'

import PropTypes, { instanceOf } from 'prop-types'
import { connect } from 'umi'
import { Page } from 'components'
import styles from './index.less'
import { Card } from 'antd'

@connect(({ app, dashboard, loading }) => ({
  dashboard,
  loading
}))
class Dashboard extends PureComponent {
  componentDidMount() {
    const { dashboard } = this.props
    const { taskStatusEChart } = dashboard
    console.log(taskStatusEChart)
    const legend = Object.keys(taskStatusEChart)

    const hourContainer = []
    for (let i = 0; i < 24; i++) {
      hourContainer.push(i + '时')
    }

    const series = []

    for (const taskName in taskStatusEChart) {
      if (taskStatusEChart.hasOwnProperty(taskName)) {
        series.push({
          name: taskName,
          type: 'line',
          stack: '总量',
          label: {
            show: true,
            position: 'top'
          },
          areaStyle: {},
          emphasis: {
            focus: 'series'
          },
          data: taskStatusEChart[taskName]
        })
      }
    }

    const chartDom = document.getElementById('main')
    const myChart = echarts.init(chartDom)
    let option
    option = {
      title: {
        text: '任务状态聚合(最近24小时)'
      },
      tooltip: {
        trigger: 'axis',
        axisPointer: {
          type: 'cross',
          label: {
            backgroundColor: '#6a7985'
          }
        }
      },
      legend: {
        // todo legend
        data: legend
      },
      toolbox: {
        feature: {
          saveAsImage: {}
        }
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true
      },
      xAxis: [
        {
          type: 'category',
          boundaryGap: false,
          data: hourContainer
        }
      ],
      yAxis: [
        {
          type: 'value'
        }
      ],
      series: series
    }

    option && myChart.setOption(option)
  }

  render() {
    return (
      <Page className={styles.dashboard}>
        <Card>
          <div id="main" style={{ width: '99%', height: 500 }} />
        </Card>
      </Page>
    )
  }
}

Dashboard.propTypes = {
  dashboard: PropTypes.object,
  loading: PropTypes.object
}

export default Dashboard
