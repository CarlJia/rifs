import React, { useState } from 'react'
import { uploadImage } from '@/services/api'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Progress } from '@/components/ui/progress'
import { Upload, Copy, Eye, Check } from 'lucide-react'

// 获取 API 基础地址函数，与 api.ts 同步
function getApiBaseUrl(): string {
  const storedUrl = localStorage.getItem('api_base_url')
  if (storedUrl) return storedUrl
  const envUrl = import.meta.env.VITE_API_BASE_URL
  if (envUrl) return envUrl
  return 'http://localhost:3000'
}

interface UploadResult {
  filename: string
  hash: string
  url: string
  markdown: string
  html: string
}

export function Home() {
  const [uploading, setUploading] = useState(false)
  const [progress, setProgress] = useState(0)
  const [results, setResults] = useState<UploadResult[]>([])
  const [autoCopy, setAutoCopy] = useState(false)
  const [copiedIndex, setCopiedIndex] = useState<number | null>(null)

  const handleFileUpload = async (files: FileList | null) => {
    if (!files || files.length === 0) return

    setUploading(true)
    setProgress(0)
    const newResults: UploadResult[] = []

    try {
      for (let i = 0; i < files.length; i++) {
        const file = files[i]
        const result = await uploadImage(file)
        const hash = result.data.hash
        const url = `${getApiBaseUrl()}/images/${hash}`

        newResults.push({
          filename: file.name,
          hash,
          url,
          markdown: `![${file.name}](${url})`,
          html: `<img src='${url}' alt='${file.name}' />`,
        })

        setProgress(((i + 1) / files.length) * 100)
      }

      setResults(newResults)

      if (autoCopy && newResults.length === 1) {
        copyToClipboard(newResults[0].url)
      }
    } catch (error) {
      console.error('Upload failed:', error)
      alert('Upload failed')
    } finally {
      setUploading(false)
      setProgress(0)
    }
  }

  const copyToClipboard = (text: string, index?: number) => {
    navigator.clipboard.writeText(text).then(() => {
      if (index !== undefined) {
        setCopiedIndex(index)
        setTimeout(() => setCopiedIndex(null), 2000)
      }
    })
  }

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>图片上传</CardTitle>
          <CardDescription>
            支持 JPG、PNG、GIF、WebP、AVIF、ICO 格式
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="relative">
            <Input
              type="file"
              id="file-input"
              multiple
              accept="image/*"
              onChange={(e) => handleFileUpload(e.target.files)}
              disabled={uploading}
              className="cursor-pointer"
            />
          </div>

          <Label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={autoCopy}
              onChange={(e) => setAutoCopy(e.target.checked)}
              className="rounded"
            />
            <span>单个文件时自动复制链接</span>
          </Label>

          {uploading && (
            <div className="space-y-2">
              <Progress value={progress} />
              <p className="text-sm text-muted-foreground text-right">
                {Math.round(progress)}%
              </p>
            </div>
          )}

          <Button
            onClick={() => document.getElementById('file-input')?.click()}
            disabled={uploading}
            className="w-full"
          >
            <Upload className="w-4 h-4 mr-2" />
            {uploading ? '上传中...' : '选择图片'}
          </Button>
        </CardContent>
      </Card>

      {results.length > 0 && (
        <div className="space-y-3 md:space-y-4">
          <h3 className="text-base md:text-lg font-semibold">上传结果</h3>
          {results.map((result, index) => (
            <Card key={index}>
              <CardContent className="pt-4 md:pt-6">
                <div className="space-y-3 md:space-y-4">
                  <div>
                    <p className="font-semibold text-sm md:text-base truncate">{result.filename}</p>
                    <p className="text-xs md:text-sm text-muted-foreground font-mono break-all">
                      {result.hash}
                    </p>
                  </div>

                  <div className="grid grid-cols-1 gap-2 md:gap-3">
                    <div>
                      <label className="text-xs md:text-sm font-medium">URL</label>
                      <div className="flex flex-col sm:flex-row gap-2 mt-1 md:mt-2">
                        <Input
                          readOnly
                          value={result.url}
                          className="bg-muted text-xs md:text-sm min-w-0"
                        />
                        <div className="flex gap-2">
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={() => copyToClipboard(result.url, index)}
                            className="text-xs md:text-sm px-2 md:px-3"
                            title="复制 URL"
                          >
                            {copiedIndex === index ? (
                              <Check className="w-3 h-3 md:w-4 md:h-4" />
                            ) : (
                              <Copy className="w-3 h-3 md:w-4 md:h-4" />
                            )}
                            <span className="hidden sm:inline ml-1">
                              {copiedIndex === index ? '成功' : '复制'}
                            </span>
                          </Button>
                          <Button
                            size="sm"
                            variant="outline"
                            asChild
                            className="text-xs md:text-sm px-2 md:px-3"
                            title="查看原图"
                          >
                            <a
                              href={result.url}
                              target="_blank"
                              rel="noopener noreferrer"
                            >
                              <Eye className="w-3 h-3 md:w-4 md:h-4" />
                            </a>
                          </Button>
                        </div>
                      </div>
                    </div>

                    <div>
                      <label className="text-xs md:text-sm font-medium">Markdown</label>
                      <div className="flex flex-col sm:flex-row gap-2 mt-1 md:mt-2">
                        <Input
                          readOnly
                          value={result.markdown}
                          className="bg-muted text-xs md:text-sm min-w-0"
                        />
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => copyToClipboard(result.markdown, index)}
                          className="text-xs md:text-sm px-2 md:px-3"
                          title="复制 Markdown"
                        >
                          <Copy className="w-3 h-3 md:w-4 md:h-4" />
                          <span className="hidden sm:inline ml-1">复制</span>
                        </Button>
                      </div>
                    </div>

                    <div>
                      <label className="text-xs md:text-sm font-medium">HTML</label>
                      <div className="flex flex-col sm:flex-row gap-2 mt-1 md:mt-2">
                        <Input
                          readOnly
                          value={result.html}
                          className="bg-muted text-xs md:text-sm min-w-0"
                        />
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => copyToClipboard(result.html, index)}
                          className="text-xs md:text-sm px-2 md:px-3"
                          title="复制 HTML"
                        >
                          <Copy className="w-3 h-3 md:w-4 md:h-4" />
                          <span className="hidden sm:inline ml-1">复制</span>
                        </Button>
                      </div>
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}

      <Card>
         <CardHeader>
           <CardTitle className="text-base md:text-xl">API 端点</CardTitle>
           <CardDescription className="text-xs md:text-sm">快速参考</CardDescription>
         </CardHeader>
         <CardContent className="space-y-3 md:space-y-4">
           <div>
             <span className="inline-block bg-green-600 text-white text-xs font-semibold px-2 py-1 rounded mr-2">
               POST
             </span>
             <span className="font-mono text-xs md:text-sm break-all">/upload</span>
             <p className="text-xs md:text-sm text-muted-foreground mt-1">上传图片文件</p>
           </div>

           <div>
             <span className="inline-block bg-blue-600 text-white text-xs font-semibold px-2 py-1 rounded mr-2">
               GET
             </span>
             <span className="font-mono text-xs md:text-sm break-all">/images/{'{id}'}</span>
             <p className="text-xs md:text-sm text-muted-foreground mt-1">获取原图或转换图片</p>
           </div>

           <div>
             <span className="inline-block bg-blue-600 text-white text-xs font-semibold px-2 py-1 rounded mr-2">
               GET
             </span>
             <span className="font-mono text-xs md:text-sm break-all">/images/{'{id}'}@w800_h600_jpeg_q90</span>
             <p className="text-xs md:text-sm text-muted-foreground mt-1">
               实时转换：宽800px、高600px、JPEG格式、质量90%
             </p>
           </div>
         </CardContent>
       </Card>

       <Card>
         <CardHeader>
           <CardTitle className="text-base md:text-xl">转换参数</CardTitle>
           <CardDescription className="text-xs md:text-sm">在图片URL后添加参数进行实时转换</CardDescription>
         </CardHeader>
         <CardContent>
           <div className="grid grid-cols-2 md:grid-cols-4 gap-2 md:gap-4">
             <div className="p-2 md:p-3 bg-muted rounded">
               <code className="font-mono text-xs md:text-sm break-all">w{'{数字}'}</code>
               <p className="text-xs text-muted-foreground mt-1">最大宽度</p>
             </div>
             <div className="p-2 md:p-3 bg-muted rounded">
               <code className="font-mono text-xs md:text-sm break-all">h{'{数字}'}</code>
               <p className="text-xs text-muted-foreground mt-1">最大高度</p>
             </div>
             <div className="p-2 md:p-3 bg-muted rounded">
               <code className="font-mono text-xs md:text-sm break-all">jpeg/png/webp/avif</code>
               <p className="text-xs text-muted-foreground mt-1">目标格式</p>
             </div>
             <div className="p-2 md:p-3 bg-muted rounded">
               <code className="font-mono text-xs md:text-sm break-all">q{'{1-100}'}</code>
               <p className="text-xs text-muted-foreground mt-1">质量参数</p>
             </div>
           </div>
         </CardContent>
       </Card>
    </div>
  )
}
