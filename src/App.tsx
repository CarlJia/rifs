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

  const renderPage = () => {
    switch (currentPage) {
      case 'home':
        return <Home />
      case 'gallery':
        return <Gallery />
      case 'cache':
        return <CacheManagement />
      case 'settings':
        return <Settings />
      case 'users':
        return <UserManagement />
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
