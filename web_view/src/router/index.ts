import { createRouter, createWebHistory } from 'vue-router'
import Login from '../components/Login.vue'
import Home from '../components/Home.vue'

const routes = [
  {
    path: '/',
    redirect: '/login'
  },
  {
    path: '/login',
    name: 'Login',
    component: Login,
    meta: {
      title: '登录'
    }
  },
  {
    path: '/home',
    name: 'Home',
    component: Home,
    meta: {
      title: '首页',
      requiresAuth: true
    }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// 路由守卫 - 检查认证状态
router.beforeEach((to, _from, next) => {
  // 设置页面标题
  if (to.meta.title) {
    document.title = `${to.meta.title} - 系统管理`
  }
  
  // 检查是否需要认证
  if (to.meta.requiresAuth) {
    // 这里可以添加实际的认证检查逻辑
    // 目前简单跳过认证检查
    next()
  } else {
    next()
  }
})

export default router
