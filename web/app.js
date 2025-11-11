document.addEventListener('DOMContentLoaded', async function() {
    // 检查是否需要认证
    const authRequired = await checkAuthRequired();
    if (authRequired && !localStorage.getItem('auth_token')) {
        // 如果需要认证但未登录，跳转到登录页面
        window.location.href = '/login';
        return;
    }
    
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
        let html = '<div class="success">上传成功！</div>';
        
        responses.forEach(({ file, result }) => {
            const imageUrl = `${window.location.origin}/images/${result.id}`;
            html += `
                <div class="success">
                    <strong>${file}</strong><br>
                    <small>ID: ${result.id}</small><br>
                    <a href="${imageUrl}" target="_blank">${imageUrl}</a>
                </div>
            `;
        });

        uploadResult.innerHTML = html;

        if (autoCopy.checked && responses.length === 1) {
            const url = `${window.location.origin}/images/${responses[0].result.id}`;
            copyToClipboard(url);
        }
    }

    function showError(message) {
        uploadResult.innerHTML = `<div class="error">${message}</div>`;
    }

    function copyToClipboard(text) {
        navigator.clipboard.writeText(text).then(() => {
            showToast('链接已复制到剪贴板');
        }).catch(() => {
            showToast('复制失败，请手动复制');
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