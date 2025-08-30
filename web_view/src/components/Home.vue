<template>
  <div class="home-container">
    <!-- 侧边栏 -->
    <div class="sidebar-wrapper" :class="{ 'sidebar-collapsed': isCollapsed }">
      <Sidebar :collapsed="isCollapsed" />
    </div>

    <!-- 主内容区域 -->
    <div class="main-wrapper" :class="{ 'main-expanded': isCollapsed }">
      <!-- 顶部导航栏 -->
      <header class="header">
        <div class="header-content">
          <div class="header-left">
            <el-button 
              type="text" 
              @click="toggleSidebar"
              class="sidebar-toggle"
            >
              <el-icon><Fold v-if="!isCollapsed" /><Expand v-else /></el-icon>
            </el-button>
            <h1>欢迎使用系统</h1>
          </div>
          <div class="user-actions">
            <span class="welcome-text">欢迎回来！</span>
            <button @click="handleLogout" class="logout-btn">退出登录</button>
          </div>
        </div>
      </header>

        <!-- 主要内容 -->
        <main class="main-content">
          <router-view />
        </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import Sidebar from './Sidebar.vue'
import { Fold, Expand } from '@element-plus/icons-vue'

const router = useRouter()

// 侧边栏折叠状态
const isCollapsed = ref(false)

// 切换侧边栏
const toggleSidebar = () => {
  isCollapsed.value = !isCollapsed.value
}

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
  display: flex;
  background: #f5f5f5;
}

/* 侧边栏容器 */
.sidebar-wrapper {
  width: 180px;
  height: 100vh;
  transition: width 0.28s ease-in-out;
  background-color: #304156;
  flex-shrink: 0;
}

.sidebar-wrapper.sidebar-collapsed {
  width: 50px;
}

/* 主内容区域 */
.main-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  transition: margin-left 0.28s ease-in-out;
}

/* 顶部导航栏 */
.header {
  background: white;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  padding: 0 30px 0 20px;
  width: 100%;
  flex-shrink: 0;
}

.header-content {
  width: 100%;
  max-height: 50px;
  margin: 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 15px;
}

.sidebar-toggle {
  color: #333 !important;
  font-size: 18px !important;
  padding: 8px !important;
  min-height: auto !important;
}

.sidebar-toggle:hover {
  background-color: #f5f5f5 !important;
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
  margin-left: auto;
  flex-shrink: 0;
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

/* 主要内容区域 */
.main-content {
  flex: 1;
  overflow-y: auto;
  padding: 40px 20px;
}

.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
  max-width: 1200px;
  margin: 0 auto;
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

/* 响应式设计 */
@media (max-width: 768px) {
  .sidebar-wrapper {
    position: fixed;
    left: 0;
    top: 0;
    z-index: 1000;
    transform: translateX(-100%);
  }
  
  .sidebar-wrapper:not(.sidebar-collapsed) {
    transform: translateX(0);
  }
  
  .main-wrapper {
    margin-left: 0;
  }
}
</style>
