/**
 * JWT Token Management System for BPCI Enterprise Dashboard
 * Handles secure token storage, refresh, validation, and auto-logout
 */

export interface AuthToken {
  accessToken: string;
  refreshToken: string;
  expiresAt: number;
  tokenType: string;
  scope: string[];
}

export interface UserRole {
  role: 'admin' | 'validator' | 'user' | 'guest';
  permissions: string[];
  bpiAccess: boolean;
  dashboardSections: string[];
}

export interface AuthUser {
  id: string;
  email: string;
  role: UserRole;
  bpiAddress?: string;
  lastActivity: number;
  sessionId: string;
}

class AuthTokenManager {
  private static instance: AuthTokenManager;
  private readonly TOKEN_KEY = 'bpci_auth_token';
  private readonly USER_KEY = 'bpci_user_data';
  private readonly SESSION_TIMEOUT = 30 * 60 * 1000; // 30 minutes
  private refreshTimer: NodeJS.Timeout | null = null;
  private activityTimer: NodeJS.Timeout | null = null;

  private constructor() {
    this.setupActivityTracking();
    this.setupAutoRefresh();
  }

  public static getInstance(): AuthTokenManager {
    if (!AuthTokenManager.instance) {
      AuthTokenManager.instance = new AuthTokenManager();
    }
    return AuthTokenManager.instance;
  }

  /**
   * Store authentication token securely
   */
  public storeToken(token: AuthToken, user: AuthUser): void {
    try {
      // Encrypt sensitive data before storage
      const encryptedToken = this.encryptData(JSON.stringify(token));
      const encryptedUser = this.encryptData(JSON.stringify(user));
      
      localStorage.setItem(this.TOKEN_KEY, encryptedToken);
      localStorage.setItem(this.USER_KEY, encryptedUser);
      
      // Set up auto-refresh
      this.scheduleTokenRefresh(token.expiresAt);
      
      // Track user activity
      this.updateLastActivity();
      
      console.log('🔐 Token stored securely');
    } catch (error) {
      console.error('Failed to store token:', error);
      throw new Error('Token storage failed');
    }
  }

  /**
   * Get stored authentication token
   */
  public getToken(): AuthToken | null {
    try {
      const encryptedToken = localStorage.getItem(this.TOKEN_KEY);
      if (!encryptedToken) return null;

      const decryptedToken = this.decryptData(encryptedToken);
      const token: AuthToken = JSON.parse(decryptedToken);

      // Check if token is expired
      if (this.isTokenExpired(token)) {
        this.clearToken();
        return null;
      }

      return token;
    } catch (error) {
      console.error('Failed to retrieve token:', error);
      this.clearToken();
      return null;
    }
  }

  /**
   * Get current authenticated user
   */
  public getCurrentUser(): AuthUser | null {
    try {
      const encryptedUser = localStorage.getItem(this.USER_KEY);
      if (!encryptedUser) return null;

      const decryptedUser = this.decryptData(encryptedUser);
      const user: AuthUser = JSON.parse(decryptedUser);

      // Check session timeout
      if (this.isSessionExpired(user)) {
        this.logout();
        return null;
      }

      return user;
    } catch (error) {
      console.error('Failed to retrieve user:', error);
      this.logout();
      return null;
    }
  }

  /**
   * Refresh authentication token
   */
  public async refreshToken(): Promise<string | null> {
    try {
      const currentToken = this.getToken();
      if (!currentToken?.refreshToken) {
        throw new Error('No refresh token available');
      }

      const response = await fetch('/api/auth/token/refresh', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          refreshToken: currentToken.refreshToken,
        }),
      });

      if (!response.ok) {
        throw new Error('Token refresh failed');
      }

      const data = await response.json();
      const newToken: AuthToken = {
        accessToken: data.accessToken,
        refreshToken: data.refreshToken || currentToken.refreshToken,
        expiresAt: Date.now() + (data.expiresIn * 1000),
        tokenType: data.tokenType || 'Bearer',
        scope: data.scope || currentToken.scope,
      };

      // Update stored token
      const currentUser = this.getCurrentUser();
      if (currentUser) {
        this.storeToken(newToken, currentUser);
      }

      console.log('🔄 Token refreshed successfully');
      return newToken.accessToken;
    } catch (error) {
      console.error('Token refresh failed:', error);
      this.logout();
      return null;
    }
  }

  /**
   * Validate current token
   */
  public async validateToken(): Promise<boolean> {
    try {
      const token = this.getToken();
      if (!token) return false;

      const response = await fetch('/api/auth/session/validate', {
        method: 'GET',
        headers: {
          'Authorization': `${token.tokenType} ${token.accessToken}`,
        },
      });

      if (response.ok) {
        this.updateLastActivity();
        return true;
      } else {
        this.logout();
        return false;
      }
    } catch (error) {
      console.error('Token validation failed:', error);
      return false;
    }
  }

  /**
   * Check if user has specific permission
   */
  public hasPermission(permission: string): boolean {
    const user = this.getCurrentUser();
    return user?.role.permissions.includes(permission) || false;
  }

  /**
   * Check if user has BPI access
   */
  public hasBpiAccess(): boolean {
    const user = this.getCurrentUser();
    return user?.role.bpiAccess || false;
  }

  /**
   * Check if user can access dashboard section
   */
  public canAccessSection(section: string): boolean {
    const user = this.getCurrentUser();
    return user?.role.dashboardSections.includes(section) || false;
  }

  /**
   * Logout user and clear all data
   */
  public async logout(): Promise<void> {
    try {
      const token = this.getToken();
      
      // Call logout endpoint if token exists
      if (token) {
        await fetch('/api/auth/logout', {
          method: 'POST',
          headers: {
            'Authorization': `${token.tokenType} ${token.accessToken}`,
          },
        });
      }
    } catch (error) {
      console.error('Logout API call failed:', error);
    } finally {
      this.clearToken();
      this.clearTimers();
      console.log('🚪 User logged out');
    }
  }

  /**
   * Auto-logout due to inactivity
   */
  private autoLogout(): void {
    console.log('⏰ Auto-logout due to inactivity');
    this.logout();
    
    // Redirect to login page
    window.location.href = '/login';
  }

  /**
   * Clear stored token and user data
   */
  private clearToken(): void {
    localStorage.removeItem(this.TOKEN_KEY);
    localStorage.removeItem(this.USER_KEY);
  }

  /**
   * Check if token is expired
   */
  private isTokenExpired(token: AuthToken): boolean {
    return Date.now() >= token.expiresAt;
  }

  /**
   * Check if session is expired due to inactivity
   */
  private isSessionExpired(user: AuthUser): boolean {
    return Date.now() - user.lastActivity > this.SESSION_TIMEOUT;
  }

  /**
   * Update last activity timestamp
   */
  private updateLastActivity(): void {
    const user = this.getCurrentUser();
    if (user) {
      user.lastActivity = Date.now();
      const encryptedUser = this.encryptData(JSON.stringify(user));
      localStorage.setItem(this.USER_KEY, encryptedUser);
    }
  }

  /**
   * Setup activity tracking for auto-logout
   */
  private setupActivityTracking(): void {
    const events = ['mousedown', 'mousemove', 'keypress', 'scroll', 'touchstart'];
    
    const resetActivityTimer = () => {
      this.updateLastActivity();
      
      if (this.activityTimer) {
        clearTimeout(this.activityTimer);
      }
      
      this.activityTimer = setTimeout(() => {
        this.autoLogout();
      }, this.SESSION_TIMEOUT);
    };

    events.forEach(event => {
      document.addEventListener(event, resetActivityTimer, true);
    });
  }

  /**
   * Setup automatic token refresh
   */
  private setupAutoRefresh(): void {
    const token = this.getToken();
    if (token) {
      this.scheduleTokenRefresh(token.expiresAt);
    }
  }

  /**
   * Schedule token refresh before expiration
   */
  private scheduleTokenRefresh(expiresAt: number): void {
    if (this.refreshTimer) {
      clearTimeout(this.refreshTimer);
    }

    // Refresh 5 minutes before expiration
    const refreshTime = expiresAt - Date.now() - (5 * 60 * 1000);
    
    if (refreshTime > 0) {
      this.refreshTimer = setTimeout(() => {
        this.refreshToken();
      }, refreshTime);
    }
  }

  /**
   * Clear all timers
   */
  private clearTimers(): void {
    if (this.refreshTimer) {
      clearTimeout(this.refreshTimer);
      this.refreshTimer = null;
    }
    if (this.activityTimer) {
      clearTimeout(this.activityTimer);
      this.activityTimer = null;
    }
  }

  /**
   * Simple encryption for local storage (in production, use proper encryption)
   */
  private encryptData(data: string): string {
    // In production, use proper encryption like AES
    // For now, using base64 encoding as placeholder
    return btoa(data);
  }

  /**
   * Simple decryption for local storage
   */
  private decryptData(encryptedData: string): string {
    // In production, use proper decryption
    // For now, using base64 decoding as placeholder
    return atob(encryptedData);
  }
}

export default AuthTokenManager.getInstance();
