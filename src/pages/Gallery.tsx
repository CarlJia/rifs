import React, { useEffect, useRef, useCallback } from 'react'
import { getGalleryImages } from '@/services/api'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Loader2, RefreshCw } from 'lucide-react'

// 获取 API 基础地址函数，与 api.ts 同步
function getApiBaseUrl(): string {
  const storedUrl = localStorage.getItem('api_base_url')
  if (storedUrl) return storedUrl
  const envUrl = import.meta.env.VITE_API_BASE_URL
  if (envUrl) return envUrl
  return 'http://localhost:3000'
}

interface Image {
  hash: string
  extension: string
  size: number
}

export function Gallery() {
  const [images, setImages] = React.useState<Image[]>([])
  const [loading, setLoading] = React.useState(false)
  const [hasMore, setHasMore] = React.useState(true)
  const [offset, setOffset] = React.useState(0)
  const [totalCount, setTotalCount] = React.useState(0)
  const observerTarget = useRef<HTMLDivElement>(null)

  const loadImages = useCallback(async (newOffset: number = 0) => {
    if (loading) return

    setLoading(true)
    try {
      const result = await getGalleryImages(newOffset, 32)
      const newImages = result.data?.items || result.items || []

      if (newOffset === 0) {
        setImages(newImages)
      } else {
        setImages((prev) => [...prev, ...newImages])
      }

      setTotalCount(result.data?.total_count || result.total_count || 0)
      setHasMore(newImages.length === 32)
      setOffset(newOffset + 32)
    } catch (error) {
      console.error('Failed to load images:', error)
    } finally {
      setLoading(false)
    }
  }, [loading])

  useEffect(() => {
    loadImages(0)
  }, [])

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && hasMore && !loading) {
          loadImages(offset)
        }
      },
      { rootMargin: '100px' }
    )

    if (observerTarget.current) {
      observer.observe(observerTarget.current)
    }

    return () => observer.disconnect()
  }, [hasMore, loading, offset, loadImages])

  const imageUrl = (hash: string, ext: string) =>
    `${getApiBaseUrl()}/images/${hash}`

  const thumbnailUrl = (hash: string) =>
    `${getApiBaseUrl()}/images/${hash}@w400_h200_jpeg_q80`

  return (
    <div className="space-y-3 md:space-y-6">
      <Card>
         <CardHeader className="p-3 md:p-6">
           <CardTitle className="text-base md:text-xl">图片库</CardTitle>
           <CardDescription className="text-xs md:text-sm">
             共 {totalCount} 张图片
           </CardDescription>
         </CardHeader>
         <CardContent className="p-3 md:p-6 pt-0">
           <Button
             onClick={() => loadImages(0)}
             disabled={loading}
             variant="outline"
             className="text-xs md:text-sm"
             size="sm"
           >
             <RefreshCw className={`w-3 h-3 md:w-4 md:h-4 mr-2 ${loading ? 'animate-spin' : ''}`} />
             刷新
           </Button>
         </CardContent>
       </Card>

       {images.length > 0 && (
         <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-2 md:gap-4">
           {images.map((image, index) => (
             <div
               key={`${image.hash}-${index}`}
               className="group relative overflow-hidden rounded-lg bg-muted aspect-video"
             >
               <img
                 src={thumbnailUrl(image.hash)}
                 alt={image.original_filename}
                 className="w-full h-full object-cover transition-transform group-hover:scale-110 group-active:scale-110"
                 loading="lazy"
               />
               <div className="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 group-active:opacity-100 transition-opacity flex items-center justify-center gap-1 md:gap-2">
                 <Button
                   size="sm"
                   variant="secondary"
                   asChild
                   className="text-xs h-7 md:h-8"
                 >
                   <a
                     href={imageUrl(image.hash, image.extension)}
                     target="_blank"
                     rel="noopener noreferrer"
                   >
                     <span className="hidden sm:inline">查看</span>
                     <span className="sm:hidden">看</span>
                   </a>
                 </Button>
               </div>
               <div className="absolute bottom-0 left-0 right-0 bg-black/50 text-white text-xs p-1 md:p-2 truncate">
                 {image.original_filename}...
               </div>
             </div>
           ))}
         </div>
       )}

       {loading && images.length === 0 && (
         <div className="flex items-center justify-center py-8 md:py-12">
           <Loader2 className="w-6 h-6 md:w-8 md:h-8 animate-spin text-muted-foreground" />
         </div>
       )}

       {images.length === 0 && !loading && (
         <div className="text-center py-8 md:py-12">
           <p className="text-sm md:text-base text-muted-foreground">暂无图片</p>
         </div>
       )}

       {hasMore && (
         <div ref={observerTarget} className="flex justify-center py-4 md:py-8">
           {loading && <Loader2 className="w-5 h-5 md:w-6 md:h-6 animate-spin" />}
         </div>
       )}
     </div>
   )
}
