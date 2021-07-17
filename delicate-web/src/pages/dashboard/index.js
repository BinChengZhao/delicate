import React, { PureComponent } from 'react'
import * as echarts from 'echarts'

import PropTypes from 'prop-types'
import { connect } from 'umi'
import { Page } from 'components'
import styles from './index.less'
import { Card } from 'antd'

@connect(({ dashboard, loading }) => ({
  dashboard,
  loading
}))
class Dashboard extends PureComponent {
  initEChart() {
    const { dashboard } = this.props
    const { taskStatusEChart } = dashboard
    const legend = []

    const hourContainer = taskStatusEChart.hours_range || []
    const series = []

    for (const taskName in taskStatusEChart) {
      if (taskStatusEChart.hasOwnProperty(taskName) && taskName !== 'hours_range') {
        series.push({
          name: taskName,
          type: 'line',
          stack: '总量',
          label: {
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

    series.filter((e) => {
      legend.push(e.name)
    })

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

  shouldComponentUpdate(nextProps, nextState, nextContext) {
    const { dashboard } = nextProps
    const { taskStatusEChart } = dashboard
    taskStatusEChart && this.initEChart()
    return true
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
