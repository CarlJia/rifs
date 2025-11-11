document.addEventListener('DOMContentLoaded', async function() {
    const loginForm = document.getElementById('login-form');
    const tokenInput = document.getElementById('token');
    const loginBtn = document.getElementById('login-btn');
    const messageDiv = document.getElementById('message');

    // 检查是否需要认证
    const authRequired = await checkAuthRequired();
    if (!authRequired) {
        // 如果不需要认证，直接跳转到主页
        window.location.href = '/';
        return;
    }

    // 检查是否已经登录 - 但只在首次加载时检查一次
    if (!sessionStorage.getItem('login_check_done')) {
        sessionStorage.setItem('login_check_done', 'true');
        checkAuthStatus();
    }

    loginForm.addEventListener('submit', async function(e) {
        e.preventDefault();
        
        const token = tokenInput.value.trim();
        if (!token) {
            showMessage('请输入认证令牌', 'error');
            return;
        }

        loginBtn.disabled = true;
        loginBtn.innerHTML = '<span class="loading"></span>验证中...';

        try {
            // 验证 token
            const response = await fetch('/api/auth/verify', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ token })
            });

            const result = await response.json();

            if (result.success) {
                // 保存 token 到 localStorage
                localStorage.setItem('auth_token', token);
                localStorage.setItem('auth_header_name', result.header_name || 'Authorization');
                
                showMessage('登录成功，正在跳转...', 'success');
                
                // 立即跳转到主页 - app.js 会正确显示主页面
                window.location.href = '/';
            } else {
                showMessage(result.message || '认证失败', 'error');
            }
        } catch (error) {
            showMessage('网络错误，请稍后重试', 'error');
        } finally {
            loginBtn.disabled = false;
            loginBtn.innerHTML = '登录';
        }
    });

    function showMessage(message, type) {
        messageDiv.style.display = 'block';
        messageDiv.className = type === 'error' ? 'error-message' : 'success-message';
        messageDiv.textContent = message;
        
        // 5秒后自动隐藏
        setTimeout(() => {
            messageDiv.style.display = 'none';
        }, 5000);
    }
});

// 检查认证状态
function checkAuthStatus() {
    const token = localStorage.getItem('auth_token');
    if (token) {
        // 如果已有 token，验证是否仍然有效
        verifyStoredToken(token);
    }
}

// 验证存储的 token
async function verifyStoredToken(token) {
    try {
        const response = await fetch('/api/auth/verify', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ token })
        });

        if (response.ok) {
            const result = await response.json();
            if (result.success) {
                // token 有效，不再重定向到 "/" 而是保留在登录页
                // 用户需要手动刷新或点击链接来进入主页
            }
        }
    } catch (error) {
        // 验证失败，清除无效 token
        localStorage.removeItem('auth_token');
        localStorage.removeItem('auth_header_name');
    }
}

// 退出登录
function logout() {
    localStorage.removeItem('auth_token');
    localStorage.removeItem('auth_header_name');
    window.location.href = '/login';
}

// 检查是否需要认证
function requiresAuth() {
    const token = localStorage.getItem('auth_token');
    return !token;
}

// 检查是否需要认证（从后端获取配置）
async function checkAuthRequired() {
    try {
        const response = await fetch('/api/auth/config');
        const result = await response.json();
        return result.enabled;
    } catch (error) {
        // 如果获取配置失败，默认不需要认证
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
