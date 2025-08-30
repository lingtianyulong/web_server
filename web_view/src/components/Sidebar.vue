<template>
  <div class="sidebar-container">
    <el-menu
      :default-active="activeMenu"
      class="sidebar-menu"
      :collapse="isCollapsed"
      :collapse-transition="true"
      background-color="#304156"
      text-color="#bfcbd9"
      active-text-color="#409EFF"
      @select="handleMenuSelect"
    >
      <!-- 菜单项 -->
      <el-menu-item index="/home/dashboard">
        <el-icon><Odometer /></el-icon>
        <template #title>数据统计</template>
      </el-menu-item>

      <el-menu-item index="/home/users">
        <el-icon><User /></el-icon>
        <template #title>用户管理</template>
      </el-menu-item>

      <el-sub-menu index="system">
        <template #title>
          <el-icon><Setting /></el-icon>
          <span>系统管理</span>
        </template>
        <el-menu-item index="/system/config">
          <el-icon><Tools /></el-icon>
          <template #title>系统配置</template>
        </el-menu-item>
        <el-menu-item index="/system/logs">
          <el-icon><Document /></el-icon>
          <template #title>日志管理</template>
        </el-menu-item>
      </el-sub-menu>

      <el-menu-item index="/reports">
        <el-icon><DataAnalysis /></el-icon>
        <template #title>报表中心</template>
      </el-menu-item>

      <el-menu-item index="/settings">
        <el-icon><Tools /></el-icon>
        <template #title>个人设置</template>
      </el-menu-item>
    </el-menu>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import {
  Odometer,
  User,
  Setting,
  Tools,
  Document,
  DataAnalysis
} from '@element-plus/icons-vue'

// Props
interface Props {
  collapsed: boolean
}

const props = defineProps<Props>()

// Router
const route = useRoute()
const router = useRouter()

// 响应式数据
const isCollapsed = computed(() => props.collapsed)
const activeMenu = ref(route.path)

// 事件处理
const handleMenuSelect = (index: string) => {
  activeMenu.value = index
  
  // 跳转到对应的路由
  if (router.currentRoute.value.path !== index) {
    router.push(index)
  }
}

// 监听路由变化
watch(() => route.path, (newPath) => {
  activeMenu.value = newPath
})
</script>

<style scoped>
.sidebar-container {
  height: 100%;
  background-color: #304156;
  transition: width 0.28s ease-in-out;
}

.sidebar-menu {
  border: none;
  height: 100%;
  width: 100%;
}

/* 折叠状态下的菜单样式 */
.sidebar-menu.el-menu--collapse {
  width: 50px;
}

/* 自定义菜单项样式 */
:deep(.el-menu-item) {
  height: 50px;
  line-height: 50px;
  margin: 0;
}

:deep(.el-sub-menu__title) {
  height: 50px;
  line-height: 50px;
}

/* 激活状态的菜单项 */
:deep(.el-menu-item.is-active) {
  background-color: #409EFF !important;
  color: #fff !important;
}

/* 鼠标悬停效果 */
:deep(.el-menu-item:hover) {
  background-color: #48576a !important;
  color: #fff !important;
}

:deep(.el-sub-menu__title:hover) {
  background-color: #48576a !important;
  color: #fff !important;
}

/* 子菜单样式 */
:deep(.el-menu--popup) {
  background-color: #304156 !important;
}

:deep(.el-menu--popup .el-menu-item) {
  background-color: #1f2d3d !important;
}

:deep(.el-menu--popup .el-menu-item:hover) {
  background-color: #48576a !important;
}
</style>
