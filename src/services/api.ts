// 获取 API 基础地址，支持环境变量和本地存储配置
function getApiBaseUrl(): string {
  // 首先检查本地存储中的配置
  const storedUrl = localStorage.getItem('api_base_url')
  if (storedUrl) {
    return storedUrl
  }

  // 其次检查环境变量
  const envUrl = import.meta.env.VITE_API_BASE_URL
  if (envUrl) {
    return envUrl
  }

  // 默认值
  return 'http://localhost:3000'
}

export function setApiBaseUrl(url: string) {
  localStorage.setItem('api_base_url', url)
}

let authToken: string | null = null
let headerName: string = 'Authorization'

export function setAuthToken(token: string | null, name: string = 'Authorization') {
  authToken = token
  headerName = name
}

function getHeaders(includeAuth = true): HeadersInit {
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
  }

  if (includeAuth && authToken) {
    headers[headerName] = `Bearer ${authToken}`
  }

  return headers
}

export async function checkAuthRequired(): Promise<boolean> {
  try {
    const response = await fetch(`${getApiBaseUrl()}/api/auth/config`)
    const result = await response.json()
    return result.enabled
  } catch (error) {
    console.error('Failed to check auth config:', error)
    return false
  }
}

export async function uploadImage(file: File): Promise<any> {
  const formData = new FormData()
  formData.append('file', file)

  const headers: HeadersInit = {}
  if (authToken) {
    headers[headerName] = `Bearer ${authToken}`
  }

  const response = await fetch(`${getApiBaseUrl()}/upload`, {
    method: 'POST',
    headers,
    body: formData,
  })

  if (!response.ok) {
    throw new Error(`Upload failed: ${response.statusText}`)
  }

  return response.json()
}

export async function getGalleryImages(offset: number = 0, limit: number = 32): Promise<any> {
  const params = new URLSearchParams({
    offset: offset.toString(),
    limit: limit.toString(),
  })

  const response = await fetch(`${getApiBaseUrl()}/api/images?${params}`, {
    headers: getHeaders(),
  })

  if (!response.ok) {
    throw new Error(`Failed to fetch images: ${response.statusText}`)
  }

  return response.json()
}

export async function getCacheStats(): Promise<any> {
  const response = await fetch(`${getApiBaseUrl()}/api/cache/stats`, {
    headers: getHeaders(),
  })

  if (!response.ok) {
    throw new Error(`Failed to fetch cache stats: ${response.statusText}`)
  }

  return response.json()
}

export async function cleanCache(maxAge?: number, maxSize?: number): Promise<any> {
  const body: any = {}
  if (maxAge !== undefined) body.max_age = maxAge
  if (maxSize !== undefined) body.max_size = maxSize

  const response = await fetch(`${getApiBaseUrl()}/api/cache/clean`, {
    method: 'POST',
    headers: getHeaders(),
    body: JSON.stringify(body),
  })

  if (!response.ok) {
    throw new Error(`Failed to clean cache: ${response.statusText}`)
  }

  return response.json()
}

export async function getHealthStatus(): Promise<any> {
  const response = await fetch(`${getApiBaseUrl()}/health`)

  if (!response.ok) {
    throw new Error(`Failed to fetch health: ${response.statusText}`)
  }

  return response.json()
}
