import React, { useState } from 'react'
import { useAuth } from '@/hooks/useAuth'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Menu, LogOut, Home, Image, Settings, Sliders, Users } from 'lucide-react'
import { cn } from '@/lib/utils'

interface LayoutProps {
  children: React.ReactNode
  currentPage: 'home' | 'gallery' | 'cache' | 'settings' | 'users'
  onPageChange: (page: 'home' | 'gallery' | 'cache' | 'settings' | 'users') => void
}

export function Layout({ children, currentPage, onPageChange }: LayoutProps) {
  const { logout, isAuthenticated, authRequired } = useAuth()
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const [isMobile, setIsMobile] = useState(window.innerWidth < 768)

  React.useEffect(() => {
    const handleResize = () => {
      const mobile = window.innerWidth < 768
      setIsMobile(mobile)
      if (mobile) {
        setSidebarOpen(false)
      }
    }

    window.addEventListener('resize', handleResize)
    return () => window.removeEventListener('resize', handleResize)
  }, [])

  const handleLogout = () => {
    logout()
    window.location.reload()
  }

  const handleMenuClick = (page: typeof currentPage) => {
    onPageChange(page)
    if (isMobile) {
      setSidebarOpen(false)
    }
  }

  const { userRole } = useAuth()

  const baseMenuItems = [
    { id: 'home', label: '首页', icon: Home, requiredRole: 'user' as const },
    { id: 'gallery', label: '图片库', icon: Image, requiredRole: 'user' as const },
    { id: 'cache', label: '缓存管理', icon: Settings, requiredRole: 'admin' as const },
    { id: 'users', label: '用户管理', icon: Users, requiredRole: 'admin' as const },
    { id: 'settings', label: '系统设置', icon: Sliders, requiredRole: 'admin' as const },
  ] as const

  const menuItems = baseMenuItems.filter(item => {
    if (item.requiredRole === 'admin') {
      return userRole === 'admin'
    }
    return true
  })

  return (
    <div className="flex h-screen bg-background overflow-hidden">
      {/* Mobile Overlay */}
      {isMobile && sidebarOpen && (
        <div
          className="fixed inset-0 bg-black/50 z-40"
          onClick={() => setSidebarOpen(false)}
        />
      )}

      {/* Sidebar */}
      <aside
        className={cn(
          'bg-card border-r border-border transition-all duration-300 flex flex-col',
          isMobile
            ? 'fixed left-0 top-0 h-full w-64 z-50'
            : 'w-64',
          !sidebarOpen && 'hidden md:flex'
        )}
      >
        <div className="p-4 md:p-6 border-b border-border">
          <h1 className="text-xl md:text-2xl font-bold bg-gradient-to-r from-cyan-500 to-blue-500 bg-clip-text text-transparent">
            RIFS
          </h1>
          <p className="text-xs text-muted-foreground mt-1">图床服务</p>
        </div>

        <nav className="p-3 md:p-4 space-y-1 md:space-y-2 flex-1">
          {menuItems.map(({ id, label, icon: Icon }) => (
            <button
              key={id}
              onClick={() => handleMenuClick(id as typeof currentPage)}
              className={cn(
                'w-full flex items-center gap-3 px-3 md:px-4 py-2 md:py-2 rounded-lg transition-colors text-sm md:text-base',
                currentPage === id
                  ? 'bg-primary text-primary-foreground'
                  : 'text-muted-foreground hover:text-foreground hover:bg-muted'
              )}
            >
              <Icon className="w-5 h-5 md:w-5 md:h-5 flex-shrink-0" />
              <span className="truncate">{label}</span>
            </button>
          ))}
        </nav>

        {authRequired && isAuthenticated && (
          <div className="p-3 md:p-4 border-t border-border">
            <Button
              onClick={handleLogout}
              variant="outline"
              className="w-full text-xs md:text-sm"
              size="sm"
            >
              <LogOut className="w-4 h-4 mr-2" />
              <span className="hidden sm:inline">退出</span>
            </Button>
          </div>
        )}
      </aside>

      {/* Main Content */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Header */}
        <header className="bg-card border-b border-border px-3 md:px-6 py-3 md:py-4 flex items-center justify-between shrink-0">
          <button
            onClick={() => setSidebarOpen(!sidebarOpen)}
            className="p-2 hover:bg-muted rounded-lg transition-colors md:hidden"
            aria-label="切换菜单"
          >
            <Menu className="w-6 h-6" />
          </button>

          <div className="hidden md:block text-lg font-semibold text-foreground">
            RIFS 图床服务
          </div>

          <div className="flex items-center gap-2 md:gap-4">
            {authRequired && isAuthenticated && (
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" size="sm" className="text-xs md:text-sm">
                    账户
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem onClick={handleLogout}>
                    <LogOut className="w-4 h-4 mr-2" />
                    退出登录
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            )}
          </div>
        </header>

        {/* Content */}
        <main className="flex-1 overflow-auto">
          <div className="w-full max-w-7xl mx-auto px-3 md:px-6 py-4 md:py-8">
            {children}
          </div>
        </main>
      </div>
    </div>
  )
}
