let currentPage = 1;
const pageSize = 32; // 每页显示32张图片 (4列 × 8行)
let hasMore = true;
let loading = false;
let totalImages = 0;
let displayedCount = 0;
let imageObserver = null;
let selectMode = false;
let selectedImages = new Set();
let confirmCallback = null;
let imageCache = new Map();
let toastTimer = null;

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
    
    updateSelectModeUI();
    loadStats();
    loadImages(true);
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

async function loadImages(reset = false) {
    if (loading || (!hasMore && !reset)) return;
    
    if (reset) {
        currentPage = 1;
        hasMore = true;
        displayedCount = 0;
        const masonry = document.getElementById('masonry');
        if (masonry) {
            masonry.innerHTML = '';
        }
    }
    
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
            original_name: item.original_filename,
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
    div.dataset.imageId = image.id;
    div.dataset.displayName = image.original_name;
    
    const checkbox = document.createElement('input');
    checkbox.type = 'checkbox';
    checkbox.className = 'image-checkbox';
    checkbox.dataset.imageId = image.id;
    checkbox.checked = selectedImages.has(image.id);
    checkbox.addEventListener('click', (event) => event.stopPropagation());
    checkbox.addEventListener('change', (event) => {
        handleSelectionChange(image.id, event.target.checked);
    });
    div.appendChild(checkbox);
    
    const actions = document.createElement('div');
    actions.className = 'image-actions';
    
    const previewBtn = document.createElement('button');
    previewBtn.className = 'action-btn';
    previewBtn.textContent = '预览';
    previewBtn.addEventListener('click', (event) => {
        event.stopPropagation();
        openModal(image.id);
    });
    actions.appendChild(previewBtn);
    
    const deleteBtn = document.createElement('button');
    deleteBtn.className = 'action-btn delete';
    deleteBtn.textContent = '删除';
    deleteBtn.addEventListener('click', (event) => {
        event.stopPropagation();
        confirmDeleteSingle(image.id);
    });
    actions.appendChild(deleteBtn);
    
    div.appendChild(actions);
    
    const img = document.createElement('img');
    img.dataset.src = `/images/${image.id}@w400_h200_jpeg_q80`;
    img.alt = image.original_name;
    img.src = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='400' height='200'%3E%3Crect fill='%23064e5f' width='400' height='200'/%3E%3C/svg%3E";
    img.loading = 'lazy';
    div.appendChild(img);
    
    const info = document.createElement('div');
    info.className = 'image-info';
    
    const name = document.createElement('div');
    name.className = 'image-name';
    name.title = image.original_name;
    name.textContent = image.original_name;
    info.appendChild(name);
    
    const meta = document.createElement('div');
    meta.className = 'image-meta';
    meta.textContent = `${formatSize(image.size)} • ${image.mime_type} • ${new Date(image.created_at).toLocaleDateString()}`;
    info.appendChild(meta);
    
    div.appendChild(info);
    
    if (selectedImages.has(image.id)) {
        div.classList.add('selected');
    }
    
    div.addEventListener('click', () => {
        if (selectMode) {
            const isChecked = !checkbox.checked;
            checkbox.checked = isChecked;
            handleSelectionChange(image.id, isChecked);
        } else {
            openModal(image.id);
        }
    });
    
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

// 刷新图片列表
function refreshImages(silent = false) {
    clearSelection();
    updateSelectModeUI();
    loadImages(true);
    loadStats();
    if (!silent) {
        showToast('图片列表已刷新', 'success');
    }
}

// 切换批量选择模式
function toggleSelectMode() {
    selectMode = !selectMode;
    if (!selectMode) {
        selectedImages.clear();
    }
    updateSelectModeUI();
}

// 更新选择模式UI
function updateSelectModeUI() {
    const batchActions = document.getElementById('batch-actions');
    const selectModeBtn = document.getElementById('select-mode-btn');
    
    if (selectMode) {
        batchActions.classList.add('active');
        selectModeBtn.textContent = '取消选择';
        selectModeBtn.style.background = '#94a3b8';
        document.body.classList.add('select-mode');
    } else {
        batchActions.classList.remove('active');
        selectModeBtn.textContent = '批量选择';
        selectModeBtn.style.background = '';
        document.body.classList.remove('select-mode');
    }
    
    updateSelectedCount();
}

// 处理图片选择变化
function handleSelectionChange(imageId, isSelected) {
    if (isSelected) {
        selectedImages.add(imageId);
    } else {
        selectedImages.delete(imageId);
    }
    
    const item = document.querySelector(`.image-item[data-image-id="${imageId}"]`);
    if (item) {
        if (isSelected) {
            item.classList.add('selected');
        } else {
            item.classList.remove('selected');
        }
    }
    
    updateSelectedCount();
}

// 更新已选择计数
function updateSelectedCount() {
    const selectInfo = document.getElementById('select-info');
    if (selectInfo) {
        selectInfo.textContent = `已选择 ${selectedImages.size} 项`;
    }
}

// 全选当前页
function selectAllVisible() {
    const checkboxes = document.querySelectorAll('.image-checkbox');
    checkboxes.forEach(checkbox => {
        if (!checkbox.checked) {
            checkbox.checked = true;
            handleSelectionChange(checkbox.dataset.imageId, true);
        }
    });
}

// 清除选择
function clearSelection() {
    selectedImages.clear();
    const checkboxes = document.querySelectorAll('.image-checkbox');
    checkboxes.forEach(checkbox => {
        checkbox.checked = false;
    });
    document.querySelectorAll('.image-item').forEach(item => {
        item.classList.remove('selected');
    });
    updateSelectedCount();
}

// 确认删除单张图片
function confirmDeleteSingle(imageId) {
    const item = document.querySelector(`.image-item[data-image-id="${imageId}"]`);
    const imageName = item ? item.dataset.displayName : imageId.substring(0, 8);
    
    showConfirmDialog(
        '确认删除',
        `确定要删除图片 "${imageName}" 吗？此操作不可恢复。`,
        () => deleteImages([imageId])
    );
}

// 确认删除选中图片
function confirmDeleteSelected() {
    if (selectedImages.size === 0) {
        showToast('请先选择要删除的图片', 'error');
        return;
    }
    
    showConfirmDialog(
        '确认批量删除',
        `确定要删除选中的 ${selectedImages.size} 张图片吗？此操作不可恢复。`,
        () => deleteImages(Array.from(selectedImages))
    );
}

// 删除图片
async function deleteImages(imageIds) {
    const totalCount = imageIds.length;
    let successCount = 0;
    let failedCount = 0;
    
    showToast(`正在删除 ${totalCount} 张图片...`, 'info');
    
    for (const imageId of imageIds) {
        try {
            const response = await fetch(`/api/images/${imageId}`, {
                method: 'DELETE',
                headers: getAuthHeaders()
            });
            
            if (response.ok) {
                successCount++;
                // 从DOM中移除该图片项
                const item = document.querySelector(`.image-item[data-image-id="${imageId}"]`);
                if (item) {
                    item.remove();
                }
                selectedImages.delete(imageId);
            } else {
                failedCount++;
                console.error(`删除图片 ${imageId} 失败:`, await response.text());
            }
        } catch (error) {
            failedCount++;
            console.error(`删除图片 ${imageId} 失败:`, error);
        }
    }
    
    // 显示删除结果
    if (successCount > 0) {
        showToast(`成功删除 ${successCount} 张图片${failedCount > 0 ? `，${failedCount} 张失败` : ''}`, failedCount > 0 ? 'error' : 'success');
        
        // 更新统计信息
        totalImages -= successCount;
        displayedCount -= successCount;
        updateSelectedCount();
        loadStats();
        
        // 如果当前页面没有图片了，尝试加载更多
        const masonry = document.getElementById('masonry');
        if (masonry.children.length === 0 && hasMore) {
            loadImages();
        } else if (masonry.children.length === 0) {
            showNoImages();
        }
    } else {
        showToast(`删除失败，请重试`, 'error');
    }
    
    // 退出选择模式
    if (selectMode) {
        selectMode = false;
        updateSelectModeUI();
    }
}

// 显示确认对话框
function showConfirmDialog(title, message, onConfirm) {
    const dialog = document.getElementById('confirm-dialog');
    document.getElementById('confirm-title').textContent = title;
    document.getElementById('confirm-message').textContent = message;
    
    confirmCallback = onConfirm;
    dialog.classList.add('show');
}

// 关闭确认对话框
function closeConfirmDialog() {
    const dialog = document.getElementById('confirm-dialog');
    dialog.classList.remove('show');
    confirmCallback = null;
}

// 确认对话框的确认按钮
function confirmDialogAction() {
    if (confirmCallback) {
        confirmCallback();
    }
    closeConfirmDialog();
}

// 显示Toast提示
function showToast(message, type = 'info') {
    const toast = document.getElementById('toast');
    toast.textContent = message;
    toast.className = `toast ${type} show`;
    
    if (toastTimer) {
        clearTimeout(toastTimer);
    }
    
    toastTimer = setTimeout(() => {
        toast.classList.remove('show');
        toastTimer = null;
    }, 3000);
}
