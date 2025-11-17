const API_BASE_URL = 'http://localhost:3000'

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
    const response = await fetch(`${API_BASE_URL}/api/auth/config`)
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

  const response = await fetch(`${API_BASE_URL}/upload`, {
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

  const response = await fetch(`${API_BASE_URL}/api/images?${params}`, {
    headers: getHeaders(),
  })

  if (!response.ok) {
    throw new Error(`Failed to fetch images: ${response.statusText}`)
  }

  return response.json()
}

export async function getCacheStats(): Promise<any> {
  const response = await fetch(`${API_BASE_URL}/api/cache/stats`, {
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

  const response = await fetch(`${API_BASE_URL}/api/cache/clean`, {
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
  const response = await fetch(`${API_BASE_URL}/health`)

  if (!response.ok) {
    throw new Error(`Failed to fetch health: ${response.statusText}`)
  }

  return response.json()
}
