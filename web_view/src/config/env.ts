// 环境配置文件

export interface EnvironmentConfig {
  API_BASE_URL: string
  APP_NAME: string
  VERSION: string
  DEBUG: boolean
  TIMEOUT: number
}

// 开发环境配置
const developmentConfig: EnvironmentConfig = {
  API_BASE_URL: 'http://127.0.0.1:8080',
  APP_NAME: '系统管理平台',
  VERSION: '1.0.0',
  DEBUG: true,
  TIMEOUT: 10000
}

// 测试环境配置
const testConfig: EnvironmentConfig = {
  API_BASE_URL: 'http://test-api.example.com',
  APP_NAME: '系统管理平台-测试',
  VERSION: '1.0.0',
  DEBUG: true,
  TIMEOUT: 15000
}

// 生产环境配置
const productionConfig: EnvironmentConfig = {
  API_BASE_URL: 'https://api.example.com',
  APP_NAME: '系统管理平台',
  VERSION: '1.0.0',
  DEBUG: false,
  TIMEOUT: 30000
}

// 根据环境变量选择配置
const getConfig = (): EnvironmentConfig => {
  const env = import.meta.env.MODE || 'development'
  
  switch (env) {
    case 'test':
      return testConfig
    case 'production':
      return productionConfig
    case 'development':
    default:
      return developmentConfig
  }
}

// 导出当前环境配置
export const ENV_CONFIG = getConfig()

// 导出常用配置项
export const {
  API_BASE_URL,
  APP_NAME,
  VERSION,
  DEBUG,
  TIMEOUT
} = ENV_CONFIG

// 环境检查函数
export const isDevelopment = () => import.meta.env.DEV
export const isProduction = () => import.meta.env.PROD
export const isTest = () => import.meta.env.MODE === 'test'

// 日志函数（仅在开发环境输出）
export const devLog = (...args: any[]) => {
  if (DEBUG) {
    console.log('[DEV]', ...args)
  }
}

export const devWarn = (...args: any[]) => {
  if (DEBUG) {
    console.warn('[DEV]', ...args)
  }
}

export const devError = (...args: any[]) => {
  if (DEBUG) {
    console.error('[DEV]', ...args)
  }
}
