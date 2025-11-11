let currentPage = 1;
const pageSize = 20;
let hasMore = true;
let loading = false;

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
    
    loadImages();
    loadStats();
});

async function loadImages() {
    if (loading || !hasMore) return;
    
    loading = true;
    document.getElementById('loading').style.display = 'block';
    
    try {
        const response = await fetch(`/api/images/query?page=${currentPage}&size=${pageSize}`, {
            headers: getAuthHeaders()
        });
        const result = await response.json();
        
        if (result.success && result.data.images.length > 0) {
            displayImages(result.data.images, currentPage === 1);
            updateStats(result.data.images, result.data.total);
            hasMore = result.data.images.length === pageSize;
            
            document.getElementById('load-more').style.display = hasMore ? 'block' : 'none';
        } else {
            if (currentPage === 1) {
                showNoImages();
            }
            hasMore = false;
        }
    } catch (error) {
        showError('加载图片失败: ' + error.message);
    } finally {
        loading = false;
        document.getElementById('loading').style.display = 'none';
    }
}

function displayImages(images, replace = false) {
    const masonry = document.getElementById('masonry');
    const noImages = document.getElementById('no-images');
    
    if (replace) {
        masonry.innerHTML = '';
    }
    
    masonry.style.display = 'grid';
    noImages.style.display = 'none';
    
    images.forEach(image => {
        const item = createImageItem(image);
        masonry.appendChild(item);
    });
}

function createImageItem(image) {
    const div = document.createElement('div');
    div.className = 'image-item';
    div.onclick = () => openModal(image.id);
    
    const thumbnailUrl = `/images/${image.id}@w200_h150`;
    
    div.innerHTML = `
        <img src="${thumbnailUrl}" alt="${image.original_name}" loading="lazy">
        <div class="image-info">
            <div class="image-name" title="${image.original_name}">${image.original_name}</div>
            <div class="image-meta">
                ${formatSize(image.size)} • ${image.mime_type} • ${new Date(image.created_at).toLocaleDateString()}
            </div>
        </div>
    `;
    
    return div;
}

function loadMoreImages() {
    currentPage++;
    loadImages();
}

function showNoImages() {
    document.getElementById('loading').style.display = 'none';
    document.getElementById('masonry').style.display = 'none';
    document.getElementById('no-images').style.display = 'block';
    document.getElementById('load-more').style.display = 'none';
}

function showError(message) {
    const errorDiv = document.createElement('div');
    errorDiv.className = 'error';
    errorDiv.textContent = message;
    errorDiv.style.margin = '20px 0';
    
    const cardContent = document.querySelector('.card-content');
    cardContent.insertBefore(errorDiv, cardContent.firstChild);
    
    setTimeout(() => {
        errorDiv.remove();
    }, 5000);
}

function formatSize(bytes) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

function updateStats(images, total) {
    const currentCount = document.getElementById('masonry').children.length;
    const totalSize = images.reduce((sum, img) => sum + img.size, 0);
    const avgSize = images.length > 0 ? totalSize / images.length : 0;
    
    document.getElementById('total-count').textContent = total;
    document.getElementById('current-count').textContent = currentCount;
    document.getElementById('avg-size').textContent = formatSize(avgSize);
}

async function loadStats() {
    try {
        const response = await fetch('/api/stats', {
            headers: getAuthHeaders()
        });
        const result = await response.json();
        
        if (result.success) {
            document.getElementById('total-count').textContent = result.data.total_images;
        }
    } catch (error) {
        console.error('加载统计信息失败:', error);
    }
}

function openModal(imageId) {
    const modal = document.getElementById('image-modal');
    const modalImage = document.getElementById('modal-image');
    
    modalImage.src = `/images/${imageId}`;
    modal.style.display = 'flex';
}

function closeModal() {
    const modal = document.getElementById('image-modal');
    modal.style.display = 'none';
}

// ESC 键关闭模态框
document.addEventListener('keydown', function(e) {
    if (e.key === 'Escape') {
        closeModal();
    }
});

// 点击背景关闭模态框
document.getElementById('image-modal').addEventListener('click', function(e) {
    if (e.target === this) {
        closeModal();
    }
});