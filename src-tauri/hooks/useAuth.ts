import { useState, useEffect } from 'react'
import { checkAuthRequired, setAuthToken, getUserInfo } from '@/services/api'

export type UserRole = 'admin' | 'user'

export interface UserInfo {
  name: string
  role: UserRole
}

export function useAuth() {
  const [isAuthenticated, setIsAuthenticated] = useState(false)
  const [authRequired, setAuthRequired] = useState(false)
  const [loading, setLoading] = useState(true)
  const [userRole, setUserRole] = useState<UserRole>('user')
  const [userInfo, setUserInfo] = useState<UserInfo | null>(null)

  useEffect(() => {
    async function initAuth() {
      try {
        const required = await checkAuthRequired()
        setAuthRequired(required)

        const token = localStorage.getItem('auth_token')
        const headerName = localStorage.getItem('auth_header_name') || 'Authorization'

        // 如果需要认证，必须有有效的令牌
        if (required) {
          if (token) {
            setAuthToken(token, headerName)
            setIsAuthenticated(true)
            // 获取用户信息
            try {
              const userInfo = await getUserInfo()
              if (userInfo && userInfo.data) {
                setUserInfo(userInfo.data)
                setUserRole((userInfo.data.role as UserRole) || 'user')
              }
            } catch (error) {
              console.error('Failed to fetch user info:', error)
              setUserRole('user')
            }
          } else {
            // 认证是必需的但没有令牌，保持未认证状态
            setIsAuthenticated(false)
          }
        } else {
          // 认证不是必需的，允许访问
          if (token) {
            setAuthToken(token, headerName)
            setIsAuthenticated(true)
            // 获取用户信息
            try {
              const userInfo = await getUserInfo()
              if (userInfo && userInfo.data) {
                setUserInfo(userInfo.data)
                setUserRole((userInfo.data.role as UserRole) || 'user')
              }
            } catch (error) {
              console.error('Failed to fetch user info:', error)
              setUserRole('admin') // 如果没有认证要求，默认为 admin
            }
          } else {
            setIsAuthenticated(true)
            setUserRole('admin') // 没有认证时默认为 admin
          }
        }
      } catch (error) {
        console.error('Failed to init auth:', error)
        // 如果检查认证配置失败，默认允许访问（与原始行为一致）
        setIsAuthenticated(true)
        setUserRole('admin')
      } finally {
        setLoading(false)
      }
    }

    initAuth()
  }, [])

  const login = async (token: string, headerName: string = 'Authorization') => {
    localStorage.setItem('auth_token', token)
    localStorage.setItem('auth_header_name', headerName)
    setAuthToken(token, headerName)
    setIsAuthenticated(true)
    
    // 获取用户信息
    try {
      const userInfo = await getUserInfo()
      if (userInfo && userInfo.data) {
        setUserInfo(userInfo.data)
        setUserRole((userInfo.data.role as UserRole) || 'user')
      }
    } catch (error) {
      console.error('Failed to fetch user info:', error)
      setUserRole('user')
    }
  }

  const logout = () => {
    localStorage.removeItem('auth_token')
    localStorage.removeItem('auth_header_name')
    setAuthToken(null)
    setIsAuthenticated(false)
    setUserRole('user')
    setUserInfo(null)
  }

  return {
    isAuthenticated,
    authRequired,
    loading,
    userRole,
    userInfo,
    login,
    logout,
  }
}
