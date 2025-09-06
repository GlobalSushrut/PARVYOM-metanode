// PARVYOM Metanode Documentation Website - Main JavaScript

document.addEventListener('DOMContentLoaded', function() {
    // Initialize all components
    initNavigation();
    initModuleFilters();
    initCopyButtons();
    initScrollEffects();
    initTooltips();
    initTabs();
    initSearchFunctionality();
    
    console.log('PARVYOM Metanode Documentation Website Loaded');
});

// Navigation functionality
function initNavigation() {
    const navToggle = document.getElementById('nav-toggle');
    const navMenu = document.getElementById('nav-menu');
    
    if (navToggle && navMenu) {
        navToggle.addEventListener('click', function() {
            navToggle.classList.toggle('active');
            navMenu.classList.toggle('active');
        });
        
        // Close menu when clicking on links
        const navLinks = navMenu.querySelectorAll('.nav-link');
        navLinks.forEach(link => {
            link.addEventListener('click', function() {
                navToggle.classList.remove('active');
                navMenu.classList.remove('active');
            });
        });
        
        // Close menu when clicking outside
        document.addEventListener('click', function(e) {
            if (!navToggle.contains(e.target) && !navMenu.contains(e.target)) {
                navToggle.classList.remove('active');
                navMenu.classList.remove('active');
            }
        });
    }
    
    // Navbar scroll effect
    window.addEventListener('scroll', function() {
        const navbar = document.querySelector('.navbar');
        if (window.scrollY > 50) {
            navbar.style.background = 'rgba(255, 255, 255, 0.98)';
            navbar.style.boxShadow = '0 2px 20px rgba(0, 0, 0, 0.1)';
        } else {
            navbar.style.background = 'rgba(255, 255, 255, 0.95)';
            navbar.style.boxShadow = 'none';
        }
    });
}

// Module filtering functionality
function initModuleFilters() {
    const filterButtons = document.querySelectorAll('.filter-btn');
    const moduleCards = document.querySelectorAll('.module-card');
    
    filterButtons.forEach(button => {
        button.addEventListener('click', function() {
            const category = this.dataset.category;
            
            // Update active button
            filterButtons.forEach(btn => btn.classList.remove('active'));
            this.classList.add('active');
            
            // Filter modules
            moduleCards.forEach(card => {
                if (category === 'all' || card.dataset.category === category) {
                    card.style.display = 'block';
                    card.style.animation = 'fadeInUp 0.5s ease-out';
                } else {
                    card.style.display = 'none';
                }
            });
        });
    });
}

// Copy to clipboard functionality
function initCopyButtons() {
    const copyButtons = document.querySelectorAll('.copy-btn');
    
    copyButtons.forEach(button => {
        button.addEventListener('click', function() {
            const codeBlock = this.parentElement;
            const code = codeBlock.querySelector('code');
            const text = code.textContent;
            
            navigator.clipboard.writeText(text).then(() => {
                // Show success feedback
                const originalIcon = this.innerHTML;
                this.innerHTML = '<i class="fas fa-check"></i>';
                this.style.background = 'rgba(16, 185, 129, 0.2)';
                
                setTimeout(() => {
                    this.innerHTML = originalIcon;
                    this.style.background = 'rgba(255, 255, 255, 0.1)';
                }, 2000);
            }).catch(err => {
                console.error('Failed to copy text: ', err);
                // Fallback for older browsers
                const textArea = document.createElement('textarea');
                textArea.value = text;
                document.body.appendChild(textArea);
                textArea.select();
                document.execCommand('copy');
                document.body.removeChild(textArea);
                
                // Show success feedback
                const originalIcon = this.innerHTML;
                this.innerHTML = '<i class="fas fa-check"></i>';
                this.style.background = 'rgba(16, 185, 129, 0.2)';
                
                setTimeout(() => {
                    this.innerHTML = originalIcon;
                    this.style.background = 'rgba(255, 255, 255, 0.1)';
                }, 2000);
            });
        });
    });
}

// Global copy function for inline use
function copyToClipboard(button) {
    const codeBlock = button.parentElement;
    const code = codeBlock.querySelector('code');
    const text = code.textContent;
    
    navigator.clipboard.writeText(text).then(() => {
        // Show success feedback
        const originalIcon = button.innerHTML;
        button.innerHTML = '<i class="fas fa-check"></i>';
        button.style.background = 'rgba(16, 185, 129, 0.2)';
        
        setTimeout(() => {
            button.innerHTML = originalIcon;
            button.style.background = 'rgba(255, 255, 255, 0.1)';
        }, 2000);
    }).catch(err => {
        console.error('Failed to copy text: ', err);
    });
}

// Scroll effects and animations
function initScrollEffects() {
    // Smooth scrolling for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
    
    // Intersection Observer for animations
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };
    
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.style.opacity = '1';
                entry.target.style.transform = 'translateY(0)';
            }
        });
    }, observerOptions);
    
    // Observe elements for animation
    document.querySelectorAll('.module-card, .performance-card, .architecture-layer').forEach(el => {
        el.style.opacity = '0';
        el.style.transform = 'translateY(30px)';
        el.style.transition = 'opacity 0.6s ease-out, transform 0.6s ease-out';
        observer.observe(el);
    });
}

// Tooltip functionality
function initTooltips() {
    const tooltipElements = document.querySelectorAll('[data-tooltip]');
    
    tooltipElements.forEach(element => {
        element.addEventListener('mouseenter', function() {
            const tooltip = document.createElement('div');
            tooltip.className = 'tooltip-popup';
            tooltip.textContent = this.dataset.tooltip;
            tooltip.style.cssText = `
                position: absolute;
                background: #0f172a;
                color: white;
                padding: 8px 12px;
                border-radius: 6px;
                font-size: 14px;
                white-space: nowrap;
                z-index: 1000;
                pointer-events: none;
                opacity: 0;
                transition: opacity 0.2s ease;
            `;
            
            document.body.appendChild(tooltip);
            
            const rect = this.getBoundingClientRect();
            tooltip.style.left = rect.left + (rect.width / 2) - (tooltip.offsetWidth / 2) + 'px';
            tooltip.style.top = rect.top - tooltip.offsetHeight - 8 + 'px';
            
            setTimeout(() => tooltip.style.opacity = '1', 10);
            
            this.tooltipElement = tooltip;
        });
        
        element.addEventListener('mouseleave', function() {
            if (this.tooltipElement) {
                this.tooltipElement.remove();
                this.tooltipElement = null;
            }
        });
    });
}

// Tab functionality
function initTabs() {
    const tabLists = document.querySelectorAll('.tab-list');
    
    tabLists.forEach(tabList => {
        const tabItems = tabList.querySelectorAll('.tab-item');
        const tabContainer = tabList.closest('.tabs');
        const tabContents = tabContainer.querySelectorAll('.tab-content');
        
        tabItems.forEach((tab, index) => {
            tab.addEventListener('click', function() {
                // Remove active class from all tabs and contents
                tabItems.forEach(t => t.classList.remove('active'));
                tabContents.forEach(c => c.classList.remove('active'));
                
                // Add active class to clicked tab and corresponding content
                this.classList.add('active');
                if (tabContents[index]) {
                    tabContents[index].classList.add('active');
                }
            });
        });
    });
}

// Search functionality
function initSearchFunctionality() {
    // Create search overlay
    const searchOverlay = document.createElement('div');
    searchOverlay.className = 'search-overlay';
    searchOverlay.innerHTML = `
        <div class="search-container">
            <div class="search-header">
                <input type="text" class="search-input" placeholder="Search documentation...">
                <button class="search-close">&times;</button>
            </div>
            <div class="search-results">
                <div class="search-suggestions">
                    <h4>Popular Searches</h4>
                    <div class="suggestion-item" data-search="blockchain infrastructure">Blockchain Infrastructure</div>
                    <div class="suggestion-item" data-search="ai security">AI Security Systems</div>
                    <div class="suggestion-item" data-search="iot gateway">IoT Gateway</div>
                    <div class="suggestion-item" data-search="performance optimization">Performance Optimization</div>
                    <div class="suggestion-item" data-search="government api">Government APIs</div>
                </div>
            </div>
        </div>
    `;
    
    searchOverlay.style.cssText = `
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.8);
        z-index: 2000;
        display: none;
        align-items: flex-start;
        justify-content: center;
        padding-top: 100px;
    `;
    
    document.body.appendChild(searchOverlay);
    
    // Add keyboard shortcut for search (Ctrl+K or Cmd+K)
    document.addEventListener('keydown', function(e) {
        if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
            e.preventDefault();
            openSearch();
        }
        
        if (e.key === 'Escape') {
            closeSearch();
        }
    });
    
    // Search functionality
    function openSearch() {
        searchOverlay.style.display = 'flex';
        searchOverlay.querySelector('.search-input').focus();
    }
    
    function closeSearch() {
        searchOverlay.style.display = 'none';
    }
    
    // Close search when clicking overlay
    searchOverlay.addEventListener('click', function(e) {
        if (e.target === searchOverlay) {
            closeSearch();
        }
    });
    
    // Close button
    searchOverlay.querySelector('.search-close').addEventListener('click', closeSearch);
    
    // Search suggestions
    searchOverlay.querySelectorAll('.suggestion-item').forEach(item => {
        item.addEventListener('click', function() {
            const searchTerm = this.dataset.search;
            performSearch(searchTerm);
        });
    });
    
    // Search input
    const searchInput = searchOverlay.querySelector('.search-input');
    searchInput.addEventListener('input', function() {
        const query = this.value.toLowerCase();
        if (query.length > 2) {
            performSearch(query);
        }
    });
}

// Perform search function
function performSearch(query) {
    // This would typically connect to a search API or search index
    // For now, we'll simulate search results
    const searchResults = [
        {
            title: 'Blockchain Infrastructure & Core Systems',
            description: 'IBFT consensus, validator infrastructure, and cross-chain protocols',
            url: './modules/40-blockchain-infrastructure.html'
        },
        {
            title: 'AI & Machine Learning Systems',
            description: 'AI-powered security, predictive analytics, and intelligent automation',
            url: './modules/38-ai-machine-learning.html'
        },
        {
            title: 'IoT & Edge Computing Systems',
            description: 'Ultra-lightweight protocols and distributed edge processing',
            url: './modules/39-iot-edge-computing.html'
        }
    ];
    
    const resultsContainer = document.querySelector('.search-results');
    resultsContainer.innerHTML = `
        <div class="search-results-list">
            <h4>Search Results for "${query}"</h4>
            ${searchResults.map(result => `
                <div class="search-result-item">
                    <h5><a href="${result.url}">${result.title}</a></h5>
                    <p>${result.description}</p>
                </div>
            `).join('')}
        </div>
    `;
}

// Performance monitoring
function initPerformanceMonitoring() {
    // Monitor page load performance
    window.addEventListener('load', function() {
        const loadTime = performance.now();
        console.log(`Page loaded in ${loadTime.toFixed(2)}ms`);
        
        // Report to analytics (if implemented)
        if (typeof gtag !== 'undefined') {
            gtag('event', 'page_load_time', {
                value: Math.round(loadTime),
                custom_parameter: 'documentation_site'
            });
        }
    });
}

// Error handling
window.addEventListener('error', function(e) {
    console.error('JavaScript Error:', e.error);
    
    // Report to error tracking service (if implemented)
    if (typeof Sentry !== 'undefined') {
        Sentry.captureException(e.error);
    }
});

// Progressive Web App features
if ('serviceWorker' in navigator) {
    window.addEventListener('load', function() {
        navigator.serviceWorker.register('./sw.js')
            .then(function(registration) {
                console.log('ServiceWorker registration successful');
            })
            .catch(function(err) {
                console.log('ServiceWorker registration failed');
            });
    });
}

// Theme switching (if needed)
function initThemeSwitch() {
    const themeSwitch = document.querySelector('.theme-switch');
    if (themeSwitch) {
        themeSwitch.addEventListener('click', function() {
            document.body.classList.toggle('dark-theme');
            localStorage.setItem('theme', document.body.classList.contains('dark-theme') ? 'dark' : 'light');
        });
        
        // Load saved theme
        const savedTheme = localStorage.getItem('theme');
        if (savedTheme === 'dark') {
            document.body.classList.add('dark-theme');
        }
    }
}

// Initialize performance monitoring
initPerformanceMonitoring();
