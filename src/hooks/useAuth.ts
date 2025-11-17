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

        if (token) {
          setAuthToken(token, headerName)
          setIsAuthenticated(true)
        } else if (required) {
          setIsAuthenticated(false)
        } else {
          setIsAuthenticated(true)
        }
      } catch (error) {
        console.error('Failed to init auth:', error)
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
