import { useState, useEffect } from 'react'
import { checkAuthRequired, setAuthToken } from '@/services/api'

export function useAuth() {
  const [isAuthenticated, setIsAuthenticated] = useState(false)
  const [authRequired, setAuthRequired] = useState(false)
  const [loading, setLoading] = useState(true)

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
          } else {
            // 认证是必需的但没有令牌，保持未认证状态
            setIsAuthenticated(false)
          }
        } else {
          // 认证不是必需的，允许访问
          if (token) {
            setAuthToken(token, headerName)
            setIsAuthenticated(true)
          } else {
            setIsAuthenticated(true)
          }
        }
      } catch (error) {
        console.error('Failed to init auth:', error)
        // 如果检查认证配置失败，默认允许访问（与原始行为一致）
        setIsAuthenticated(true)
      } finally {
        setLoading(false)
      }
    }

    initAuth()
  }, [])

  const login = (token: string, headerName: string = 'Authorization') => {
    localStorage.setItem('auth_token', token)
    localStorage.setItem('auth_header_name', headerName)
    setAuthToken(token, headerName)
    setIsAuthenticated(true)
  }

  const logout = () => {
    localStorage.removeItem('auth_token')
    localStorage.removeItem('auth_header_name')
    setAuthToken(null)
    setIsAuthenticated(false)
  }

  return {
    isAuthenticated,
    authRequired,
    loading,
    login,
    logout,
  }
}
