import { API_CONFIG } from '../config/api'

// HTTP 响应接口
export interface ApiResponse<T = any> {
  code: number
  message: string
  data?: T
  timestamp?: string
}

// HTTP 请求配置接口
export interface RequestConfig {
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
  headers?: Record<string, string>
  body?: any
  timeout?: number
}

// HTTP 错误类
export class HttpError extends Error {
  public code: number
  public status: number

  constructor(message: string, code: number, status: number) {
    super(message)
    this.name = 'HttpError'
    this.code = code
    this.status = status
  }
}

// HTTP 客户端类
class HttpClient {
  private baseURL: string
  private defaultHeaders: Record<string, string>

  constructor() {
    this.baseURL = API_CONFIG.BASE_URL
    this.defaultHeaders = {
      'Content-Type': 'application/json',
      'Accept': 'application/json'
    }
  }

  // 获取认证 token
  private getAuthToken(): string | null {
    return localStorage.getItem('token')
  }

  // 设置认证头
  private setAuthHeaders(headers: Record<string, string>): Record<string, string> {
    const token = this.getAuthToken()
    if (token) {
      headers['Authorization'] = `Bearer ${token}`
    }
    return headers
  }

  // 处理请求超时
  private withTimeout(promise: Promise<Response>, timeout: number): Promise<Response> {
    return Promise.race([
      promise,
      new Promise<Response>((_, reject) => {
        setTimeout(() => {
          reject(new Error('Request timeout'))
        }, timeout)
      })
    ])
  }

  // 处理响应
  private async handleResponse<T>(response: Response): Promise<ApiResponse<T>> {
    const contentType = response.headers.get('content-type')
    
    let data: any
    if (contentType && contentType.includes('application/json')) {
      data = await response.json()
    } else {
      data = await response.text()
    }

    if (!response.ok) {
      // 处理 HTTP 错误状态
      const error = new HttpError(
        data.message || `HTTP Error: ${response.status}`,
        data.code || response.status,
        response.status
      )
      throw error
    }

    return data
  }

  // 通用请求方法
  public async request<T = any>(
    endpoint: string, 
    config: RequestConfig = {}
  ): Promise<ApiResponse<T>> {
    const {
      method = 'GET',
      headers = {},
      body,
      timeout = API_CONFIG.REQUEST.TIMEOUT
    } = config

    // 构建完整 URL
    const url = endpoint.startsWith('http') ? endpoint : `${this.baseURL}${endpoint}`

    // 合并请求头
    const requestHeaders = this.setAuthHeaders({
      ...this.defaultHeaders,
      ...headers
    })

    // 构建请求配置
    const requestConfig: RequestInit = {
      method,
      headers: requestHeaders
    }

    // 添加请求体（GET 请求除外）
    if (body && method !== 'GET') {
      requestConfig.body = typeof body === 'string' ? body : JSON.stringify(body)
    }

    try {
      // 发送请求（带超时）
      const response = await this.withTimeout(
        fetch(url, requestConfig),
        timeout
      )

      return await this.handleResponse<T>(response)
    } catch (error) {
      if (error instanceof HttpError) {
        throw error
      }
      
      // 处理网络错误或超时
      throw new HttpError(
        error instanceof Error ? error.message : 'Network error',
        0,
        0
      )
    }
  }

  // GET 请求
  public async get<T = any>(endpoint: string, headers?: Record<string, string>): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, { method: 'GET', headers })
  }

  // POST 请求
  public async post<T = any>(
    endpoint: string, 
    body?: any, 
    headers?: Record<string, string>
  ): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, { method: 'POST', body, headers })
  }

  // PUT 请求
  public async put<T = any>(
    endpoint: string, 
    body?: any, 
    headers?: Record<string, string>
  ): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, { method: 'PUT', body, headers })
  }

  // DELETE 请求
  public async delete<T = any>(
    endpoint: string, 
    headers?: Record<string, string>
  ): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, { method: 'DELETE', headers })
  }

  // PATCH 请求
  public async patch<T = any>(
    endpoint: string, 
    body?: any, 
    headers?: Record<string, string>
  ): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, { method: 'PATCH', body, headers })
  }
}

// 导出 HTTP 客户端实例
export const httpClient = new HttpClient()

// 导出便捷方法
export const { get, post, put, delete: del, patch } = httpClient
