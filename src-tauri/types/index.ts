export interface UploadResponse {
  code: number
  message: string
  data: {
    hash: string
    extension: string
    size: number
  }
}

export interface ImageItem {
  id: string
  hash: string
  filename: string
  url: string
}

export interface CacheStats {
  total_size: number
  total_count: number
  items: CacheItem[]
}

export interface CacheItem {
  hash: string
  format: string
  file_size: number
  last_accessed: string
}

export interface GalleryResponse {
  items: GalleryImage[]
  total_count: number
}

export interface GalleryImage {
  hash: string
  extension: string
  size: number
  width?: number
  height?: number
}

export interface AppConfig {
  auth: {
    enabled: boolean
  }
}
