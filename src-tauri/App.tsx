import React, { useState, useEffect } from 'react'
import { useAuth } from '@/hooks/useAuth'
import { Layout } from '@/components/Layout'
import { Login } from '@/pages/Login'
import { Home } from '@/pages/Home'
import { Gallery } from '@/pages/Gallery'
import { CacheManagement } from '@/pages/CacheManagement'
import { Settings } from '@/pages/Settings'
import { UserManagement } from '@/pages/UserManagement'
import { Loader2 } from 'lucide-react'
import '@/styles/globals.css'

type Page = 'home' | 'gallery' | 'cache' | 'settings' | 'users'

export default function App() {
  const { isAuthenticated, authRequired, loading } = useAuth()
  const [currentPage, setCurrentPage] = useState<Page>('home')

  if (loading) {
    return (
      <div className="flex items-center justify-center h-screen bg-background">
        <Loader2 className="w-8 h-8 animate-spin text-primary" />
      </div>
    )
  }

  if (authRequired && !isAuthenticated) {
    return <Login onLoginSuccess={() => window.location.reload()} />
  }

  // 获取用户角色，默认为普通用户
  const userRole = localStorage.getItem('user_role') || 'user'
  const isAdmin = userRole === 'admin'

  // 普通用户只能访问首页和图片库
  if (!isAdmin && currentPage !== 'home' && currentPage !== 'gallery') {
    return (
      <Layout currentPage={currentPage} onPageChange={setCurrentPage}>
        <div className="flex items-center justify-center h-screen">
          <div className="text-center">
            <h2 className="text-2xl font-bold mb-4">访问被拒绝</h2>
            <p className="text-muted-foreground mb-4">您没有权限访问此页面</p>
          </div>
        </div>
      </Layout>
    )
  }

  const renderPage = () => {
    switch (currentPage) {
      case 'home':
        return <Home />
      case 'gallery':
        return <Gallery />
      case 'cache':
        return isAdmin ? <CacheManagement /> : <Home />
      case 'settings':
        return isAdmin ? <Settings /> : <Home />
      case 'users':
        return isAdmin ? <UserManagement /> : <Home />
      default:
        return <Home />
    }
  }

  return (
    <Layout currentPage={currentPage} onPageChange={setCurrentPage}>
      {renderPage()}
    </Layout>
  )
}
