import { API_BASE_URL, TIMEOUT } from './env'

// API 配置文件
export const API_CONFIG = {
  // 服务器配置（从环境配置中获取）
  BASE_URL: API_BASE_URL,
  
  // API 路径配置
  ENDPOINTS: {
    // 用户相关接口
    USER: {
      LOGIN: '/user/login',
      LOGOUT: '/user/logout',
      REGISTER: '/user/register',
      PROFILE: '/user/profile',
      UPDATE_PROFILE: '/user/profile',
      CHANGE_PASSWORD: '/user/change-password',
      LIST: '/user/list',
      CREATE: '/user/create',
      UPDATE: '/user/update',
      DELETE: '/user/delete',
      TOGGLE_STATUS: '/user/toggle-status'
    },
    
    // 认证相关接口
    AUTH: {
      REFRESH_TOKEN: '/auth/refresh',
      VERIFY_TOKEN: '/auth/verify',
      RESET_PASSWORD: '/auth/reset-password',
      SEND_CODE: '/auth/send-code'
    },
    
    // 数据统计接口
    DASHBOARD: {
      STATS: '/dashboard/stats',
      CHARTS: '/dashboard/charts',
      ACTIVITIES: '/dashboard/activities'
    },
    
    // 系统管理接口
    SYSTEM: {
      CONFIG: '/system/config',
      LOGS: '/system/logs',
      HEALTH: '/system/health'
    }
  },
  
  // 请求配置
  REQUEST: {
    TIMEOUT: TIMEOUT, // 从环境配置获取超时时间
    RETRY_COUNT: 3, // 重试次数
    RETRY_DELAY: 1000 // 重试延迟
  },
  
  // 响应状态码
  STATUS_CODES: {
    SUCCESS: 200,
    UNAUTHORIZED: 401,
    FORBIDDEN: 403,
    NOT_FOUND: 404,
    SERVER_ERROR: 500
  }
}

// 构建完整的 API URL
export const buildApiUrl = (endpoint: string): string => {
  return `${API_CONFIG.BASE_URL}${endpoint}`
}

// 获取特定服务的 API URL
export const getApiUrl = (category: keyof typeof API_CONFIG.ENDPOINTS, action: string): string => {
  const endpoints = API_CONFIG.ENDPOINTS[category] as Record<string, string>
  const endpoint = endpoints[action]
  
  if (!endpoint) {
    throw new Error(`API endpoint not found: ${String(category)}.${action}`)
  }
  
  return buildApiUrl(endpoint)
}

// 开发环境配置
export const isDevelopment = import.meta.env.DEV

// 根据环境变量动态配置（可选）
if (isDevelopment) {
  // 开发环境可以使用不同的配置
  console.log('API Config loaded for development:', API_CONFIG.BASE_URL)
}
