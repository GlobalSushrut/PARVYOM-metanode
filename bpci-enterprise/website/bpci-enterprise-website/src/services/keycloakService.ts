/**
 * Keycloak Integration Service for BPCI Enterprise
 * Provides OAuth2/OIDC authentication, SSO, and enterprise identity management
 */

import Keycloak from 'keycloak-js';

// Keycloak configuration - Integrated with Cloudflare SSO
const keycloakConfig = {
  url: process.env.REACT_APP_KEYCLOAK_URL || 'https://auth.pravyom.com',
  realm: process.env.REACT_APP_KEYCLOAK_REALM || 'pravyom-blockchain',
  clientId: process.env.REACT_APP_KEYCLOAK_CLIENT_ID || 'bpci-web-client',
};

// Initialize Keycloak instance
const keycloak = new Keycloak(keycloakConfig);

// Keycloak initialization options
const initOptions = {
  onLoad: 'check-sso' as const,
  silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso.html',
  checkLoginIframe: false,
  pkceMethod: 'S256' as const,
};

// User profile interface from Keycloak
export interface KeycloakUserProfile {
  id?: string;
  username?: string;
  email?: string;
  firstName?: string;
  lastName?: string;
  emailVerified?: boolean;
  attributes?: {
    [key: string]: string[];
  };
}

// Enhanced user profile with BPCI-specific attributes
export interface BpciUserProfile extends KeycloakUserProfile {
  developerId?: string;
  organizationId?: string;
  roles?: string[];
  permissions?: string[];
  walletAddress?: string;
  bpiBalance?: number;
  subscriptionTier?: 'free' | 'testnet' | 'pilot' | 'enterprise';
  lastLogin?: Date;
}

// Authentication state interface
export interface AuthState {
  isAuthenticated: boolean;
  isLoading: boolean;
  user: BpciUserProfile | null;
  token: string | null;
  refreshToken: string | null;
  roles: string[];
  permissions: string[];
}

class KeycloakService {
  private keycloak: Keycloak;
  private authState: AuthState = {
    isAuthenticated: false,
    isLoading: true,
    user: null,
    token: null,
    refreshToken: null,
    roles: [],
    permissions: [],
  };
  private listeners: ((state: AuthState) => void)[] = [];

  constructor() {
    this.keycloak = keycloak;
  }

  /**
   * Initialize Keycloak authentication
   */
  async init(): Promise<boolean> {
    try {
      const authenticated = await this.keycloak.init(initOptions);
      
      if (authenticated) {
        await this.updateAuthState();
        this.setupTokenRefresh();
      } else {
        this.authState.isLoading = false;
      }

      this.notifyListeners();
      return authenticated;
    } catch (error) {
      console.error('Keycloak initialization failed:', error);
      this.authState.isLoading = false;
      this.notifyListeners();
      return false;
    }
  }

  /**
   * Login user via Keycloak
   */
  async login(options?: { redirectUri?: string }): Promise<void> {
    try {
      await this.keycloak.login({
        redirectUri: options?.redirectUri || window.location.origin,
      });
    } catch (error) {
      console.error('Keycloak login failed:', error);
      throw new Error('Login failed');
    }
  }

  /**
   * Logout user from Keycloak
   */
  async logout(options?: { redirectUri?: string }): Promise<void> {
    try {
      await this.keycloak.logout({
        redirectUri: options?.redirectUri || window.location.origin,
      });
    } catch (error) {
      console.error('Keycloak logout failed:', error);
      throw new Error('Logout failed');
    }
  }

  /**
   * Register new user (redirect to Keycloak registration)
   */
  async register(options?: { redirectUri?: string }): Promise<void> {
    try {
      await this.keycloak.register({
        redirectUri: options?.redirectUri || window.location.origin,
      });
    } catch (error) {
      console.error('Keycloak registration failed:', error);
      throw new Error('Registration failed');
    }
  }

  /**
   * Update user profile
   */
  async updateProfile(): Promise<BpciUserProfile | null> {
    if (!this.keycloak.authenticated) {
      return null;
    }

    try {
      const userProfile = await this.keycloak.loadUserProfile();
      // Convert KeycloakProfile to KeycloakUserProfile format
      const compatibleProfile: KeycloakUserProfile = {
        id: userProfile.id,
        username: userProfile.username,
        email: userProfile.email,
        firstName: userProfile.firstName,
        lastName: userProfile.lastName,
        attributes: userProfile.attributes ? 
          Object.fromEntries(
            Object.entries(userProfile.attributes).map(([key, value]) => [
              key,
              Array.isArray(value) ? value.map(v => String(v)) : [String(value)]
            ])
          ) : undefined
      };
      const bpciProfile = await this.convertProfile(compatibleProfile);
      this.authState.user = bpciProfile;
      this.notifyListeners();
      return bpciProfile;
    } catch (error) {
      console.error('Failed to update user profile:', error);
      return null;
    }
  }

  /**
   * Get current authentication state
   */
  getAuthState(): AuthState {
    return { ...this.authState };
  }

  /**
   * Check if user has specific role
   */
  hasRole(role: string): boolean {
    return this.keycloak.hasRealmRole(role) || this.keycloak.hasResourceRole(role);
  }

  /**
   * Check if user has specific permission
   */
  hasPermission(permission: string): boolean {
    return this.authState.permissions.includes(permission);
  }

  /**
   * Get access token
   */
  getToken(): string | null {
    return this.keycloak.token || null;
  }

  /**
   * Get refresh token
   */
  getRefreshToken(): string | null {
    return this.keycloak.refreshToken || null;
  }

  /**
   * Refresh access token
   */
  async refreshToken(): Promise<boolean> {
    try {
      const refreshed = await this.keycloak.updateToken(30);
      if (refreshed) {
        this.authState.token = this.keycloak.token || null;
        this.authState.refreshToken = this.keycloak.refreshToken || null;
        this.notifyListeners();
      }
      return refreshed;
    } catch (error) {
      console.error('Token refresh failed:', error);
      return false;
    }
  }

  /**
   * Subscribe to authentication state changes
   */
  subscribe(listener: (state: AuthState) => void): () => void {
    this.listeners.push(listener);
    return () => {
      const index = this.listeners.indexOf(listener);
      if (index > -1) {
        this.listeners.splice(index, 1);
      }
    };
  }

  /**
   * Get user account management URL
   */
  getAccountManagementUrl(): string {
    return this.keycloak.createAccountUrl();
  }

  /**
   * Check if token is expired
   */
  isTokenExpired(): boolean {
    return this.keycloak.isTokenExpired();
  }

  /**
   * Private: Update authentication state
   */
  private async updateAuthState(): Promise<void> {
    if (!this.keycloak.authenticated) {
      this.authState = {
        isAuthenticated: false,
        isLoading: false,
        user: null,
        token: null,
        refreshToken: null,
        roles: [],
        permissions: [],
      };
      return;
    }

    try {
      const userProfile = await this.keycloak.loadUserProfile();
      // Convert KeycloakProfile to KeycloakUserProfile format
      const compatibleProfile: KeycloakUserProfile = {
        id: userProfile.id,
        username: userProfile.username,
        email: userProfile.email,
        firstName: userProfile.firstName,
        lastName: userProfile.lastName,
        attributes: userProfile.attributes ? 
          Object.fromEntries(
            Object.entries(userProfile.attributes).map(([key, value]) => [
              key,
              Array.isArray(value) ? value.map(v => String(v)) : [String(value)]
            ])
          ) : undefined
      };
      const bpciProfile = await this.convertProfile(compatibleProfile);
      const roles = this.extractRoles();
      const permissions = await this.extractPermissions();

      this.authState = {
        isAuthenticated: true,
        isLoading: false,
        user: bpciProfile,
        token: this.keycloak.token || null,
        refreshToken: this.keycloak.refreshToken || null,
        roles,
        permissions,
      };
    } catch (error) {
      console.error('Failed to update auth state:', error);
      this.authState.isLoading = false;
    }
  }

  /**
   * Private: Convert user profile to BPCI-specific data
   */
  private async convertProfile(profile: KeycloakUserProfile): Promise<BpciUserProfile> {
    // Safely handle attributes conversion
    const attributes = profile.attributes || {};
    const safeAttributes: { [key: string]: string[] } = {};
    
    Object.keys(attributes).forEach(key => {
      const value = attributes[key];
      if (Array.isArray(value)) {
        safeAttributes[key] = value.map(v => String(v));
      } else if (value !== undefined) {
        safeAttributes[key] = [String(value)];
      }
    });

    return {
      id: profile.id || '',
      username: profile.username || '',
      email: profile.email || '',
      firstName: profile.firstName || '',
      lastName: profile.lastName || '',
      organizationId: safeAttributes.organizationId?.[0] || '',
      roles: this.extractRoles(),
      permissions: await this.extractPermissions(),
    };
  }

  /**
   * Private: Extract user roles
   */
  private extractRoles(): string[] {
    const realmRoles = this.keycloak.realmAccess?.roles || [];
    const resourceRoles = Object.values(this.keycloak.resourceAccess || {})
      .flatMap((resource: any) => resource.roles || []);
    
    return [...new Set([...realmRoles, ...resourceRoles])];
  }

  /**
   * Private: Extract user permissions
   */
  private async extractPermissions(): Promise<string[]> {
    // In a real implementation, you would fetch permissions from Keycloak
    // or derive them from roles. For now, we'll map roles to permissions.
    const rolePermissionMap: Record<string, string[]> = {
      'bpci-admin': ['admin:all', 'wallet:manage', 'user:manage', 'system:configure'],
      'bpci-developer': ['wallet:create', 'wallet:activate', 'bpi:deploy', 'bpi:test'],
      'bpci-user': ['wallet:view', 'bpi:use'],
      'enterprise-user': ['enterprise:access', 'advanced:features'],
    };

    const permissions = this.authState.roles.flatMap(role => 
      rolePermissionMap[role] || []
    );

    return [...new Set(permissions)];
  }

  /**
   * Private: Setup automatic token refresh
   */
  private setupTokenRefresh(): void {
    // Refresh token every 5 minutes if it expires in less than 30 seconds
    setInterval(async () => {
      if (this.keycloak.authenticated && this.keycloak.isTokenExpired(30)) {
        await this.refreshToken();
      }
    }, 5 * 60 * 1000);

    // Setup Keycloak event listeners
    this.keycloak.onTokenExpired = () => {
      console.log('Token expired, refreshing...');
      this.refreshToken();
    };

    this.keycloak.onAuthSuccess = () => {
      console.log('Authentication successful');
      this.updateAuthState();
    };

    this.keycloak.onAuthError = (error: any) => {
      console.error('Authentication error:', error);
    };

    this.keycloak.onAuthLogout = () => {
      console.log('User logged out');
      this.authState = {
        isAuthenticated: false,
        isLoading: false,
        user: null,
        token: null,
        refreshToken: null,
        roles: [],
        permissions: [],
      };
      this.notifyListeners();
    };
  }

  /**
   * Private: Notify all listeners of state changes
   */
  private notifyListeners(): void {
    this.listeners.forEach(listener => listener(this.authState));
  }
}

// Create singleton instance
export const keycloakService = new KeycloakService();

// Export types and service
export default keycloakService;
export { Keycloak };
