import React, { useState } from 'react'
import { useAuth } from '@/hooks/useAuth'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Loader2 } from 'lucide-react'
import api from '@/services/api'

interface LoginProps {
  onLoginSuccess: () => void
}

interface AuthResponse {
  success: boolean
  message: string
  header_name?: string
  role?: string
}

export function Login({ onLoginSuccess }: LoginProps) {
  const { login } = useAuth()
  const [token, setToken] = useState('')
  const [headerName, setHeaderName] = useState('Authorization')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault()
    setError('')

    if (!token.trim()) {
      setError('请输入授权令牌')
      return
    }

    setLoading(true)
    try {
      // 验证token
      const response = await api.post('/api/auth/verify', {
        token: token.trim(),
      }) as AuthResponse

      if (response.success) {
        // 保存用户角色
        if (response.role) {
          localStorage.setItem('user_role', response.role)
        } else {
          localStorage.setItem('user_role', 'user')
        }
        login(token, headerName)
        onLoginSuccess()
      } else {
        setError(response.message || '认证失败')
      }
    } catch (err) {
      setError('登录失败，请检查令牌是否正确')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="flex items-center justify-center min-h-screen bg-gradient-to-br from-background to-background p-3">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-2 p-4 md:p-6">
          <CardTitle className="text-xl md:text-2xl">RIFS</CardTitle>
          <CardDescription className="text-xs md:text-sm">
            高性能 Rust 图床服务 - 请登录继续
          </CardDescription>
        </CardHeader>
        <CardContent className="p-4 md:p-6 pt-0">
          <form onSubmit={handleLogin} className="space-y-3 md:space-y-4">
            <div className="space-y-1.5 md:space-y-2">
              <Label htmlFor="token" className="text-xs md:text-sm">授权令牌</Label>
              <Input
                id="token"
                type="password"
                placeholder="输入您的令牌"
                value={token}
                onChange={(e) => setToken(e.target.value)}
                disabled={loading}
                className="text-xs md:text-sm"
              />
            </div>

            <div className="space-y-1.5 md:space-y-2">
              <Label htmlFor="header-name" className="text-xs md:text-sm">自定义头名称（可选）</Label>
              <Input
                id="header-name"
                type="text"
                placeholder="Authorization"
                value={headerName}
                onChange={(e) => setHeaderName(e.target.value)}
                disabled={loading}
                className="text-xs md:text-sm"
              />
            </div>

            {error && (
              <div className="p-2 md:p-3 bg-destructive/10 border border-destructive/30 rounded text-destructive text-xs md:text-sm">
                {error}
              </div>
            )}

            <Button
              type="submit"
              disabled={loading}
              className="w-full text-xs md:text-sm"
              size="sm"
            >
              {loading ? (
                <>
                  <Loader2 className="w-3 h-3 md:w-4 md:h-4 mr-2 animate-spin" />
                  登录中...
                </>
              ) : (
                '登录'
              )}
            </Button>
          </form>
        </CardContent>
      </Card>
    </div>
  )
}
