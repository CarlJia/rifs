import React, { useEffect, useState } from 'react'
import { getCacheStats, cleanCache } from '@/services/api'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Loader2, Trash2, RefreshCw } from 'lucide-react'

interface CacheItem {
  hash: string
  format: string
  file_size: number
  last_accessed: string
}

export function CacheManagement() {
  const [stats, setStats] = useState<{
    total_size: number
    total_count: number
    items: CacheItem[]
  } | null>(null)
  const [loading, setLoading] = useState(false)
  const [cleaning, setCleaning] = useState(false)
  const [maxAge, setMaxAge] = useState('')
  const [maxSize, setMaxSize] = useState('')

  const loadStats = async () => {
    setLoading(true)
    try {
      const result = await getCacheStats()
      setStats(result.data || result)
    } catch (error) {
      console.error('Failed to load cache stats:', error)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadStats()
  }, [])

  const handleClean = async () => {
    setCleaning(true)
    try {
      await cleanCache(
        maxAge ? parseInt(maxAge) : undefined,
        maxSize ? parseInt(maxSize) : undefined
      )
      alert('缓存清理成功')
      await loadStats()
    } catch (error) {
      console.error('Failed to clean cache:', error)
      alert('缓存清理失败')
    } finally {
      setCleaning(false)
    }
  }

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i]
  }

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>缓存统计</CardTitle>
          <CardDescription>查看当前缓存使用情况</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {loading && !stats ? (
            <div className="flex justify-center py-8">
              <Loader2 className="w-8 h-8 animate-spin text-muted-foreground" />
            </div>
          ) : stats ? (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="p-4 bg-muted rounded-lg">
                <p className="text-sm text-muted-foreground">总缓存大小</p>
                <p className="text-2xl font-semibold">
                  {formatBytes(stats.total_size)}
                </p>
              </div>
              <div className="p-4 bg-muted rounded-lg">
                <p className="text-sm text-muted-foreground">缓存文件数</p>
                <p className="text-2xl font-semibold">{stats.total_count}</p>
              </div>
            </div>
          ) : null}

          <Button
            onClick={loadStats}
            disabled={loading}
            variant="outline"
            className="w-full"
          >
            <RefreshCw className={`w-4 h-4 mr-2 ${loading ? 'animate-spin' : ''}`} />
            刷新
          </Button>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>清理缓存</CardTitle>
          <CardDescription>根据条件清理缓存文件</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div>
            <Label htmlFor="max-age">最大文件年龄（秒，留空则不限制）</Label>
            <Input
              id="max-age"
              type="number"
              value={maxAge}
              onChange={(e) => setMaxAge(e.target.value)}
              placeholder="86400"
              className="mt-2"
            />
          </div>

          <div>
            <Label htmlFor="max-size">最大缓存大小（字节，留空则不限制）</Label>
            <Input
              id="max-size"
              type="number"
              value={maxSize}
              onChange={(e) => setMaxSize(e.target.value)}
              placeholder="1073741824"
              className="mt-2"
            />
          </div>

          <Button
            onClick={handleClean}
            disabled={cleaning}
            className="w-full bg-destructive hover:bg-destructive/90"
          >
            <Trash2 className="w-4 h-4 mr-2" />
            {cleaning ? '清理中...' : '清理缓存'}
          </Button>
        </CardContent>
      </Card>

      {stats && stats.items && stats.items.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>缓存详情</CardTitle>
            <CardDescription>最近缓存的文件</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b">
                    <th className="text-left py-2 px-2">哈希</th>
                    <th className="text-left py-2 px-2">格式</th>
                    <th className="text-right py-2 px-2">大小</th>
                    <th className="text-left py-2 px-2">最后访问</th>
                  </tr>
                </thead>
                <tbody>
                  {stats.items.slice(0, 10).map((item, idx) => (
                    <tr key={idx} className="border-b">
                      <td className="py-2 px-2 font-mono text-xs">
                        {item.hash.substring(0, 12)}...
                      </td>
                      <td className="py-2 px-2">{item.format}</td>
                      <td className="py-2 px-2 text-right">
                        {formatBytes(item.file_size)}
                      </td>
                      <td className="py-2 px-2 text-muted-foreground text-xs">
                        {new Date(item.last_accessed).toLocaleString()}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
