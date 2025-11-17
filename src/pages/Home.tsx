import React, { useState } from 'react'
import { uploadImage } from '@/services/api'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Progress } from '@/components/ui/progress'
import { Upload, Copy, Eye, Check } from 'lucide-react'

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
        const url = `${window.location.origin}/images/${hash}`

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
        <div className="space-y-4">
          <h3 className="text-lg font-semibold">上传结果</h3>
          {results.map((result, index) => (
            <Card key={index}>
              <CardContent className="pt-6">
                <div className="space-y-4">
                  <div>
                    <p className="font-semibold">{result.filename}</p>
                    <p className="text-sm text-muted-foreground font-mono">
                      {result.hash}
                    </p>
                  </div>

                  <div className="grid grid-cols-1 gap-3">
                    <div>
                      <label className="text-sm font-medium">URL</label>
                      <div className="flex gap-2 mt-2">
                        <Input
                          readOnly
                          value={result.url}
                          className="bg-muted"
                        />
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => copyToClipboard(result.url, index)}
                        >
                          {copiedIndex === index ? (
                            <>
                              <Check className="w-4 h-4 mr-1" />
                              复制成功
                            </>
                          ) : (
                            <>
                              <Copy className="w-4 h-4 mr-1" />
                              复制
                            </>
                          )}
                        </Button>
                        <Button
                          size="sm"
                          variant="outline"
                          asChild
                        >
                          <a
                            href={result.url}
                            target="_blank"
                            rel="noopener noreferrer"
                          >
                            <Eye className="w-4 h-4" />
                          </a>
                        </Button>
                      </div>
                    </div>

                    <div>
                      <label className="text-sm font-medium">Markdown</label>
                      <div className="flex gap-2 mt-2">
                        <Input
                          readOnly
                          value={result.markdown}
                          className="bg-muted"
                        />
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => copyToClipboard(result.markdown, index)}
                        >
                          <Copy className="w-4 h-4" />
                        </Button>
                      </div>
                    </div>

                    <div>
                      <label className="text-sm font-medium">HTML</label>
                      <div className="flex gap-2 mt-2">
                        <Input
                          readOnly
                          value={result.html}
                          className="bg-muted"
                        />
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => copyToClipboard(result.html, index)}
                        >
                          <Copy className="w-4 h-4" />
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
          <CardTitle>API 端点</CardTitle>
          <CardDescription>快速参考</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div>
            <span className="inline-block bg-green-600 text-white text-xs font-semibold px-2 py-1 rounded mr-2">
              POST
            </span>
            <span className="font-mono">/upload</span>
            <p className="text-sm text-muted-foreground mt-1">上传图片文件</p>
          </div>

          <div>
            <span className="inline-block bg-blue-600 text-white text-xs font-semibold px-2 py-1 rounded mr-2">
              GET
            </span>
            <span className="font-mono">/images/{'{id}'}</span>
            <p className="text-sm text-muted-foreground mt-1">获取原图或转换图片</p>
          </div>

          <div>
            <span className="inline-block bg-blue-600 text-white text-xs font-semibold px-2 py-1 rounded mr-2">
              GET
            </span>
            <span className="font-mono">/images/{'{id}'}@w800_h600_jpeg_q90</span>
            <p className="text-sm text-muted-foreground mt-1">
              实时转换：宽800px、高600px、JPEG格式、质量90%
            </p>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>转换参数</CardTitle>
          <CardDescription>在图片URL后添加参数进行实时转换</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 gap-4">
            <div className="p-3 bg-muted rounded">
              <code className="font-mono text-sm">w{'{数字}'}</code>
              <p className="text-xs text-muted-foreground mt-1">最大宽度</p>
            </div>
            <div className="p-3 bg-muted rounded">
              <code className="font-mono text-sm">h{'{数字}'}</code>
              <p className="text-xs text-muted-foreground mt-1">最大高度</p>
            </div>
            <div className="p-3 bg-muted rounded">
              <code className="font-mono text-sm">jpeg/png/webp/avif</code>
              <p className="text-xs text-muted-foreground mt-1">目标格式</p>
            </div>
            <div className="p-3 bg-muted rounded">
              <code className="font-mono text-sm">q{'{1-100}'}</code>
              <p className="text-xs text-muted-foreground mt-1">质量参数</p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
