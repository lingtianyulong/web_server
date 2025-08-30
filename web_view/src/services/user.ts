import { httpClient } from './http'
import type { ApiResponse } from './http'
import { getApiUrl } from '../config/api'

// 用户相关的数据类型
export interface User {
  id: number
  username: string
  email: string
  role: 'admin' | 'user' | 'vip'
  status: 'active' | 'inactive' | 'banned'
  createdAt: string
  lastLogin: string
  phone?: string
  avatar?: string
}

export interface CreateUserRequest {
  username: string
  email: string
  role: 'admin' | 'user' | 'vip'
  status: 'active' | 'inactive' | 'banned'
  phone?: string
  password?: string
}

export interface UpdateUserRequest {
  id: number
  username?: string
  email?: string
  role?: 'admin' | 'user' | 'vip'
  status?: 'active' | 'inactive' | 'banned'
  phone?: string
}

export interface UserListQuery {
  page?: number
  pageSize?: number
  keyword?: string
  status?: string
  role?: string
  sortBy?: string
  sortOrder?: 'asc' | 'desc'
}

export interface UserListResponse {
  users: User[]
  total: number
  page: number
  pageSize: number
  totalPages: number
}

// 用户管理服务类
class UserService {
  /**
   * 获取用户列表
   */
  async getUserList(query: UserListQuery = {}): Promise<ApiResponse<UserListResponse>> {
    // 构建查询参数
    const searchParams = new URLSearchParams()
    Object.entries(query).forEach(([key, value]) => {
      if (value !== undefined && value !== null && value !== '') {
        searchParams.append(key, String(value))
      }
    })
    
    const queryString = searchParams.toString()
    const endpoint = queryString 
      ? `${getApiUrl('USER', 'LIST')}?${queryString}`
      : getApiUrl('USER', 'LIST')
    
    return httpClient.get<UserListResponse>(endpoint)
  }

  /**
   * 创建用户
   */
  async createUser(userData: CreateUserRequest): Promise<ApiResponse<User>> {
    return httpClient.post<User>(getApiUrl('USER', 'CREATE'), userData)
  }

  /**
   * 更新用户信息
   */
  async updateUser(userData: UpdateUserRequest): Promise<ApiResponse<User>> {
    return httpClient.put<User>(`${getApiUrl('USER', 'UPDATE')}/${userData.id}`, userData)
  }

  /**
   * 删除用户
   */
  async deleteUser(userId: number): Promise<ApiResponse<any>> {
    return httpClient.delete(`${getApiUrl('USER', 'DELETE')}/${userId}`)
  }

  /**
   * 批量删除用户
   */
  async batchDeleteUsers(userIds: number[]): Promise<ApiResponse<any>> {
    return httpClient.post(getApiUrl('USER', 'DELETE'), { userIds })
  }

  /**
   * 切换用户状态（启用/禁用）
   */
  async toggleUserStatus(userId: number, status: 'active' | 'banned'): Promise<ApiResponse<any>> {
    return httpClient.post(`${getApiUrl('USER', 'TOGGLE_STATUS')}/${userId}`, { status })
  }

  /**
   * 获取用户详细信息
   */
  async getUserDetail(userId: number): Promise<ApiResponse<User>> {
    return httpClient.get<User>(`${getApiUrl('USER', 'PROFILE')}/${userId}`)
  }

  /**
   * 重置用户密码
   */
  async resetUserPassword(userId: number, newPassword: string): Promise<ApiResponse<any>> {
    return httpClient.post(`${getApiUrl('USER', 'CHANGE_PASSWORD')}/${userId}`, { 
      newPassword 
    })
  }

  /**
   * 导出用户数据
   */
  async exportUsers(query: UserListQuery = {}): Promise<ApiResponse<any>> {
    const searchParams = new URLSearchParams()
    Object.entries(query).forEach(([key, value]) => {
      if (value !== undefined && value !== null && value !== '') {
        searchParams.append(key, String(value))
      }
    })
    
    const queryString = searchParams.toString()
    const endpoint = queryString 
      ? `/user/export?${queryString}`
      : '/user/export'
    
    return httpClient.get(endpoint)
  }

  /**
   * 检查用户名是否可用
   */
  async checkUsernameAvailable(username: string): Promise<ApiResponse<{ available: boolean }>> {
    return httpClient.get<{ available: boolean }>(`/user/check-username?username=${encodeURIComponent(username)}`)
  }

  /**
   * 检查邮箱是否可用
   */
  async checkEmailAvailable(email: string): Promise<ApiResponse<{ available: boolean }>> {
    return httpClient.get<{ available: boolean }>(`/user/check-email?email=${encodeURIComponent(email)}`)
  }
}

// 创建并导出用户服务实例
export const userService = new UserService()

// 导出便捷方法
export const {
  getUserList,
  createUser,
  updateUser,
  deleteUser,
  batchDeleteUsers,
  toggleUserStatus,
  getUserDetail,
  resetUserPassword,
  exportUsers,
  checkUsernameAvailable,
  checkEmailAvailable
} = userService
