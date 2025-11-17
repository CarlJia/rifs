document.addEventListener('DOMContentLoaded', async function() {
    // 检查是否需要认证
    const authRequired = await checkAuthRequired();
    if (authRequired && !localStorage.getItem('auth_token')) {
        // 如果需要认证但未登录，跳转到登录页面
        window.location.href = '/login';
        return;
    }
    
    // 清除登录检查标志，因为现在只在登录页面使用
    sessionStorage.removeItem('login_check_done');
    
    // 如果已登录，显示认证信息
    if (localStorage.getItem('auth_token')) {
        document.getElementById('auth-info').style.display = 'block';
    }
    const uploadForm = document.getElementById('upload-form');
    const fileInput = document.getElementById('file-input');
    const fileLabel = document.querySelector('.file-label');
    const uploadBtn = document.getElementById('upload-btn');
    const uploadResult = document.getElementById('upload-result');
    const autoCopy = document.getElementById('auto-copy');

    // 拖拽上传
    ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
        fileLabel.addEventListener(eventName, preventDefaults, false);
    });

    function preventDefaults(e) {
        e.preventDefault();
        e.stopPropagation();
    }

    ['dragenter', 'dragover'].forEach(eventName => {
        fileLabel.addEventListener(eventName, () => fileLabel.classList.add('drag-over'), false);
    });

    ['dragleave', 'drop'].forEach(eventName => {
        fileLabel.addEventListener(eventName, () => fileLabel.classList.remove('drag-over'), false);
    });

    fileLabel.addEventListener('drop', handleDrop, false);

    function handleDrop(e) {
        const dt = e.dataTransfer;
        const files = dt.files;
        fileInput.files = files;
        updateFileLabel();
    }

    fileInput.addEventListener('change', updateFileLabel);

    function updateFileLabel() {
        const files = fileInput.files;
        if (files.length > 0) {
            const fileNames = Array.from(files).map(f => f.name).join(', ');
            fileLabel.querySelector('div:nth-child(2)').textContent = `已选择 ${files.length} 个文件`;
            fileLabel.querySelector('small').textContent = fileNames;
        }
    }

    uploadForm.addEventListener('submit', async function(e) {
        e.preventDefault();
        
        const files = fileInput.files;
        if (!files.length) {
            showError('请选择要上传的图片');
            return;
        }

        uploadBtn.disabled = true;
        uploadBtn.innerHTML = '<span class="loading"></span>上传中...';
        uploadResult.innerHTML = '';

        try {
            const formData = new FormData();
            for (let file of files) {
                formData.append('file', file);
            }

            const responses = [];
            for (let file of files) {
                const singleFormData = new FormData();
                singleFormData.append('file', file);
                
                const response = await fetch('/upload', {
                    method: 'POST',
                    headers: getAuthHeaders(),
                    body: singleFormData
                });
                
                if (!response.ok) {
                    throw new Error(`上传 ${file.name} 失败: ${response.statusText}`);
                }
                
                const result = await response.json();
                responses.push({ file: file.name, result });
            }

            displayResults(responses);
            
        } catch (error) {
            showError(error.message);
        } finally {
            uploadBtn.disabled = false;
            uploadBtn.textContent = '上传图片';
        }
    });

    function displayResults(responses) {
        let html = '<div class="success">✓ 上传成功！共 ' + responses.length + ' 个文件</div>';
        
        responses.forEach(({ file, result }, index) => {
            const { hash } = result.data;
            const imageUrl = `${window.location.origin}/images/${hash}`;
            const markdownUrl = `![](${imageUrl})`;
            const displayIndex = String(index + 1).padStart(2, '0');
            const safeFileName = escapeHtml(file || '');
            const safeHash = escapeHtml(hash);
            const htmlCode = `<img src="${imageUrl}" alt="${safeFileName}" />`;
            const urlAttr = escapeHtml(imageUrl);
            const markdownAttr = escapeHtml(markdownUrl);
            const htmlAttr = escapeHtml(htmlCode);
            
            html += `
                <div class="upload-item">
                    <div class="upload-item-header">
                        <div class="upload-item-info">
                            <span class="upload-item-chip">图片 #${displayIndex}</span>
                            <div class="upload-item-name">
                                <span class="upload-item-index">${displayIndex}</span>
                                <span title="${safeFileName}">${safeFileName}</span>
                            </div>
                            <div class="upload-item-id">🔑 ${safeHash}</div>
                        </div>
                        <div class="upload-item-actions">
                            <a href="${imageUrl}" target="_blank" rel="noopener noreferrer" class="view-btn">
                                <span>👁️</span>
                                <span>预览</span>
                            </a>
                        </div>
                    </div>
                    <div class="format-tabs">
                        <button class="format-tab active" data-format="url" data-index="${index}">🔗 URL</button>
                        <button class="format-tab" data-format="markdown" data-index="${index}">📝 Markdown</button>
                        <button class="format-tab" data-format="html" data-index="${index}">💻 HTML</button>
                    </div>
                    <div class="format-content" data-content-url="${urlAttr}" data-content-markdown="${markdownAttr}" data-content-html="${htmlAttr}">${urlAttr}</div>
                    <div class="action-buttons">
                        <button class="copy-btn" data-copy-index="${index}" data-copy-format="url">📋 复制 URL</button>
                        <button class="copy-btn" data-copy-index="${index}" data-copy-format="markdown">📋 复制 Markdown</button>
                        <button class="copy-btn" data-copy-index="${index}" data-copy-format="html">📋 复制 HTML</button>
                    </div>
                </div>
            `;
        });

        uploadResult.innerHTML = html;
        
        // 绑定格式切换事件
        document.querySelectorAll('.format-tab').forEach(tab => {
            tab.addEventListener('click', switchFormat);
        });
        
        // 绑定复制按钮事件
        document.querySelectorAll('.copy-btn').forEach(btn => {
            btn.addEventListener('click', handleCopyBtn);
        });

        if (autoCopy.checked && responses.length === 1) {
            const singleHash = responses[0].result.data.hash;
            const url = `${window.location.origin}/images/${singleHash}`;
            copyToClipboard(url);
        }
    }
    
    function switchFormat(e) {
        const format = e.target.dataset.format;
        const index = e.target.dataset.index;
        const container = e.target.closest('.upload-item');
        const content = container.querySelector('.format-content');
        
        // 更新tab状态
        container.querySelectorAll('.format-tab').forEach(tab => {
            tab.classList.remove('active');
        });
        e.target.classList.add('active');
        
        // 更新内容
        let contentText = '';
        switch(format) {
            case 'url':
                contentText = content.dataset.contentUrl;
                break;
            case 'markdown':
                contentText = content.dataset.contentMarkdown;
                break;
            case 'html':
                contentText = content.dataset.contentHtml;
                break;
        }
        content.textContent = contentText;
    }
    
    function handleCopyBtn(e) {
        const format = e.target.dataset.copyFormat;
        const index = e.target.dataset.copyIndex;
        const container = e.target.closest('.upload-item');
        const content = container.querySelector('.format-content');
        
        let contentText = '';
        switch(format) {
            case 'url':
                contentText = content.dataset.contentUrl;
                break;
            case 'markdown':
                contentText = content.dataset.contentMarkdown;
                break;
            case 'html':
                contentText = content.dataset.contentHtml;
                break;
        }
        
        copyToClipboard(contentText, e.target);
    }

    function escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text == null ? '' : text;
        return div.innerHTML;
    }

    function showError(message) {
        uploadResult.innerHTML = `<div class="error">${escapeHtml(message)}</div>`;
    }

    function copyToClipboard(text, button = null) {
        navigator.clipboard.writeText(text).then(() => {
            if (button) {
                const originalText = button.textContent;
                button.classList.add('copied');
                button.textContent = '已复制 ✓';
                setTimeout(() => {
                    button.classList.remove('copied');
                    button.textContent = originalText;
                }, 2000);
            } else {
                showToast('链接已复制到剪贴板');
            }
        }).catch(() => {
            if (button) {
                showToast('复制失败，请手动复制');
            } else {
                showToast('复制失败，请手动复制');
            }
        });
    }

    function showToast(message) {
        const toast = document.createElement('div');
        toast.className = 'success';
        toast.style.position = 'fixed';
        toast.style.top = '20px';
        toast.style.right = '20px';
        toast.style.zIndex = '1000';
        toast.textContent = message;
        document.body.appendChild(toast);
        
        setTimeout(() => {
            toast.remove();
        }, 3000);
    }
});

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
