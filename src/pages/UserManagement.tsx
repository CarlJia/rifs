import React, { useState, useEffect } from 'react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Loader2, Copy, Trash2, Plus, Eye, EyeOff } from 'lucide-react'
import api from '@/services/api'

interface TokenInfo {
  id: number
  name: string
  role: string
  is_active: boolean
  created_at: string
  last_used_at?: string
  max_upload_size?: number
  used_upload_size: number
}

interface CreateTokenPayload {
  name: string
  role: string
  max_upload_size?: number
  expires_at?: string
}

export function UserManagement() {
  const [tokens, setTokens] = useState<TokenInfo[]>([])
  const [loading, setLoading] = useState(true)
  const [creating, setCreating] = useState(false)
  const [deleting, setDeleting] = useState<number | null>(null)
  const [showNewToken, setShowNewToken] = useState(false)
  const [newTokenValue, setNewTokenValue] = useState('')
  const [showTokenValue, setShowTokenValue] = useState(false)
  const [form, setForm] = useState<CreateTokenPayload>({
    name: '',
    role: 'user',
  })
  const [error, setError] = useState('')
  const [success, setSuccess] = useState('')

  useEffect(() => {
    loadTokens()
  }, [])

  const loadTokens = async () => {
    try {
      setLoading(true)
      const response = await api.get('/api/tokens/list')
      setTokens(response.data)
    } catch (err) {
      setError('加载Token列表失败')
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const handleCreateToken = async () => {
    setError('')
    setSuccess('')

    if (!form.name.trim()) {
      setError('请输入Token名称')
      return
    }

    try {
      setCreating(true)
      const response = await api.post('/api/tokens/create', form)
      setNewTokenValue(response.data.plaintext)
      setShowNewToken(true)
      setForm({ name: '', role: 'user' })
      await loadTokens()
      setSuccess('Token创建成功')
    } catch (err) {
      setError('创建Token失败')
      console.error(err)
    } finally {
      setCreating(false)
    }
  }

  const handleDeleteToken = async (id: number) => {
    if (!window.confirm('确定要删除这个Token吗？')) {
      return
    }

    try {
      setDeleting(id)
      await api.delete(`/api/tokens/${id}`)
      await loadTokens()
      setSuccess('Token删除成功')
    } catch (err) {
      setError('删除Token失败')
      console.error(err)
    } finally {
      setDeleting(null)
    }
  }

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
    setSuccess('已复制到剪贴板')
    setTimeout(() => setSuccess(''), 2000)
  }

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString('zh-CN')
  }

  return (
    <div className="space-y-4 md:space-y-6 p-3 md:p-6">
      <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
        <div>
          <h1 className="text-2xl md:text-3xl font-bold">用户管理</h1>
          <p className="text-xs md:text-sm text-muted-foreground mt-1">管理API令牌和用户权限</p>
        </div>
        <Dialog open={showNewToken} onOpenChange={setShowNewToken}>
          <DialogTrigger asChild>
            <Button
              onClick={() => setShowNewToken(false)}
              className="flex flex-col sm:flex-row gap-2 w-full sm:w-auto text-xs md:text-sm"
            >
              <Plus className="w-3 h-3 md:w-4 md:h-4" />
              <span>创建Token</span>
            </Button>
          </DialogTrigger>
          <DialogContent className="w-full max-w-md">
            <DialogHeader>
              <DialogTitle>创建新Token</DialogTitle>
              <DialogDescription>创建一个新的API令牌以供使用</DialogDescription>
            </DialogHeader>
            <div className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="token-name">Token名称</Label>
                <Input
                  id="token-name"
                  placeholder="例如：我的应用"
                  value={form.name}
                  onChange={(e) => setForm({ ...form, name: e.target.value })}
                  disabled={creating}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="token-role">角色</Label>
                <select
                  id="token-role"
                  className="w-full px-3 py-2 border border-input rounded-md text-sm"
                  value={form.role}
                  onChange={(e) => setForm({ ...form, role: e.target.value })}
                  disabled={creating}
                >
                  <option value="user">普通用户</option>
                  <option value="admin">管理员</option>
                </select>
              </div>
              {error && (
                <div className="p-2 md:p-3 bg-destructive/10 border border-destructive/30 rounded text-destructive text-xs md:text-sm">
                  {error}
                </div>
              )}
              <Button
                onClick={handleCreateToken}
                disabled={creating}
                className="w-full text-xs md:text-sm"
              >
                {creating ? (
                  <>
                    <Loader2 className="w-3 h-3 md:w-4 md:h-4 mr-2 animate-spin" />
                    创建中...
                  </>
                ) : (
                  '创建'
                )}
              </Button>
            </div>
          </DialogContent>
        </Dialog>
      </div>

      {newTokenValue && (
        <Card className="border-primary/50 bg-primary/5">
          <CardHeader>
            <CardTitle className="text-base md:text-lg">新Token已创建</CardTitle>
            <CardDescription>请妥善保存，之后无法再次查询</CardDescription>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="flex items-center gap-2 p-2 md:p-3 bg-background rounded border border-muted">
              <code className="text-xs md:text-sm font-mono flex-1 break-all">
                {showTokenValue ? newTokenValue : '•'.repeat(20)}
              </code>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => setShowTokenValue(!showTokenValue)}
              >
                {showTokenValue ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => copyToClipboard(newTokenValue)}
              >
                <Copy className="w-4 h-4" />
              </Button>
            </div>
          </CardContent>
        </Card>
      )}

      {success && (
        <div className="p-3 md:p-4 bg-green-500/10 border border-green-500/30 rounded text-green-600 text-xs md:text-sm">
          {success}
        </div>
      )}

      <Card>
        <CardHeader>
          <CardTitle className="text-base md:text-lg">Token列表</CardTitle>
          <CardDescription>共 {tokens.length} 个Token</CardDescription>
        </CardHeader>
        <CardContent>
          {loading ? (
            <div className="flex items-center justify-center p-8">
              <Loader2 className="w-6 h-6 animate-spin text-muted-foreground" />
            </div>
          ) : tokens.length === 0 ? (
            <div className="text-center p-8">
              <p className="text-muted-foreground text-xs md:text-sm">暂无Token</p>
            </div>
          ) : (
            <div className="space-y-2">
              {tokens.map((token) => (
                <div
                  key={token.id}
                  className="flex flex-col md:flex-row md:items-center md:justify-between gap-3 md:gap-4 p-3 md:p-4 border rounded-lg hover:bg-accent transition-colors"
                >
                  <div className="flex-1 min-w-0">
                    <div className="flex flex-col gap-1">
                      <div className="flex flex-col sm:flex-row sm:items-center gap-2">
                        <h3 className="font-semibold text-xs md:text-sm break-all">{token.name}</h3>
                        <div className="flex flex-wrap gap-2">
                          <span
                            className={`text-xs px-2 py-0.5 rounded-full whitespace-nowrap ${
                              token.role === 'admin'
                                ? 'bg-red-500/20 text-red-700'
                                : 'bg-blue-500/20 text-blue-700'
                            }`}
                          >
                            {token.role === 'admin' ? '管理员' : '普通用户'}
                          </span>
                          <span
                            className={`text-xs px-2 py-0.5 rounded-full ${
                              token.is_active
                                ? 'bg-green-500/20 text-green-700'
                                : 'bg-gray-500/20 text-gray-700'
                            }`}
                          >
                            {token.is_active ? '活跃' : '禁用'}
                          </span>
                        </div>
                      </div>
                      <div className="text-xs text-muted-foreground space-y-0.5">
                        <p>创建于: {formatDate(token.created_at)}</p>
                        {token.last_used_at && (
                          <p>最后使用: {formatDate(token.last_used_at)}</p>
                        )}
                        {token.max_upload_size && (
                          <p>
                            上传配额: {token.used_upload_size} / {token.max_upload_size} 字节
                          </p>
                        )}
                      </div>
                    </div>
                  </div>
                  <Button
                    variant="destructive"
                    size="sm"
                    onClick={() => handleDeleteToken(token.id)}
                    disabled={deleting === token.id}
                    className="w-full sm:w-auto text-xs"
                  >
                    {deleting === token.id ? (
                      <>
                        <Loader2 className="w-3 h-3 mr-1 animate-spin" />
                        删除中...
                      </>
                    ) : (
                      <>
                        <Trash2 className="w-3 h-3 mr-1" />
                        删除
                      </>
                    )}
                  </Button>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
