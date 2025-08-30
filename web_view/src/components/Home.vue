<template>
  <div class="home-container">
    <header class="header">
      <div class="header-content">
        <h1>欢迎使用系统</h1>
        <div class="user-actions">
          <span class="welcome-text">欢迎回来！</span>
          <button @click="handleLogout" class="logout-btn">退出登录</button>
        </div>
      </div>
    </header>

    <main class="main-content">
      <div class="dashboard-grid">
        <div class="card">
          <div class="card-icon">📊</div>
          <h3>数据统计</h3>
          <p>查看系统数据统计信息</p>
        </div>

        <div class="card">
          <div class="card-icon">👥</div>
          <h3>用户管理</h3>
          <p>管理系统用户信息</p>
        </div>

        <div class="card">
          <div class="card-icon">⚙️</div>
          <h3>系统设置</h3>
          <p>配置系统参数</p>
        </div>

        <div class="card">
          <div class="card-icon">📝</div>
          <h3>日志管理</h3>
          <p>查看系统运行日志</p>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

const handleLogout = () => {
  if (confirm('确定要退出登录吗？')) {
    // 这里可以添加清除用户信息的逻辑
    router.push('/login')
  }
}
</script>

<style scoped>
.home-container {
  height: 100vh;
  width: 100vw;
  background: #f5f5f5;
  overflow-y: auto;
}

.header {
  background: white;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  padding: 0 20px;
}

.header-content {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 0;
}

.header h1 {
  margin: 0;
  color: #333;
  font-size: 24px;
  font-weight: 600;
}

.user-actions {
  display: flex;
  align-items: center;
  gap: 15px;
}

.welcome-text {
  color: #666;
  font-size: 14px;
}

.logout-btn {
  padding: 8px 16px;
  background: #ff4757;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  transition: background 0.3s ease;
}

.logout-btn:hover {
  background: #ff3838;
}

.main-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 40px 20px;
}

.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
}

.card {
  background: white;
  border-radius: 12px;
  padding: 30px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.08);
  transition: transform 0.3s ease, box-shadow 0.3s ease;
  cursor: pointer;
  text-align: center;
}

.card:hover {
  transform: translateY(-5px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

.card-icon {
  font-size: 48px;
  margin-bottom: 15px;
}

.card h3 {
  margin: 0 0 10px 0;
  color: #333;
  font-size: 18px;
  font-weight: 600;
}

.card p {
  margin: 0;
  color: #666;
  font-size: 14px;
  line-height: 1.5;
}
</style>
