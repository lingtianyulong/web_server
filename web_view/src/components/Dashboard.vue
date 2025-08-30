<template>
  <div class="dashboard-container">
    <div>
        <h3>数据统计</h3>
    </div>
    <!-- 统计卡片 -->
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon user-icon">
          <div class="icon">👥</div>
        </div>
        <div class="stat-content">
          <h3>{{ stats.totalUsers.toLocaleString() }}</h3>
          <p>总用户数</p>
          <span class="stat-change positive">+{{ stats.userGrowth }}%</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon order-icon">
          <div class="icon">🛒</div>
        </div>
        <div class="stat-content">
          <h3>{{ stats.totalOrders.toLocaleString() }}</h3>
          <p>总订单数</p>
          <span class="stat-change positive">+{{ stats.orderGrowth }}%</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon revenue-icon">
          <div class="icon">💰</div>
        </div>
        <div class="stat-content">
          <h3>¥{{ stats.totalRevenue.toLocaleString() }}</h3>
          <p>总收入</p>
          <span class="stat-change positive">+{{ stats.revenueGrowth }}%</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon visit-icon">
          <div class="icon">👁️</div>
        </div>
        <div class="stat-content">
          <h3>{{ stats.totalVisits.toLocaleString() }}</h3>
          <p>总访问量</p>
          <span class="stat-change negative">{{ stats.visitGrowth }}%</span>
        </div>
      </div>
    </div>

    <!-- 图表区域 -->
    <div class="charts-grid">
      <!-- 访问趋势图 -->
      <div class="chart-card">
        <div class="chart-header">
          <h3>访问趋势</h3>
          <div class="button-group">
            <button 
              :class="['period-btn', { active: chartPeriod === '7d' }]" 
              @click="chartPeriod = '7d'"
            >
              7天
            </button>
            <button 
              :class="['period-btn', { active: chartPeriod === '30d' }]" 
              @click="chartPeriod = '30d'"
            >
              30天
            </button>
            <button 
              :class="['period-btn', { active: chartPeriod === '90d' }]" 
              @click="chartPeriod = '90d'"
            >
              90天
            </button>
          </div>
        </div>
        <div class="chart-placeholder">
          <div class="mock-chart">
            <div class="chart-bars">
              <div v-for="i in 12" :key="i" class="chart-bar" :style="{ height: Math.random() * 80 + 20 + '%' }"></div>
            </div>
            <p>访问量趋势图 (模拟数据)</p>
          </div>
        </div>
      </div>

      <!-- 用户分布图 -->
      <div class="chart-card">
        <div class="chart-header">
          <h3>用户地域分布</h3>
        </div>
        <div class="chart-placeholder">
          <div class="mock-pie-chart">
            <div class="pie-legend">
              <div class="legend-item">
                <span class="legend-color" style="background: #409EFF;"></span>
                <span>北京 (35%)</span>
              </div>
              <div class="legend-item">
                <span class="legend-color" style="background: #67C23A;"></span>
                <span>上海 (28%)</span>
              </div>
              <div class="legend-item">
                <span class="legend-color" style="background: #E6A23C;"></span>
                <span>广州 (20%)</span>
              </div>
              <div class="legend-item">
                <span class="legend-color" style="background: #F56C6C;"></span>
                <span>其他 (17%)</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 最近活动 -->
    <div class="activity-section">
      <div class="section-header">
        <h3>最近活动</h3>
        <button class="view-all-btn">查看全部</button>
      </div>
      <div class="activity-list">
        <div v-for="activity in recentActivities" :key="activity.id" class="activity-item">
          <div class="activity-avatar">
            <div class="avatar-icon">👤</div>
          </div>
          <div class="activity-content">
            <p class="activity-text">{{ activity.user }} {{ activity.action }}</p>
            <span class="activity-time">{{ activity.time }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'

// 统计数据
const stats = reactive({
  totalUsers: 15420,
  userGrowth: 12.3,
  totalOrders: 8567,
  orderGrowth: 8.7,
  totalRevenue: 125890,
  revenueGrowth: 15.2,
  totalVisits: 34521,
  visitGrowth: -2.1
})

// 图表时间周期
const chartPeriod = ref('7d')

// 最近活动数据
const recentActivities = ref([
  {
    id: 1,
    user: '张三',
    action: '登录了系统',
    time: '2分钟前'
  },
  {
    id: 2,
    user: '李四',
    action: '创建了新订单',
    time: '5分钟前'
  },
  {
    id: 3,
    user: '王五',
    action: '更新了个人信息',
    time: '10分钟前'
  },
  {
    id: 4,
    user: '赵六',
    action: '完成了支付',
    time: '15分钟前'
  },
  {
    id: 5,
    user: '钱七',
    action: '提交了反馈',
    time: '20分钟前'
  }
])
</script>

<style scoped>
.dashboard-container {
  background: #f5f5f5;
  min-height: 100%;
}

.breadcrumb {
  background: white;
  padding: 12px 24px;
  margin: 0;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  border-bottom: 1px solid #ebeef5;
}

/* 统计卡片网格 */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 24px;
  margin: 24px 24px 24px 24px;
}

.stat-card {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.stat-icon .icon {
  font-size: 24px;
}

.user-icon {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.order-icon {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.revenue-icon {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
}

.visit-icon {
  background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
}

.stat-content h3 {
  margin: 0 0 4px 0;
  color: #303133;
  font-size: 24px;
  font-weight: 600;
}

.stat-content p {
  margin: 0 0 8px 0;
  color: #606266;
  font-size: 14px;
}

.stat-change {
  font-size: 12px;
  font-weight: 500;
}

.stat-change.positive {
  color: #67C23A;
}

.stat-change.negative {
  color: #F56C6C;
}

/* 图表网格 */
.charts-grid {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 24px;
  margin: 0 24px 24px 24px;
}

.chart-card {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
}

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.chart-header h3 {
  margin: 0;
  color: #303133;
  font-size: 18px;
  font-weight: 600;
}

/* 按钮组样式 */
.button-group {
  display: flex;
  border-radius: 4px;
  overflow: hidden;
  border: 1px solid #dcdfe6;
}

.period-btn {
  padding: 6px 12px;
  border: none;
  background: white;
  color: #606266;
  cursor: pointer;
  font-size: 12px;
  transition: all 0.3s;
  border-right: 1px solid #dcdfe6;
}

.period-btn:last-child {
  border-right: none;
}

.period-btn:hover {
  background: #f5f7fa;
  color: #409EFF;
}

.period-btn.active {
  background: #409EFF;
  color: white;
}

.chart-placeholder {
  height: 300px;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 模拟图表样式 */
.mock-chart {
  text-align: center;
  width: 100%;
}

.chart-bars {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  height: 200px;
  margin-bottom: 20px;
  padding: 0 20px;
}

.chart-bar {
  width: 20px;
  background: linear-gradient(180deg, #409EFF 0%, #67C23A 100%);
  border-radius: 2px 2px 0 0;
  min-height: 20px;
}

.mock-pie-chart {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
}

.pie-legend {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: #606266;
}

.legend-color {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

/* 活动区域 */
.activity-section {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  margin: 0 24px 24px 24px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.section-header h3 {
  margin: 0;
  color: #303133;
  font-size: 18px;
  font-weight: 600;
}

.view-all-btn {
  background: none;
  border: none;
  color: #409EFF;
  cursor: pointer;
  font-size: 14px;
  transition: color 0.3s;
}

.view-all-btn:hover {
  color: #66b1ff;
  text-decoration: underline;
}

.activity-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.activity-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 0;
  border-bottom: 1px solid #f0f0f0;
}

.activity-item:last-child {
  border-bottom: none;
}

.activity-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: #409EFF;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
}

.avatar-icon {
  font-size: 16px;
}

.activity-content {
  flex: 1;
}

.activity-text {
  margin: 0 0 4px 0;
  color: #303133;
  font-size: 14px;
}

.activity-time {
  color: #909399;
  font-size: 12px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .breadcrumb {
    padding: 12px 16px;
  }
  
  .stats-grid,
  .charts-grid,
  .activity-section {
    margin-left: 12px;
    margin-right: 12px;
  }
  
  .charts-grid {
    grid-template-columns: 1fr;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
  }
}
</style>