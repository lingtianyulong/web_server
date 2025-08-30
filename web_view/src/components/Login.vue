<template>
  <div class="login-container">
    <div class="login-card">
      <div class="login-header">
        <h2>用户登录</h2>
        <p>请输入您的账户信息</p>
      </div>
      
      <form @submit.prevent="handleLogin" class="login-form">
        <div class="form-group">
          <label for="username">用户名</label>
          <input
            id="username"
            v-model="form.user_name"
            type="text"
            placeholder="请输入用户名"
            required
          />
        </div>
        
        <div class="form-group">
          <label for="password">密码</label>
          <input
            id="password"
            v-model="form.password"
            type="password"
            placeholder="请输入密码"
            required
          />
        </div>
        
        <!-- <div class="form-group checkbox-group">
          <input
            id="remember"
            v-model="form.remember"
            type="checkbox"
          />
          <label for="remember">记住我</label>
        </div> -->
        
        <button type="submit" class="login-btn" :disabled="loading">
          {{ loading ? '登录中...' : '登录' }}
        </button>
      </form>
      
      <div class="login-footer">
        <a href="#" class="forgot-link">忘记密码？</a>
        <span>|</span>
        <a href="#" class="register-link">注册账户</a>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { authService } from '../services/auth'
import type { LoginRequest } from '../services/auth'

const router = useRouter()

const form = reactive({
  user_name: '',
  password: '',
  // remember: false
})

const loading = ref(false)

const handleLogin = async () => {
  // if (!form.user_name || !form.password) {
  //   ElMessage.warning('请填写完整的登录信息')
  //   return
  // }
  
  loading.value = true
  
  try {
    // 构建登录请求数据
    const loginData: LoginRequest = {
      user_name: form.user_name,
      password: form.password,
      // remember: form.remember
    }

    // 调用封装后的登录服务
    const response = await authService.login(loginData)
    
    // 检查登录结果
    if (response.code === 200) {
      // 保存用户信息
      if (response.data?.user) {
        authService.setUserInfo(response.data.user)
      }
      
      // 登录成功提示
      ElMessage.success('登录成功！')
      
      // 跳转到主页
      router.push('/home/dashboard')
    } else {
      throw new Error(response.message || '登录失败')
    }
  } catch (error: unknown) {
    console.error('Login error:', error)
    
    let errorMessage = '登录失败，请稍后重试'
    
    // 处理不同类型的错误
    if (error && typeof error === 'object' && 'status' in error) {
      // 处理 HTTP 错误
      const httpError = error as { status: number; message: string }
      switch (httpError.status) {
        case 401:
          errorMessage = '用户名或密码错误'
          break
        case 403:
          errorMessage = '账户已被禁用'
          break
        case 404:
          errorMessage = '用户不存在'
          break
        case 500:
          errorMessage = '服务器错误，请稍后重试'
          break
        default:
          errorMessage = httpError.message || '登录失败'
      }
    } else if (error instanceof Error) {
      if (error.message.includes('timeout')) {
        errorMessage = '请求超时，请检查网络连接'
      } else if (error.message.includes('fetch')) {
        errorMessage = '网络连接失败，请检查服务器状态'
      } else {
        errorMessage = error.message
      }
    }
    
    ElMessage.error(errorMessage)
  } finally {
    loading.value = false
  }
}

// 页面加载时检查是否已登录
const checkAuthStatus = () => {
  if (authService.isAuthenticated()) {
    // 如果已经登录，直接跳转到主页
    router.push('/home/dashboard')
  }
}

// 组件挂载时检查登录状态
checkAuthStatus()
</script>

<style scoped>
.login-container {
  height: 100vh;
  width: 100vw;
  position: fixed;
  top: 0;
  left: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 20px;
  margin: 0;
  overflow: auto;
}

.login-card {
  background: white;
  border-radius: 12px;
  box-shadow: 0 15px 35px rgba(0, 0, 0, 0.1);
  padding: 40px;
  width: 100%;
  max-width: 400px;
  animation: fadeInUp 0.6s ease-out;
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.login-header {
  text-align: center;
  margin-bottom: 30px;
}

.login-header h2 {
  color: #333;
  margin: 0 0 8px 0;
  font-size: 28px;
  font-weight: 600;
}

.login-header p {
  color: #666;
  margin: 0;
  font-size: 14px;
}

.login-form {
  margin-bottom: 20px;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  color: #333;
  font-weight: 500;
  font-size: 14px;
}

.form-group input[type="text"],
.form-group input[type="password"] {
  width: 100%;
  padding: 12px 16px;
  border: 2px solid #e1e1e1;
  border-radius: 8px;
  font-size: 16px;
  transition: border-color 0.3s ease;
  box-sizing: border-box;
}

.form-group input[type="text"]:focus,
.form-group input[type="password"]:focus {
  outline: none;
  border-color: #667eea;
}

.checkbox-group {
  display: flex;
  align-items: center;
  margin-bottom: 25px;
}

.checkbox-group input[type="checkbox"] {
  margin-right: 8px;
  width: auto;
}

.checkbox-group label {
  margin-bottom: 0;
  color: #666;
  font-size: 14px;
  cursor: pointer;
}

.login-btn {
  width: 100%;
  padding: 14px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.login-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(102, 126, 234, 0.3);
}

.login-btn:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

.login-footer {
  text-align: center;
  margin-top: 20px;
}

.login-footer a {
  color: #667eea;
  text-decoration: none;
  font-size: 14px;
  transition: color 0.3s ease;
}

.login-footer a:hover {
  color: #764ba2;
  text-decoration: underline;
}

.login-footer span {
  margin: 0 10px;
  color: #ccc;
}
</style>
