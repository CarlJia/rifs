let currentPage = 1;
const pageSize = 32; // 每页显示32张图片 (4列 × 8行)
let hasMore = true;
let loading = false;
let totalImages = 0;
let displayedCount = 0;
let imageObserver = null;

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
    
    // 初始化 Intersection Observer 用于懒加载
    initializeImageObserver();
    
    // 初始化无限滚动
    initializeInfiniteScroll();
    
    loadStats();
    loadImages();
});

// 初始化 Intersection Observer 用于图片懒加载
function initializeImageObserver() {
    const options = {
        root: null,
        rootMargin: '100px', // 提前100px加载图片，改善用户体验
        threshold: 0.01
    };
    
    imageObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const img = entry.target;
                if (img.dataset.src) {
                    img.src = img.dataset.src;
                    img.removeAttribute('data-src');
                    imageObserver.unobserve(img);
                }
            }
        });
    }, options);
}

// 初始化无限滚动
function initializeInfiniteScroll() {
    const scrollThreshold = 500; // 距离底部多少像素时触发加载
    let scrollTimeout;
    
    window.addEventListener('scroll', () => {
        // 防抖处理，避免频繁触发
        clearTimeout(scrollTimeout);
        scrollTimeout = setTimeout(() => {
            const scrollPosition = window.innerHeight + window.scrollY;
            const documentHeight = document.documentElement.scrollHeight;
            
            if (scrollPosition >= documentHeight - scrollThreshold && hasMore && !loading) {
                loadMoreImages();
            }
        }, 150);
    }, { passive: true });
}

async function loadImages() {
    if (loading || !hasMore) return;
    
    loading = true;
    const isFirstPage = currentPage === 1;
    
    // 只在第一页时显示加载提示
    if (isFirstPage) {
        document.getElementById('loading').style.display = 'block';
        document.getElementById('masonry').style.display = 'none';
        document.getElementById('pagination').style.display = 'none';
    }
    
    try {
        const startTime = performance.now();
        
        // 创建超时控制
        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), 10000); // 10秒超时
        
        // 计算 offset 基于当前页码
        const offset = (currentPage - 1) * pageSize;
        const response = await fetch(`/api/images/query?offset=${offset}&limit=${pageSize}`, {
            headers: getAuthHeaders(),
            signal: controller.signal
        });
        clearTimeout(timeoutId);
        
        const result = await response.json();
        const endTime = performance.now();
        
        console.log(`加载第${currentPage}页耗时: ${(endTime - startTime).toFixed(2)}ms`);
        
        // API 返回的是 items 字段，需要转换为 images 格式
        const images = result.data.items || result.data.images || [];
        const imagesWithId = images.map(item => ({
            id: item.hash,  // 使用 hash 作为 id
            original_name: `${item.hash.substring(0, 8)}.${item.extension}`,
            mime_type: item.mime_type,
            size: item.size,
            created_at: item.created_at
        }));
        
        if (result.success && imagesWithId.length > 0) {
            displayImages(imagesWithId, isFirstPage);
            totalImages = result.data.total;
            displayedCount += imagesWithId.length;
            
            updateStats(imagesWithId, result.data.total);
            hasMore = imagesWithId.length === pageSize;
            
            // 更新分页信息
            updatePaginationInfo();
            
            // 显示/隐藏按钮
            const loadMoreBtn = document.getElementById('load-more-btn');
            if (loadMoreBtn) {
                loadMoreBtn.disabled = !hasMore;
                loadMoreBtn.textContent = hasMore ? '加载更多' : '已加载全部';
            }
            
            if (isFirstPage) {
                document.getElementById('loading').style.display = 'none';
                document.getElementById('masonry').style.display = 'grid';
                document.getElementById('pagination').style.display = 'flex';
            }
        } else {
            if (isFirstPage) {
                showNoImages();
            }
            hasMore = false;
        }
    } catch (error) {
        console.error('加载图片失败:', error);
        let errorMsg = '加载图片失败';
        if (error.name === 'AbortError') {
            errorMsg = '加载超时，请检查网络连接';
        } else if (error.message) {
            errorMsg = '加载图片失败: ' + error.message;
        }
        showError(errorMsg);
        if (isFirstPage) {
            document.getElementById('loading').style.display = 'none';
        }
    } finally {
        loading = false;
    }
}

function displayImages(images, replace = false) {
    const masonry = document.getElementById('masonry');
    const noImages = document.getElementById('no-images');
    
    if (replace) {
        masonry.innerHTML = '';
        displayedCount = 0;
    }
    
    masonry.style.display = 'grid';
    noImages.style.display = 'none';
    
    images.forEach(image => {
        const item = createImageItem(image);
        masonry.appendChild(item);
        
        // 观察新添加的图片以进行懒加载
        const img = item.querySelector('img');
        if (img && imageObserver) {
            imageObserver.observe(img);
        }
    });
}

function createImageItem(image) {
    const div = document.createElement('div');
    div.className = 'image-item';
    div.onclick = () => openModal(image.id);
    
    const thumbnailUrl = `/images/${image.id}@w400_h200_jpeg_q80`;
    
    div.innerHTML = `
        <img data-src="${thumbnailUrl}" alt="${image.original_name}" src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='400' height='200'%3E%3Crect fill='%23064e5f' width='400' height='200'/%3E%3C/svg%3E" loading="lazy">
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
    document.getElementById('pagination').style.display = 'none';
    document.getElementById('no-images').style.display = 'block';
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

function updatePaginationInfo() {
    const displayStart = Math.max(1, (currentPage - 1) * pageSize + 1);
    const displayEnd = Math.min(totalImages, currentPage * pageSize);
    
    document.getElementById('display-start').textContent = displayStart;
    document.getElementById('display-end').textContent = displayEnd;
    document.getElementById('total-display').textContent = totalImages;
}

async function loadStats() {
    try {
        const response = await fetch('/api/stats', {
            headers: getAuthHeaders()
        });
        const result = await response.json();
        
        if (result.success) {
            document.getElementById('total-count').textContent = result.data.total_images;
            totalImages = result.data.total_images;
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
document.addEventListener('DOMContentLoaded', function() {
    const modal = document.getElementById('image-modal');
    if (modal) {
        modal.addEventListener('click', function(e) {
            if (e.target === this) {
                closeModal();
            }
        });
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
