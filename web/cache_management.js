document.addEventListener('DOMContentLoaded', async function() {
    // 检查是否需要认证
    const authRequired = await checkAuthRequired();
    if (authRequired && !localStorage.getItem('auth_token')) {
        window.location.href = '/login';
        return;
    }
    
    // 如果已登录，显示认证信息
    if (localStorage.getItem('auth_token')) {
        document.getElementById('auth-info').style.display = 'block';
    }
    
    refreshStats();
});

async function refreshStats() {
    try {
        const response = await fetch('/api/cache/stats', {
            headers: getAuthHeaders()
        });
        const result = await response.json();
        
        if (result.success && result.data) {
            const stats = result.data;
            
            document.getElementById('cache-count').textContent = stats.count || 0;
            document.getElementById('cache-size').textContent = formatSize(stats.total_size || 0);
            document.getElementById('hit-rate').textContent = `${(stats.hit_rate || 0).toFixed(1)}%`;
            
            if (stats.oldest_cache) {
                const oldestDate = new Date(stats.oldest_cache);
                document.getElementById('oldest-cache').textContent = formatRelativeTime(oldestDate);
            } else {
                document.getElementById('oldest-cache').textContent = '无';
            }
            
            // 更新使用率进度条
            const usagePercent = Math.min((stats.usage_percent || 0), 100);
            document.getElementById('usage-bar').style.width = `${usagePercent}%`;
            document.getElementById('usage-text').textContent = `使用率: ${usagePercent.toFixed(1)}%`;
            
            // 根据使用率改变颜色
            const usageBar = document.getElementById('usage-bar');
            if (usagePercent > 80) {
                usageBar.style.background = 'linear-gradient(90deg, #ef4444, #dc2626)';
            } else if (usagePercent > 60) {
                usageBar.style.background = 'linear-gradient(90deg, #f59e0b, #d97706)';
            } else {
                usageBar.style.background = 'linear-gradient(90deg, #06b6d4, #3b82f6)';
            }
        }
    } catch (error) {
        logMessage(`获取统计信息失败: ${error.message}`, 'error');
    }
}

async function autoCleanup() {
    if (!confirm('确定要执行自动清理吗？这将删除过期和低热度的缓存文件。')) {
        return;
    }
    
    logMessage('开始自动清理...', 'info');
    
    try {
        const response = await fetch('/api/cache/cleanup/auto', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                ...getAuthHeaders()
            }
        });
        
        const result = await response.json();
        
        if (result.success) {
            logMessage(`自动清理完成: ${result.message}`, 'success');
            if (result.data) {
                logMessage(`清理了 ${result.data.deleted_count} 个文件，释放 ${formatSize(result.data.freed_size)} 空间`, 'info');
            }
        } else {
            logMessage(`自动清理失败: ${result.message}`, 'error');
        }
    } catch (error) {
        logMessage(`自动清理失败: ${error.message}`, 'error');
    }
    
    // 刷新统计信息
    setTimeout(refreshStats, 1000);
}

async function decayHeatScores() {
    if (!confirm('确定要执行热度衰减吗？这将降低所有缓存文件的热度分数。')) {
        return;
    }
    
    logMessage('开始热度衰减...', 'info');
    
    try {
        const response = await fetch('/api/cache/decay', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                ...getAuthHeaders()
            }
        });
        
        const result = await response.json();
        
        if (result.success) {
            logMessage(`热度衰减完成: ${result.message}`, 'success');
            if (result.data) {
                logMessage(`处理了 ${result.data.processed_count} 个文件`, 'info');
            }
        } else {
            logMessage(`热度衰减失败: ${result.message}`, 'error');
        }
    } catch (error) {
        logMessage(`热度衰减失败: ${error.message}`, 'error');
    }
    
    // 刷新统计信息
    setTimeout(refreshStats, 1000);
}

async function clearAllCache() {
    if (!confirm('确定要清空所有缓存吗？此操作不可恢复！')) {
        return;
    }
    
    if (!confirm('再次确认：清空所有缓存将删除所有转换后的图片文件，但不会删除原始图片。')) {
        return;
    }
    
    logMessage('开始清空所有缓存...', 'warning');
    
    try {
        const response = await fetch('/api/cache/clear', {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
                ...getAuthHeaders()
            }
        });
        
        const result = await response.json();
        
        if (result.success) {
            logMessage(`清空缓存完成: ${result.message}`, 'success');
            if (result.data) {
                logMessage(`删除了 ${result.data.deleted_count} 个文件，释放 ${formatSize(result.data.freed_size)} 空间`, 'info');
            }
        } else {
            logMessage(`清空缓存失败: ${result.message}`, 'error');
        }
    } catch (error) {
        logMessage(`清空缓存失败: ${error.message}`, 'error');
    }
    
    // 刷新统计信息
    setTimeout(refreshStats, 1000);
}

function logMessage(message, type = 'info') {
    const logArea = document.getElementById('operation-log');
    logArea.style.display = 'block';
    
    const timestamp = new Date().toLocaleTimeString();
    const prefix = {
        'info': 'ℹ️',
        'success': '✅',
        'warning': '⚠️',
        'error': '❌'
    }[type] || 'ℹ️';
    
    logArea.textContent += `[${timestamp}] ${prefix} ${message}\n`;
    logArea.scrollTop = logArea.scrollHeight;
}

function formatSize(bytes) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

function formatRelativeTime(date) {
    const now = new Date();
    const diffMs = now - date;
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMins / 60);
    const diffDays = Math.floor(diffHours / 24);
    
    if (diffMins < 1) return '刚刚';
    if (diffMins < 60) return `${diffMins}分钟前`;
    if (diffHours < 24) return `${diffHours}小时前`;
    if (diffDays < 30) return `${diffDays}天前`;
    
    return date.toLocaleDateString();
}

// 自动刷新统计信息
setInterval(refreshStats, 30000); // 每30秒刷新一次

// 退出登录
function logout() {
    localStorage.removeItem('auth_token');
    localStorage.removeItem('auth_header_name');
    window.location.href = '/login';
}

// 检查是否需要认证（从后端获取配置）
async function checkAuthRequired() {
    try {
        const response = await fetch('/api/auth/config');
        const result = await response.json();
        return result.enabled;
    } catch (error) {
        return false;
    }
}

// 获取认证头
function getAuthHeaders() {
    const token = localStorage.getItem('auth_token');
    const headerName = localStorage.getItem('auth_header_name') || 'Authorization';
    
    if (token) {
        return {
            [headerName]: `Bearer ${token}`
        };
    }
    return {};
}