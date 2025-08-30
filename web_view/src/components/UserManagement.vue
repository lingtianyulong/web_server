<template>
    <div class="user-management-container">
        <!-- 面包屑导航 -->
        <div>
            <h3>用户管理</h3>
        </div>

        <!-- 操作栏 -->
        <div class="toolbar">
            <div class="search-section">
                <div class="search-group">
                    <input v-model="searchKeyword" type="text" placeholder="搜索用户名、邮箱或手机号..." class="search-input"
                        @input="handleSearch" />
                    <button class="search-btn" @click="handleSearch">
                        <span class="search-icon">🔍</span>
                        搜索
                    </button>
                </div>
                <div class="filter-group">
                    <select v-model="statusFilter" @change="handleFilter" class="filter-select">
                        <option value="">全部状态</option>
                        <option value="active">活跃</option>
                        <option value="inactive">未激活</option>
                        <option value="banned">已禁用</option>
                    </select>
                    <select v-model="roleFilter" @change="handleFilter" class="filter-select">
                        <option value="">全部角色</option>
                        <option value="admin">管理员</option>
                        <option value="user">普通用户</option>
                        <option value="vip">VIP用户</option>
                    </select>
                </div>
            </div>
            <div class="action-section">
                <button class="btn btn-primary" @click="showAddUserDialog = true">
                    <span class="btn-icon">➕</span>
                    添加用户
                </button>
                <button class="btn btn-secondary" @click="exportUsers">
                    <span class="btn-icon">📥</span>
                    导出数据
                </button>
            </div>
        </div>

        <!-- 用户表格 -->
        <div class="table-container">
            <table class="user-table">
                <thead>
                    <tr>
                        <th>
                            <input type="checkbox" v-model="selectAll" @change="handleSelectAll" class="checkbox" />
                        </th>
                        <th>用户ID</th>
                        <th>用户名</th>
                        <th>邮箱</th>
                        <th>角色</th>
                        <th>状态</th>
                        <th>注册时间</th>
                        <th>最后登录</th>
                        <th>操作</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="user in paginatedUsers" :key="user.id" class="table-row">
                        <td>
                            <input type="checkbox" v-model="selectedUsers" :value="user.id" class="checkbox" />
                        </td>
                        <td>{{ user.id }}</td>
                        <td>
                            <div class="user-info">
                                <div class="user-avatar">{{ user.username.charAt(0).toUpperCase() }}</div>
                                <span>{{ user.username }}</span>
                            </div>
                        </td>
                        <td>{{ user.email }}</td>
                        <td>
                            <span class="role-tag" :class="getRoleClass(user.role)">
                                {{ getRoleText(user.role) }}
                            </span>
                        </td>
                        <td>
                            <span class="status-tag" :class="getStatusClass(user.status)">
                                {{ getStatusText(user.status) }}
                            </span>
                        </td>
                        <td>{{ formatDate(user.createdAt) }}</td>
                        <td>{{ formatDate(user.lastLogin) }}</td>
                        <td>
                            <div class="action-buttons">
                                <button class="action-btn edit-btn" @click="editUser(user)">编辑</button>
                                <button class="action-btn"
                                    :class="user.status === 'active' ? 'ban-btn' : 'activate-btn'"
                                    @click="toggleUserStatus(user)">
                                    {{ user.status === 'active' ? '禁用' : '启用' }}
                                </button>
                                <button class="action-btn delete-btn" @click="deleteUser(user)">删除</button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <!-- 分页 -->
        <div class="pagination">
            <div class="pagination-info">
                显示第 {{ (currentPage - 1) * pageSize + 1 }} - {{ Math.min(currentPage * pageSize, filteredUsers.length)
                }} 条，
                共 {{ filteredUsers.length }} 条记录
            </div>
            <div class="pagination-controls">
                <button class="page-btn" :disabled="currentPage <= 1" @click="currentPage--">
                    上一页
                </button>
                <span class="page-info">{{ currentPage }} / {{ totalPages }}</span>
                <button class="page-btn" :disabled="currentPage >= totalPages" @click="currentPage++">
                    下一页
                </button>
            </div>
        </div>

        <!-- 添加/编辑用户对话框 -->
        <div v-if="showAddUserDialog || editingUser" class="modal-overlay" @click="closeDialog">
            <div class="modal-content" @click.stop>
                <div class="modal-header">
                    <h3>{{ editingUser ? '编辑用户' : '添加用户' }}</h3>
                    <button class="close-btn" @click="closeDialog">×</button>
                </div>
                <form @submit.prevent="saveUser" class="user-form">
                    <div class="form-row">
                        <div class="form-group">
                            <label>用户名</label>
                            <input v-model="userForm.username" type="text" required class="form-input" />
                        </div>
                        <div class="form-group">
                            <label>邮箱</label>
                            <input v-model="userForm.email" type="email" required class="form-input" />
                        </div>
                    </div>
                    <div class="form-row">
                        <div class="form-group">
                            <label>角色</label>
                            <select v-model="userForm.role" required class="form-select">
                                <option value="user">普通用户</option>
                                <option value="vip">VIP用户</option>
                                <option value="admin">管理员</option>
                            </select>
                        </div>
                        <div class="form-group">
                            <label>状态</label>
                            <select v-model="userForm.status" required class="form-select">
                                <option value="active">活跃</option>
                                <option value="inactive">未激活</option>
                                <option value="banned">已禁用</option>
                            </select>
                        </div>
                    </div>
                    <div class="form-row">
                        <div class="form-group">
                            <label>手机号</label>
                            <input v-model="userForm.phone" type="tel" class="form-input" />
                        </div>
                    </div>
                    <div class="form-actions">
                        <button type="button" class="btn btn-secondary" @click="closeDialog">取消</button>
                        <button type="submit" class="btn btn-primary">{{ editingUser ? '更新' : '创建' }}</button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive } from 'vue'
import { ArrowRight } from '@element-plus/icons-vue'

// 用户数据类型
interface User {
    id: number
    username: string
    email: string
    role: 'admin' | 'user' | 'vip'
    status: 'active' | 'inactive' | 'banned'
    createdAt: string
    lastLogin: string
    phone?: string
}

// 响应式数据
const searchKeyword = ref('')
const statusFilter = ref('')
const roleFilter = ref('')
const currentPage = ref(1)
const pageSize = ref(10)
const selectAll = ref(false)
const selectedUsers = ref<number[]>([])
const showAddUserDialog = ref(false)
const editingUser = ref<User | null>(null)

// 用户表单
const userForm = reactive({
    username: '',
    email: '',
    role: 'user' as 'admin' | 'user' | 'vip',
    status: 'active' as 'active' | 'inactive' | 'banned',
    phone: ''
})

// 模拟用户数据
const users = ref<User[]>([
    {
        id: 1,
        username: '张三',
        email: 'zhangsan@example.com',
        role: 'admin',
        status: 'active',
        createdAt: '2024-01-01 10:00:00',
        lastLogin: '2024-01-15 14:30:00',
        phone: '13800138001'
    },
    {
        id: 2,
        username: '李四',
        email: 'lisi@example.com',
        role: 'user',
        status: 'active',
        createdAt: '2024-01-02 11:00:00',
        lastLogin: '2024-01-15 13:45:00',
        phone: '13800138002'
    },
    {
        id: 3,
        username: '王五',
        email: 'wangwu@example.com',
        role: 'vip',
        status: 'inactive',
        createdAt: '2024-01-03 12:00:00',
        lastLogin: '2024-01-10 16:20:00',
        phone: '13800138003'
    },
    {
        id: 4,
        username: '赵六',
        email: 'zhaoliu@example.com',
        role: 'user',
        status: 'banned',
        createdAt: '2024-01-04 13:00:00',
        lastLogin: '2024-01-05 09:15:00',
        phone: '13800138004'
    },
    {
        id: 5,
        username: '钱七',
        email: 'qianqi@example.com',
        role: 'user',
        status: 'active',
        createdAt: '2024-01-05 14:00:00',
        lastLogin: '2024-01-15 11:30:00',
        phone: '13800138005'
    }
])

// 计算属性
const filteredUsers = computed(() => {
    return users.value.filter(user => {
        const matchesSearch = !searchKeyword.value ||
            user.username.toLowerCase().includes(searchKeyword.value.toLowerCase()) ||
            user.email.toLowerCase().includes(searchKeyword.value.toLowerCase()) ||
            (user.phone && user.phone.includes(searchKeyword.value))

        const matchesStatus = !statusFilter.value || user.status === statusFilter.value
        const matchesRole = !roleFilter.value || user.role === roleFilter.value

        return matchesSearch && matchesStatus && matchesRole
    })
})

const paginatedUsers = computed(() => {
    const start = (currentPage.value - 1) * pageSize.value
    const end = start + pageSize.value
    return filteredUsers.value.slice(start, end)
})

const totalPages = computed(() => {
    return Math.ceil(filteredUsers.value.length / pageSize.value)
})

// 方法
const handleSearch = () => {
    currentPage.value = 1
}

const handleFilter = () => {
    currentPage.value = 1
}

const handleSelectAll = () => {
    if (selectAll.value) {
        selectedUsers.value = paginatedUsers.value.map(user => user.id)
    } else {
        selectedUsers.value = []
    }
}

const editUser = (user: User) => {
    editingUser.value = user
    userForm.username = user.username
    userForm.email = user.email
    userForm.role = user.role
    userForm.status = user.status
    userForm.phone = user.phone || ''
}

const deleteUser = (user: User) => {
    if (confirm(`确定要删除用户 "${user.username}" 吗？`)) {
        const index = users.value.findIndex(u => u.id === user.id)
        if (index > -1) {
            users.value.splice(index, 1)
            alert('用户删除成功')
        }
    }
}

const toggleUserStatus = (user: User) => {
    const newStatus = user.status === 'active' ? 'banned' : 'active'
    const action = newStatus === 'active' ? '启用' : '禁用'

    if (confirm(`确定要${action}用户 "${user.username}" 吗？`)) {
        user.status = newStatus
        alert(`用户${action}成功`)
    }
}

const saveUser = () => {
    if (editingUser.value) {
        // 编辑用户
        const user = users.value.find(u => u.id === editingUser.value!.id)
        if (user) {
            user.username = userForm.username
            user.email = userForm.email
            user.role = userForm.role
            user.status = userForm.status
            user.phone = userForm.phone
            alert('用户更新成功')
        }
    } else {
        // 添加新用户
        const newUser: User = {
            id: Math.max(...users.value.map(u => u.id)) + 1,
            username: userForm.username,
            email: userForm.email,
            role: userForm.role,
            status: userForm.status,
            phone: userForm.phone,
            createdAt: new Date().toLocaleString('zh-CN'),
            lastLogin: '从未登录'
        }
        users.value.push(newUser)
        alert('用户创建成功')
    }
    closeDialog()
}

const closeDialog = () => {
    showAddUserDialog.value = false
    editingUser.value = null
    userForm.username = ''
    userForm.email = ''
    userForm.role = 'user'
    userForm.status = 'active'
    userForm.phone = ''
}

const exportUsers = () => {
    alert('导出功能开发中...')
}

// 辅助函数
const getRoleClass = (role: string) => {
    const classes = {
        admin: 'role-admin',
        vip: 'role-vip',
        user: 'role-user'
    }
    return classes[role as keyof typeof classes] || 'role-user'
}

const getRoleText = (role: string) => {
    const texts = {
        admin: '管理员',
        vip: 'VIP用户',
        user: '普通用户'
    }
    return texts[role as keyof typeof texts] || '普通用户'
}

const getStatusClass = (status: string) => {
    const classes = {
        active: 'status-active',
        inactive: 'status-inactive',
        banned: 'status-banned'
    }
    return classes[status as keyof typeof classes] || 'status-inactive'
}

const getStatusText = (status: string) => {
    const texts = {
        active: '活跃',
        inactive: '未激活',
        banned: '已禁用'
    }
    return texts[status as keyof typeof texts] || '未激活'
}

const formatDate = (dateStr: string) => {
    if (dateStr === '从未登录') return dateStr
    return new Date(dateStr).toLocaleString('zh-CN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit'
    })
}
</script>

<style scoped>
.user-management-container {
    background: #f5f5f5;
    min-height: 100%;
}

.breadcrumb {
    background: white;
    padding: 12px 24px;
    margin: 0;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    border-bottom: 1px solid #ebeef5;
}

/* 工具栏 */
.toolbar {
    background: white;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
    margin: 24px 24px 20px 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 16px;
}

.search-section {
    display: flex;
    gap: 16px;
    flex-wrap: wrap;
}

.search-group {
    display: flex;
    gap: 8px;
}

.search-input {
    width: 300px;
    padding: 8px 12px;
    border: 1px solid #dcdfe6;
    border-radius: 4px;
    font-size: 14px;
}

.search-input:focus {
    outline: none;
    border-color: #409EFF;
}

.search-btn {
    padding: 8px 16px;
    background: #409EFF;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 4px;
    transition: background 0.3s;
}

.search-btn:hover {
    background: #66b1ff;
}

.filter-group {
    display: flex;
    gap: 8px;
}

.filter-select {
    padding: 8px 12px;
    border: 1px solid #dcdfe6;
    border-radius: 4px;
    font-size: 14px;
    min-width: 120px;
}

.action-section {
    display: flex;
    gap: 12px;
}

.btn {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    display: flex;
    align-items: center;
    gap: 4px;
    transition: all 0.3s;
}

.btn-primary {
    background: #409EFF;
    color: white;
}

.btn-primary:hover {
    background: #66b1ff;
}

.btn-secondary {
    background: #909399;
    color: white;
}

.btn-secondary:hover {
    background: #a6a9ad;
}

/* 表格样式 */
.table-container {
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
    overflow: hidden;
    margin: 0 24px 20px 24px;
}

.user-table {
    width: 100%;
    border-collapse: collapse;
}

.user-table th,
.user-table td {
    padding: 12px;
    text-align: left;
    border-bottom: 1px solid #ebeef5;
}

.user-table th {
    background: #f5f7fa;
    color: #606266;
    font-weight: 600;
    font-size: 14px;
}

.table-row:hover {
    background: #f5f7fa;
}

.user-info {
    display: flex;
    align-items: center;
    gap: 8px;
}

.user-avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: #409EFF;
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 14px;
}

.role-tag,
.status-tag {
    padding: 2px 8px;
    border-radius: 12px;
    font-size: 12px;
    font-weight: 500;
}

.role-admin {
    background: #f56c6c;
    color: white;
}

.role-vip {
    background: #e6a23c;
    color: white;
}

.role-user {
    background: #909399;
    color: white;
}

.status-active {
    background: #67c23a;
    color: white;
}

.status-inactive {
    background: #e6a23c;
    color: white;
}

.status-banned {
    background: #f56c6c;
    color: white;
}

.action-buttons {
    display: flex;
    gap: 4px;
}

.action-btn {
    padding: 4px 8px;
    border: none;
    border-radius: 3px;
    cursor: pointer;
    font-size: 12px;
    transition: all 0.3s;
}

.edit-btn {
    background: #409EFF;
    color: white;
}

.edit-btn:hover {
    background: #66b1ff;
}

.ban-btn {
    background: #e6a23c;
    color: white;
}

.ban-btn:hover {
    background: #ebb563;
}

.activate-btn {
    background: #67c23a;
    color: white;
}

.activate-btn:hover {
    background: #85ce61;
}

.delete-btn {
    background: #f56c6c;
    color: white;
}

.delete-btn:hover {
    background: #f78989;
}

/* 分页样式 */
.pagination {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: white;
    padding: 16px 20px;
    border-radius: 8px;
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
    margin: 0 24px 24px 24px;
}

.pagination-info {
    color: #606266;
    font-size: 14px;
}

.pagination-controls {
    display: flex;
    align-items: center;
    gap: 12px;
}

.page-btn {
    padding: 6px 12px;
    border: 1px solid #dcdfe6;
    background: white;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.3s;
}

.page-btn:hover:not(:disabled) {
    background: #409EFF;
    color: white;
    border-color: #409EFF;
}

.page-btn:disabled {
    background: #f5f7fa;
    color: #c0c4cc;
    cursor: not-allowed;
}

.page-info {
    color: #606266;
    font-size: 14px;
}

/* 模态框样式 */
.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.modal-content {
    background: white;
    border-radius: 8px;
    width: 90%;
    max-width: 600px;
    max-height: 90vh;
    overflow-y: auto;
}

.modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 24px;
    border-bottom: 1px solid #ebeef5;
}

.modal-header h3 {
    margin: 0;
    color: #303133;
    font-size: 18px;
    font-weight: 600;
}

.close-btn {
    background: none;
    border: none;
    font-size: 24px;
    cursor: pointer;
    color: #909399;
    padding: 0;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.close-btn:hover {
    color: #606266;
}

.user-form {
    padding: 24px;
}

.form-row {
    display: flex;
    gap: 16px;
    margin-bottom: 20px;
}

.form-group {
    flex: 1;
}

.form-group label {
    display: block;
    margin-bottom: 6px;
    color: #606266;
    font-weight: 500;
    font-size: 14px;
}

.form-input,
.form-select {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #dcdfe6;
    border-radius: 4px;
    font-size: 14px;
    box-sizing: border-box;
}

.form-input:focus,
.form-select:focus {
    outline: none;
    border-color: #409EFF;
}

.form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 24px;
    padding-top: 20px;
    border-top: 1px solid #ebeef5;
}

.checkbox {
    cursor: pointer;
}

/* 响应式设计 */
@media (max-width: 768px) {
    .breadcrumb {
        padding: 12px 16px;
    }

    .toolbar,
    .table-container,
    .pagination {
        margin-left: 12px;
        margin-right: 12px;
    }

    .toolbar {
        flex-direction: column;
        align-items: stretch;
    }

    .search-section {
        flex-direction: column;
    }

    .search-group,
    .filter-group {
        justify-content: stretch;
    }

    .search-input {
        width: 100%;
    }

    .action-section {
        justify-content: center;
    }

    .form-row {
        flex-direction: column;
    }

    .user-table {
        font-size: 12px;
    }

    .user-table th,
    .user-table td {
        padding: 8px 4px;
    }

    .action-buttons {
        flex-direction: column;
    }
}
</style>
