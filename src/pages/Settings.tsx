import React, { useState, useEffect } from 'react'
import { setApiBaseUrl } from '@/services/api'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Save, AlertCircle } from 'lucide-react'

export function Settings() {
  const [apiUrl, setApiUrl] = useState('http://localhost:3000')
  const [saved, setSaved] = useState(false)

  useEffect(() => {
    // 加载当前保存的 API URL
    const storedUrl = localStorage.getItem('api_base_url')
    if (storedUrl) {
      setApiUrl(storedUrl)
    }
  }, [])

  const handleSave = () => {
    if (!apiUrl.trim()) {
      alert('请输入有效的 API 地址')
      return
    }

    try {
      // 验证 URL 格式
      new URL(apiUrl)
      setApiBaseUrl(apiUrl)
      setSaved(true)
      setTimeout(() => setSaved(false), 3000)
    } catch {
      alert('请输入有效的 URL 地址（如 http://localhost:3000）')
    }
  }

  const handleReset = () => {
    const defaultUrl = 'http://localhost:3000'
    setApiUrl(defaultUrl)
    setApiBaseUrl(defaultUrl)
    setSaved(true)
    setTimeout(() => setSaved(false), 3000)
  }

  return (
    <div className="space-y-3 md:space-y-6">
      <Card>
         <CardHeader className="p-3 md:p-6">
           <CardTitle className="text-base md:text-xl">系统设置</CardTitle>
           <CardDescription className="text-xs md:text-sm">配置应用的基本参数</CardDescription>
         </CardHeader>
         <CardContent className="p-3 md:p-6 pt-0 space-y-3 md:space-y-6">
           <div className="space-y-2 md:space-y-4">
             <div>
               <Label htmlFor="api-url" className="text-xs md:text-sm">API 服务器地址</Label>
               <p className="text-xs md:text-sm text-muted-foreground mb-1 md:mb-2">
                 输入后端服务器的地址，格式为 http://host:port 或 https://host
               </p>
               <Input
                 id="api-url"
                 type="text"
                 value={apiUrl}
                 onChange={(e) => setApiUrl(e.target.value)}
                 placeholder="http://localhost:3000"
                 className="font-mono text-xs md:text-sm"
               />
             </div>

             {saved && (
               <div className="p-2 md:p-3 bg-green-900/20 border border-green-700/50 rounded text-green-400 text-xs md:text-sm flex items-center gap-2">
                 <span>✓ 设置已保存</span>
               </div>
             )}

             <div className="p-2 md:p-3 bg-blue-900/20 border border-blue-700/50 rounded text-blue-300 text-xs md:text-sm flex items-start gap-2">
               <AlertCircle className="w-3 h-3 md:w-4 md:h-4 mt-0.5 flex-shrink-0" />
               <span>修改 API 地址后，请刷新页面以应用新配置</span>
             </div>
           </div>

           <div className="flex flex-col sm:flex-row gap-2 md:gap-3">
             <Button onClick={handleSave} className="flex-1 text-xs md:text-sm" size="sm">
               <Save className="w-3 h-3 md:w-4 md:h-4 mr-2" />
               保存设置
             </Button>
             <Button onClick={handleReset} variant="outline" className="flex-1 text-xs md:text-sm" size="sm">
               重置为默认值
             </Button>
           </div>

           <div className="border-t pt-3 md:pt-6">
             <h3 className="font-semibold mb-2 md:mb-4 text-sm md:text-base">当前配置信息</h3>
             <div className="space-y-2 text-xs md:text-sm">
               <div className="flex flex-col sm:flex-row sm:justify-between gap-1">
                 <span className="text-muted-foreground">当前 API 地址：</span>
                 <span className="font-mono break-all text-xs">{apiUrl}</span>
               </div>
               <div className="flex flex-col sm:flex-row sm:justify-between gap-1">
                 <span className="text-muted-foreground">应用版本：</span>
                 <span>0.1.0</span>
               </div>
               <div className="flex flex-col sm:flex-row sm:justify-between gap-1">
                 <span className="text-muted-foreground">框架：</span>
                 <span>Tauri + React 18</span>
               </div>
             </div>
           </div>
         </CardContent>
       </Card>

       <Card>
         <CardHeader className="p-3 md:p-6">
           <CardTitle className="text-base md:text-xl">说明</CardTitle>
         </CardHeader>
         <CardContent className="p-3 md:p-6 pt-0 text-xs md:text-sm text-muted-foreground space-y-1 md:space-y-2">
           <p>• 确保输入的 API 地址是可访问的后端服务器地址</p>
           <p>• 地址应包含协议（http 或 https）</p>
           <p>• 如果后端运行在本地端口 3000，使用 http://localhost:3000</p>
           <p>• 修改后需要刷新页面才能生效</p>
         </CardContent>
       </Card>
     </div>
   )
}
