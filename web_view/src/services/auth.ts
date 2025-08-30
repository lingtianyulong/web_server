import { httpClient } from './http'
import type { ApiResponse } from './http'
import { getApiUrl } from '../config/api'

// 认证相关的数据类型
export interface LoginRequest {
  user_name: string
  password: string
//   remember?: boolean
}

export interface LoginResponse {
  token: string
  user: {
    id: number
    username: string
    email: string
    role: string
    avatar?: string
  }
  expiresIn: number
}

export interface RegisterRequest {
  username: string
  email: string
  password: string
  confirmPassword: string
  phone?: string
}

export interface UserProfile {
  id: number
  username: string
  email: string
  role: string
  avatar?: string
  phone?: string
  createdAt: string
  lastLogin: string
}

export interface ChangePasswordRequest {
  oldPassword: string
  newPassword: string
  confirmPassword: string
}

// 认证服务类
class AuthService {
  /**
   * 用户登录
   */
  async login(loginData: LoginRequest): Promise<ApiResponse<LoginResponse>> {
    try {
      const response = await httpClient.post<LoginResponse>(
        getApiUrl('USER', 'LOGIN'),
        loginData
      )

      console.log('response', response)

      // 登录成功后保存 token
      if (response.code === 200 && response.data?.token) {
        this.setToken(response.data.token)
        
        // 如果用户选择记住我，设置更长的过期时间
        // if (loginData.remember) {
        //   this.setRememberMe(true)
        // }
      }

      return response
    } catch (error) {
      console.error('Login failed:', error)
      throw error
    }
  }

  /**
   * 用户登出
   */
  async logout(): Promise<void> {
    try {
      // 调用服务器登出接口（可选）
      await httpClient.post(getApiUrl('USER', 'LOGOUT'))
    } catch (error) {
      console.warn('Logout request failed:', error)
    } finally {
      // 无论服务器响应如何，都清除本地数据
      this.clearAuth()
    }
  }

  /**
   * 用户注册
   */
  async register(registerData: RegisterRequest): Promise<ApiResponse<any>> {
    return httpClient.post(getApiUrl('USER', 'REGISTER'), registerData)
  }

  /**
   * 获取用户信息
   */
  async getUserProfile(): Promise<ApiResponse<UserProfile>> {
    return httpClient.get<UserProfile>(getApiUrl('USER', 'PROFILE'))
  }

  /**
   * 更新用户信息
   */
  async updateProfile(profileData: Partial<UserProfile>): Promise<ApiResponse<any>> {
    return httpClient.put(getApiUrl('USER', 'UPDATE_PROFILE'), profileData)
  }

  /**
   * 修改密码
   */
  async changePassword(passwordData: ChangePasswordRequest): Promise<ApiResponse<any>> {
    return httpClient.post(getApiUrl('USER', 'CHANGE_PASSWORD'), passwordData)
  }

  /**
   * 刷新 token
   */
  async refreshToken(): Promise<ApiResponse<{ token: string }>> {
    return httpClient.post(getApiUrl('AUTH', 'REFRESH_TOKEN'))
  }

  /**
   * 验证 token 有效性
   */
  async verifyToken(): Promise<ApiResponse<any>> {
    return httpClient.get(getApiUrl('AUTH', 'VERIFY_TOKEN'))
  }

  /**
   * 重置密码
   */
  async resetPassword(email: string): Promise<ApiResponse<any>> {
    return httpClient.post(getApiUrl('AUTH', 'RESET_PASSWORD'), { email })
  }

  /**
   * 发送验证码
   */
  async sendVerificationCode(phone: string): Promise<ApiResponse<any>> {
    return httpClient.post(getApiUrl('AUTH', 'SEND_CODE'), { phone })
  }

  // Token 管理方法
  
  /**
   * 设置认证 token
   */
  setToken(token: string): void {
    localStorage.setItem('token', token)
  }

  /**
   * 获取认证 token
   */
  getToken(): string | null {
    return localStorage.getItem('token')
  }

  /**
   * 移除认证 token
   */
  removeToken(): void {
    localStorage.removeItem('token')
  }

  /**
   * 设置记住我状态
   */
  setRememberMe(remember: boolean): void {
    if (remember) {
      localStorage.setItem('remember_me', 'true')
    } else {
      localStorage.removeItem('remember_me')
    }
  }

  /**
   * 获取记住我状态
   */
  getRememberMe(): boolean {
    return localStorage.getItem('remember_me') === 'true'
  }

  /**
   * 检查是否已登录
   */
  isAuthenticated(): boolean {
    const token = this.getToken()
    return !!token
  }

  /**
   * 清除所有认证信息
   */
  clearAuth(): void {
    this.removeToken()
    localStorage.removeItem('remember_me')
    localStorage.removeItem('user_info')
  }

  /**
   * 保存用户信息
   */
  setUserInfo(userInfo: any): void {
    localStorage.setItem('user_info', JSON.stringify(userInfo))
  }

  /**
   * 获取用户信息
   */
  getUserInfo(): any {
    const userInfo = localStorage.getItem('user_info')
    return userInfo ? JSON.parse(userInfo) : null
  }

  /**
   * 自动刷新 token（在 token 即将过期时）
   */
  async autoRefreshToken(): Promise<boolean> {
    try {
      const response = await this.refreshToken()
      if (response.code === 200 && response.data?.token) {
        this.setToken(response.data.token)
        return true
      }
      return false
    } catch (error) {
      console.error('Auto refresh token failed:', error)
      this.clearAuth()
      return false
    }
  }
}

// 创建并导出认证服务实例
export const authService = new AuthService()

// 导出便捷方法
export const {
  login,
  logout,
  register,
  getUserProfile,
  updateProfile,
  changePassword,
  resetPassword,
  sendVerificationCode,
  isAuthenticated,
  getToken,
  setToken,
  clearAuth,
  getUserInfo,
  setUserInfo
} = authService
