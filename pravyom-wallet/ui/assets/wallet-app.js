// Pravyom Wallet - Modern BPI Core Management Interface
// TypeScript-style JavaScript for real-time wallet and component management

class PravyomWallet {
    constructor() {
        this.websocket = null;
        this.isConnected = false;
        this.walletLocked = true;
        this.components = new Map();
        this.currentTab = 'dashboard';
        this.retryCount = 0;
        this.maxRetries = 5;
        
        this.init();
    }
    
    init() {
        this.setupEventListeners();
        this.setupTabs();
        this.connectWebSocket();
        this.loadInitialData();
        
        // Start periodic updates
        setInterval(() => this.updateMetrics(), 5000);
    }
    
    setupEventListeners() {
        // Wallet controls
        document.getElementById('wallet-lock-btn').addEventListener('click', () => {
            if (this.walletLocked) {
                this.showUnlockModal();
            } else {
                this.lockWallet();
            }
        });
        
        // Unlock modal
        document.getElementById('unlock-form').addEventListener('submit', (e) => {
            e.preventDefault();
            this.unlockWallet();
        });
        
        document.getElementById('cancel-unlock-btn').addEventListener('click', () => {
            this.hideModal('unlock-modal');
        });
        
        // Send transaction
        document.getElementById('send-btn').addEventListener('click', () => {
            if (!this.walletLocked) {
                this.showSendModal();
            } else {
                this.showNotification('Please unlock wallet first', 'warning');
            }
        });
        
        document.getElementById('send-form').addEventListener('submit', (e) => {
            e.preventDefault();
            this.sendTransaction();
        });
        
        document.getElementById('cancel-send-btn').addEventListener('click', () => {
            this.hideModal('send-modal');
        });
        
        // Quick actions
        document.getElementById('start-core-btn').addEventListener('click', () => {
            this.startCoreComponents();
        });
        
        document.getElementById('stop-all-btn').addEventListener('click', () => {
            this.stopAllComponents();
        });
        
        document.getElementById('refresh-btn').addEventListener('click', () => {
            this.refreshComponents();
        });
        
        document.getElementById('refresh-components-btn').addEventListener('click', () => {
            this.refreshComponents();
        });
        
        // Component modal
        document.getElementById('close-component-modal').addEventListener('click', () => {
            this.hideModal('component-modal');
        });
        
        // Log component selection
        document.getElementById('log-component-select').addEventListener('change', (e) => {
            if (e.target.value) {
                this.loadComponentLogs(e.target.value);
            }
        });
        
        // Category filter
        document.getElementById('category-filter').addEventListener('change', () => {
            this.filterComponents();
        });
    }
    
    setupTabs() {
        const tabs = document.querySelectorAll('.tab');
        tabs.forEach(tab => {
            tab.addEventListener('click', () => {
                const tabName = tab.dataset.tab;
                this.switchTab(tabName);
            });
        });
    }
    
    switchTab(tabName) {
        // Update tab buttons
        document.querySelectorAll('.tab').forEach(tab => {
            tab.classList.remove('active');
        });
        document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');
        
        // Update tab content
        document.querySelectorAll('.tab-content').forEach(content => {
            content.classList.add('hidden');
        });
        document.getElementById(`${tabName}-tab`).classList.remove('hidden');
        
        this.currentTab = tabName;
        
        // Load tab-specific data
        if (tabName === 'components') {
            this.loadComponents();
        } else if (tabName === 'transactions') {
            this.loadTransactions();
        }
    }
    
    connectWebSocket() {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws`;
        
        try {
            this.websocket = new WebSocket(wsUrl);
            
            this.websocket.onopen = () => {
                console.log('🔌 WebSocket connected');
                this.isConnected = true;
                this.retryCount = 0;
                this.updateConnectionStatus(true);
            };
            
            this.websocket.onmessage = (event) => {
                try {
                    const data = JSON.parse(event.data);
                    this.handleWebSocketMessage(data);
                } catch (e) {
                    console.error('Failed to parse WebSocket message:', e);
                }
            };
            
            this.websocket.onclose = () => {
                console.log('🔌 WebSocket disconnected');
                this.isConnected = false;
                this.updateConnectionStatus(false);
                this.scheduleReconnect();
            };
            
            this.websocket.onerror = (error) => {
                console.error('WebSocket error:', error);
                this.updateConnectionStatus(false);
            };
        } catch (e) {
            console.error('Failed to create WebSocket connection:', e);
            this.scheduleReconnect();
        }
    }
    
    scheduleReconnect() {
        if (this.retryCount < this.maxRetries) {
            const delay = Math.pow(2, this.retryCount) * 1000; // Exponential backoff
            setTimeout(() => {
                this.retryCount++;
                console.log(`Reconnecting... (attempt ${this.retryCount})`);
                this.connectWebSocket();
            }, delay);
        } else {
            this.showNotification('Connection lost. Please refresh the page.', 'error');
        }
    }
    
    handleWebSocketMessage(data) {
        switch (data.type) {
            case 'status_update':
                this.updateWalletStatus(data.data);
                break;
            case 'unlock_response':
                this.handleUnlockResponse(data);
                break;
            case 'lock_response':
                this.handleLockResponse(data);
                break;
            case 'component_response':
                this.handleComponentResponse(data);
                break;
            case 'logs_response':
                this.handleLogsResponse(data);
                break;
            default:
                console.log('Unknown message type:', data.type);
        }
    }
    
    sendWebSocketMessage(message) {
        if (this.websocket && this.websocket.readyState === WebSocket.OPEN) {
            this.websocket.send(JSON.stringify(message));
        } else {
            console.error('WebSocket not connected');
            this.showNotification('Connection lost. Please refresh the page.', 'error');
        }
    }
    
    updateConnectionStatus(connected) {
        const statusIndicator = document.getElementById('connection-status');
        const statusText = document.getElementById('connection-text');
        
        if (connected) {
            statusIndicator.className = 'status-indicator status-running';
            statusText.textContent = 'Connected';
        } else {
            statusIndicator.className = 'status-indicator status-stopped';
            statusText.textContent = 'Disconnected';
        }
    }
    
    updateWalletStatus(status) {
        // Update wallet info
        document.getElementById('wallet-address').textContent = status.wallet.address || 'Not available';
        document.getElementById('wallet-balance').textContent = `${status.wallet.balance || '0.00'} BPI`;
        document.getElementById('wallet-network').textContent = status.wallet.network || 'unknown';
        
        // Update lock status
        this.walletLocked = status.wallet.status === 'Locked';
        this.updateLockButton();
        
        // Update system metrics
        const healthScore = Math.round(status.system.health_score || 0);
        document.getElementById('health-score').textContent = `${healthScore}%`;
        document.getElementById('health-progress').style.width = `${healthScore}%`;
        
        const cpuUsage = Math.round(status.system.cpu_usage || 0);
        document.getElementById('cpu-usage').textContent = `${cpuUsage}%`;
        document.getElementById('cpu-progress').style.width = `${cpuUsage}%`;
        
        const memoryUsage = Math.round(status.system.memory_usage || 0);
        document.getElementById('memory-usage').textContent = `${memoryUsage}%`;
        document.getElementById('memory-progress').style.width = `${memoryUsage}%`;
        
        // Update component counts
        document.getElementById('total-components').textContent = status.components.total || '28';
        document.getElementById('running-components').textContent = status.components.running || '0';
        document.getElementById('stopped-components').textContent = (status.components.total - status.components.running) || '28';
    }
    
    updateLockButton() {
        const lockBtn = document.getElementById('wallet-lock-btn');
        const lockIcon = document.getElementById('lock-icon');
        const lockText = document.getElementById('lock-text');
        
        if (this.walletLocked) {
            lockIcon.textContent = '🔒';
            lockText.textContent = 'Locked';
            lockBtn.className = 'btn-secondary';
        } else {
            lockIcon.textContent = '🔓';
            lockText.textContent = 'Unlocked';
            lockBtn.className = 'btn-primary';
        }
    }
    
    showUnlockModal() {
        document.getElementById('unlock-modal').classList.add('active');
        document.getElementById('unlock-password').focus();
    }
    
    showSendModal() {
        document.getElementById('send-modal').classList.add('active');
        document.getElementById('send-to').focus();
    }
    
    hideModal(modalId) {
        document.getElementById(modalId).classList.remove('active');
    }
    
    unlockWallet() {
        const password = document.getElementById('unlock-password').value;
        if (!password) {
            this.showNotification('Please enter a password', 'warning');
            return;
        }
        
        this.setButtonLoading('unlock-btn-text', 'unlock-spinner', true);
        
        this.sendWebSocketMessage({
            type: 'unlock_wallet',
            password: password
        });
    }
    
    handleUnlockResponse(data) {
        this.setButtonLoading('unlock-btn-text', 'unlock-spinner', false);
        
        if (data.success) {
            this.walletLocked = false;
            this.updateLockButton();
            this.hideModal('unlock-modal');
            document.getElementById('unlock-password').value = '';
            this.showNotification('Wallet unlocked successfully', 'success');
        } else {
            this.showNotification(data.error || 'Failed to unlock wallet', 'error');
        }
    }
    
    lockWallet() {
        this.sendWebSocketMessage({
            type: 'lock_wallet'
        });
    }
    
    handleLockResponse(data) {
        if (data.success) {
            this.walletLocked = true;
            this.updateLockButton();
            this.showNotification('Wallet locked successfully', 'success');
        } else {
            this.showNotification(data.error || 'Failed to lock wallet', 'error');
        }
    }
    
    sendTransaction() {
        const to = document.getElementById('send-to').value;
        const amount = parseFloat(document.getElementById('send-amount').value);
        
        if (!to || !amount || amount <= 0) {
            this.showNotification('Please enter valid recipient and amount', 'warning');
            return;
        }
        
        this.setButtonLoading('send-btn-text', 'send-spinner', true);
        
        fetch('/api/wallet/send', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ to, amount })
        })
        .then(response => response.json())
        .then(data => {
            this.setButtonLoading('send-btn-text', 'send-spinner', false);
            
            if (data.status === 'pending') {
                this.hideModal('send-modal');
                document.getElementById('send-to').value = '';
                document.getElementById('send-amount').value = '';
                this.showNotification(`Transaction sent: ${data.transaction_id}`, 'success');
            } else {
                this.showNotification(data.error || 'Transaction failed', 'error');
            }
        })
        .catch(error => {
            this.setButtonLoading('send-btn-text', 'send-spinner', false);
            this.showNotification('Network error', 'error');
        });
    }
    
    loadInitialData() {
        // Load wallet info
        fetch('/api/wallet/info')
            .then(response => response.json())
            .then(data => {
                document.getElementById('wallet-address').textContent = data.address || 'Not available';
                document.getElementById('wallet-balance').textContent = `${data.balance || '0.00'} BPI`;
                document.getElementById('wallet-network').textContent = data.network || 'unknown';
            })
            .catch(error => console.error('Failed to load wallet info:', error));
    }
    
    loadComponents() {
        fetch('/api/bpi/components')
            .then(response => response.json())
            .then(data => {
                this.components.clear();
                
                // Populate log component select
                const logSelect = document.getElementById('log-component-select');
                logSelect.innerHTML = '<option value="">Select Component</option>';
                
                // Process components by category
                Object.entries(data.components_by_category || {}).forEach(([category, components]) => {
                    components.forEach(component => {
                        this.components.set(component.name, { ...component, category });
                        
                        // Add to log select
                        const option = document.createElement('option');
                        option.value = component.name;
                        option.textContent = component.display_name || component.name;
                        logSelect.appendChild(option);
                    });
                });
                
                this.renderComponents();
                this.renderComponentCategories();
            })
            .catch(error => {
                console.error('Failed to load components:', error);
                this.showNotification('Failed to load components', 'error');
            });
    }
    
    renderComponents() {
        const grid = document.getElementById('components-grid');
        const filter = document.getElementById('category-filter').value;
        
        grid.innerHTML = '';
        
        this.components.forEach((component, name) => {
            if (filter && component.category !== filter) return;
            
            const card = document.createElement('div');
            card.className = 'card p-4 cursor-pointer hover:bg-gray-700 transition-colors';
            card.onclick = () => this.showComponentDetails(name);
            
            const statusClass = this.getStatusClass(component.status);
            const statusText = this.getStatusText(component.status);
            
            card.innerHTML = `
                <div class="flex items-center justify-between mb-2">
                    <h4 class="font-semibold">${component.display_name || name}</h4>
                    <div class="flex items-center">
                        <div class="status-indicator ${statusClass}"></div>
                        <span class="text-sm">${statusText}</span>
                    </div>
                </div>
                
                <p class="text-sm text-gray-400 mb-3">${component.description || 'BPI Core component'}</p>
                
                <div class="flex justify-between items-center">
                    <span class="text-xs px-2 py-1 bg-gray-600 rounded">${component.category}</span>
                    <div class="flex space-x-2">
                        ${component.status === 'Running' ? 
                            `<button class="btn-secondary text-xs px-2 py-1" onclick="event.stopPropagation(); window.wallet.stopComponent('${name}')">Stop</button>` :
                            `<button class="btn-primary text-xs px-2 py-1" onclick="event.stopPropagation(); window.wallet.startComponent('${name}')">Start</button>`
                        }
                    </div>
                </div>
            `;
            
            grid.appendChild(card);
        });
        
        if (grid.children.length === 0) {
            grid.innerHTML = '<div class="text-center text-gray-400 py-8">No components found</div>';
        }
    }
    
    renderComponentCategories() {
        const container = document.getElementById('component-categories');
        const categories = {};
        
        // Group components by category
        this.components.forEach((component) => {
            if (!categories[component.category]) {
                categories[component.category] = { total: 0, running: 0 };
            }
            categories[component.category].total++;
            if (component.status === 'Running') {
                categories[component.category].running++;
            }
        });
        
        container.innerHTML = '';
        
        Object.entries(categories).forEach(([category, stats]) => {
            const percentage = stats.total > 0 ? (stats.running / stats.total) * 100 : 0;
            
            const categoryCard = document.createElement('div');
            categoryCard.className = 'flex items-center justify-between p-4 bg-gray-700 rounded-lg';
            
            categoryCard.innerHTML = `
                <div>
                    <h4 class="font-medium capitalize">${category}</h4>
                    <p class="text-sm text-gray-400">${stats.running}/${stats.total} running</p>
                </div>
                
                <div class="w-24">
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: ${percentage}%"></div>
                    </div>
                </div>
            `;
            
            container.appendChild(categoryCard);
        });
    }
    
    filterComponents() {
        this.renderComponents();
    }
    
    getStatusClass(status) {
        switch (status) {
            case 'Running': return 'status-running';
            case 'Stopped': return 'status-stopped';
            case 'Starting': return 'status-starting';
            default: return 'status-error';
        }
    }
    
    getStatusText(status) {
        if (typeof status === 'object' && status.Error) {
            return 'Error';
        }
        return status || 'Unknown';
    }
    
    startComponent(name) {
        this.sendWebSocketMessage({
            type: 'start_component',
            component: name
        });
    }
    
    stopComponent(name) {
        this.sendWebSocketMessage({
            type: 'stop_component',
            component: name
        });
    }
    
    handleComponentResponse(data) {
        if (data.success) {
            this.showNotification(data.message, 'success');
            // Refresh components after a short delay
            setTimeout(() => this.loadComponents(), 1000);
        } else {
            this.showNotification(data.error || 'Component operation failed', 'error');
        }
    }
    
    startCoreComponents() {
        const coreComponents = ['bpi-core', 'consensus-engine', 'network-manager'];
        coreComponents.forEach(component => {
            if (this.components.has(component)) {
                this.startComponent(component);
            }
        });
    }
    
    stopAllComponents() {
        if (confirm('Are you sure you want to stop all components?')) {
            this.components.forEach((component, name) => {
                if (component.status === 'Running') {
                    this.stopComponent(name);
                }
            });
        }
    }
    
    refreshComponents() {
        this.sendWebSocketMessage({
            type: 'refresh_components'
        });
        
        // Also reload the components list
        setTimeout(() => this.loadComponents(), 500);
    }
    
    showComponentDetails(name) {
        const component = this.components.get(name);
        if (!component) return;
        
        const modal = document.getElementById('component-modal');
        const title = document.getElementById('component-modal-title');
        const content = document.getElementById('component-modal-content');
        
        title.textContent = component.display_name || name;
        
        const statusClass = this.getStatusClass(component.status);
        const statusText = this.getStatusText(component.status);
        
        content.innerHTML = `
            <div class="space-y-4">
                <div class="flex items-center justify-between">
                    <div class="flex items-center">
                        <div class="status-indicator ${statusClass}"></div>
                        <span class="font-medium">${statusText}</span>
                    </div>
                    <span class="text-sm px-2 py-1 bg-gray-600 rounded">${component.category}</span>
                </div>
                
                <div>
                    <h4 class="font-medium mb-2">Description</h4>
                    <p class="text-gray-400">${component.description || 'No description available'}</p>
                </div>
                
                ${component.port ? `
                <div>
                    <h4 class="font-medium mb-2">Port</h4>
                    <p class="text-gray-400">${component.port}</p>
                </div>
                ` : ''}
                
                ${component.dependencies && component.dependencies.length > 0 ? `
                <div>
                    <h4 class="font-medium mb-2">Dependencies</h4>
                    <div class="flex flex-wrap gap-2">
                        ${component.dependencies.map(dep => 
                            `<span class="text-xs px-2 py-1 bg-gray-600 rounded">${dep}</span>`
                        ).join('')}
                    </div>
                </div>
                ` : ''}
                
                <div class="flex space-x-2 pt-4">
                    ${component.status === 'Running' ? 
                        `<button class="btn-secondary" onclick="window.wallet.stopComponent('${name}'); window.wallet.hideModal('component-modal')">Stop Component</button>` :
                        `<button class="btn-primary" onclick="window.wallet.startComponent('${name}'); window.wallet.hideModal('component-modal')">Start Component</button>`
                    }
                    <button class="btn-secondary" onclick="window.wallet.loadComponentLogs('${name}'); window.wallet.switchTab('logs'); window.wallet.hideModal('component-modal')">View Logs</button>
                </div>
            </div>
        `;
        
        modal.classList.add('active');
    }
    
    loadComponentLogs(componentName) {
        document.getElementById('log-component-select').value = componentName;
        
        this.sendWebSocketMessage({
            type: 'get_component_logs',
            component: componentName
        });
    }
    
    handleLogsResponse(data) {
        const container = document.getElementById('logs-container');
        
        if (data.success && data.logs) {
            container.innerHTML = data.logs.map(log => 
                `<div class="mb-1">
                    <span class="text-gray-500">[${log.timestamp || new Date().toISOString()}]</span>
                    <span class="text-white">${log.message || log}</span>
                </div>`
            ).join('');
            
            // Scroll to bottom
            container.scrollTop = container.scrollHeight;
        } else {
            container.innerHTML = `<div class="text-red-400">Failed to load logs: ${data.error || 'Unknown error'}</div>`;
        }
    }
    
    loadTransactions() {
        fetch('/api/wallet/transactions')
            .then(response => response.json())
            .then(data => {
                const container = document.getElementById('transactions-list');
                
                if (data.transactions && data.transactions.length > 0) {
                    container.innerHTML = data.transactions.map(tx => `
                        <div class="card p-4">
                            <div class="flex justify-between items-center">
                                <div>
                                    <p class="font-medium">${tx.type || 'Transfer'}</p>
                                    <p class="text-sm text-gray-400">${tx.hash || 'N/A'}</p>
                                </div>
                                <div class="text-right">
                                    <p class="font-medium ${tx.amount > 0 ? 'text-green-400' : 'text-red-400'}">
                                        ${tx.amount > 0 ? '+' : ''}${tx.amount} BPI
                                    </p>
                                    <p class="text-sm text-gray-400">${new Date(tx.timestamp).toLocaleString()}</p>
                                </div>
                            </div>
                        </div>
                    `).join('');
                } else {
                    container.innerHTML = '<div class="text-center text-gray-400 py-8">No transactions found</div>';
                }
            })
            .catch(error => {
                console.error('Failed to load transactions:', error);
                document.getElementById('transactions-list').innerHTML = 
                    '<div class="text-center text-red-400 py-8">Failed to load transactions</div>';
            });
    }
    
    updateMetrics() {
        if (!this.isConnected) return;
        
        fetch('/api/bpi/metrics')
            .then(response => response.json())
            .then(data => {
                // Update health score
                const healthScore = Math.round(data.health_score || 0);
                document.getElementById('health-score').textContent = `${healthScore}%`;
                document.getElementById('health-progress').style.width = `${healthScore}%`;
                
                // Update system metrics
                if (data.system) {
                    const cpuUsage = Math.round(data.system.cpu_usage || 0);
                    document.getElementById('cpu-usage').textContent = `${cpuUsage}%`;
                    document.getElementById('cpu-progress').style.width = `${cpuUsage}%`;
                    
                    const memoryUsage = Math.round(data.system.memory_usage || 0);
                    document.getElementById('memory-usage').textContent = `${memoryUsage}%`;
                    document.getElementById('memory-progress').style.width = `${memoryUsage}%`;
                }
            })
            .catch(error => console.error('Failed to update metrics:', error));
    }
    
    setButtonLoading(textId, spinnerId, loading) {
        const textEl = document.getElementById(textId);
        const spinnerEl = document.getElementById(spinnerId);
        
        if (loading) {
            textEl.style.display = 'none';
            spinnerEl.classList.remove('hidden');
        } else {
            textEl.style.display = 'inline';
            spinnerEl.classList.add('hidden');
        }
    }
    
    showNotification(message, type = 'info') {
        const container = document.getElementById('notification-container');
        
        const notification = document.createElement('div');
        notification.className = `notification ${type}`;
        notification.textContent = message;
        
        container.appendChild(notification);
        
        // Show notification
        setTimeout(() => notification.classList.add('show'), 100);
        
        // Hide and remove notification
        setTimeout(() => {
            notification.classList.remove('show');
            setTimeout(() => container.removeChild(notification), 300);
        }, 4000);
    }
}

// Initialize wallet when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.wallet = new PravyomWallet();
    console.log('🚀 Pravyom Wallet initialized');
});
