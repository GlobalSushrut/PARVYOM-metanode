/**
 * Unified Authentication Service for BPCI Enterprise
 * Integrates existing authService with Keycloak-Cloudflare SSO pipeline
 * Provides seamless email/password signup and login authentication
 */

import { authService } from './authService';
import type { DeveloperLoginRequest, DeveloperSignupRequest, AuthResponse } from './authService';
import { keycloakService } from './keycloakService';
import type { BpciUserProfile, AuthState } from './keycloakService';

// Unified authentication configuration
const AUTH_CONFIG = {
  // Keycloak-Cloudflare SSO endpoints
  keycloakUrl: process.env.REACT_APP_KEYCLOAK_URL || 'https://auth.pravyom.com',
  loginUrl: process.env.REACT_APP_AUTH_LOGIN_URL || 'https://auth.pravyom.com/auth/login',
  signupUrl: process.env.REACT_APP_AUTH_SIGNUP_URL || 'https://auth.pravyom.com/auth/register',
  tokenUrl: process.env.REACT_APP_AUTH_TOKEN_URL || 'https://auth.pravyom.com/auth/token',
  userInfoUrl: process.env.REACT_APP_AUTH_USERINFO_URL || 'https://auth.pravyom.com/auth/userinfo',
  logoutUrl: process.env.REACT_APP_AUTH_LOGOUT_URL || 'https://auth.pravyom.com/auth/logout',
  
  // Feature flags
  enableKeycloakSSO: process.env.REACT_APP_ENABLE_KEYCLOAK_SSO === 'true',
  enableEmailVerification: process.env.REACT_APP_ENABLE_EMAIL_VERIFICATION === 'true',
  enableOTPVerification: process.env.REACT_APP_ENABLE_OTP_VERIFICATION === 'true',
  enableWalletActivation: process.env.REACT_APP_ENABLE_WALLET_ACTIVATION === 'true',
};

// Unified user profile interface
export interface UnifiedUserProfile {
  // Core user data
  id: string;
  email: string;
  username?: string;
  firstName?: string;
  lastName?: string;
  
  // BPCI-specific data
  developerId?: string;
  organizationId?: string;
  roles: string[];
  permissions: string[];
  
  // Wallet data
  walletAddress?: string;
  walletActivated: boolean;
  bpiBalance?: number;
  subscriptionTier: 'free' | 'testnet' | 'pilot' | 'enterprise';
  
  // Authentication state
  isAuthenticated: boolean;
  profileComplete: boolean;
  emailVerified: boolean;
  
  // Session data
  sessionToken?: string;
  accessToken?: string;
  refreshToken?: string;
  lastLogin?: Date;
}

// Unified authentication response
export interface UnifiedAuthResponse {
  success: boolean;
  message: string;
  user?: UnifiedUserProfile;
  requiresVerification?: boolean;
  requiresProfileCompletion?: boolean;
  requiresWalletActivation?: boolean;
}

// Authentication method type
export type AuthMethod = 'email_password' | 'keycloak_sso' | 'oauth';

// Authentication method constants
export const AuthMethods = {
  EMAIL_PASSWORD: 'email_password' as AuthMethod,
  KEYCLOAK_SSO: 'keycloak_sso' as AuthMethod,
  OAUTH: 'oauth' as AuthMethod,
} as const;

class UnifiedAuthService {
  private currentUser: UnifiedUserProfile | null = null;
  private authMethod: AuthMethod | null = null;
  private listeners: ((user: UnifiedUserProfile | null) => void)[] = [];

  constructor() {
    this.initializeAuthState();
  }

  /**
   * Initialize authentication state from stored data
   */
  private async initializeAuthState(): Promise<void> {
    try {
      // Check for existing Keycloak session first
      if (AUTH_CONFIG.enableKeycloakSSO) {
        const keycloakInitialized = await keycloakService.init();
        if (keycloakInitialized) {
          const keycloakState = keycloakService.getAuthState();
          if (keycloakState.isAuthenticated && keycloakState.user) {
            this.currentUser = this.convertKeycloakProfile(keycloakState.user);
            this.authMethod = AuthMethods.KEYCLOAK_SSO;
            this.notifyListeners();
            return;
          }
        }
      }

      // Fallback to traditional auth service
      const developer = authService.getCurrentDeveloper();
      if (developer && authService.isAuthenticated()) {
        this.currentUser = this.convertDeveloperProfile(developer);
        this.authMethod = AuthMethods.EMAIL_PASSWORD;
        this.notifyListeners();
      }
    } catch (error) {
      console.error('Failed to initialize auth state:', error);
    }
  }

  /**
   * Unified login method - supports both email/password and SSO
   */
  async login(credentials: DeveloperLoginRequest, method: AuthMethod = AuthMethods.EMAIL_PASSWORD): Promise<UnifiedAuthResponse> {
    try {
      if (method === AuthMethods.KEYCLOAK_SSO && AUTH_CONFIG.enableKeycloakSSO) {
        // Use Keycloak SSO login
        await keycloakService.login();
        const keycloakState = keycloakService.getAuthState();
        
        if (keycloakState.isAuthenticated && keycloakState.user) {
          this.currentUser = this.convertKeycloakProfile(keycloakState.user);
          this.authMethod = AuthMethods.KEYCLOAK_SSO;
          this.notifyListeners();
          
          return {
            success: true,
            message: 'Successfully logged in via SSO',
            user: this.currentUser,
          };
        }
      } else {
        // Use traditional email/password login
        const response = await authService.login(credentials);
        
        if (response.success && response.data) {
          await authService.fetchDeveloperProfile();
          const developer = authService.getCurrentDeveloper();
          
          if (developer) {
            this.currentUser = this.convertDeveloperProfile(developer);
            this.authMethod = AuthMethods.EMAIL_PASSWORD;
            this.notifyListeners();
            
            return {
              success: true,
              message: response.message,
              user: this.currentUser,
              requiresVerification: !developer.profile_complete,
              requiresWalletActivation: !developer.wallet_activated,
            };
          }
        }
        
        return {
          success: false,
          message: response.message || 'Login failed',
        };
      }
    } catch (error) {
      console.error('Login error:', error);
      return {
        success: false,
        message: 'Login failed due to network error',
      };
    }

    return {
      success: false,
      message: 'Login method not available',
    };
  }

  /**
   * Unified signup method
   */
  async signup(userData: DeveloperSignupRequest): Promise<UnifiedAuthResponse> {
    try {
      if (AUTH_CONFIG.enableKeycloakSSO) {
        // Redirect to Keycloak registration
        await keycloakService.register();
        return {
          success: true,
          message: 'Redirected to SSO registration',
        };
      } else {
        // Use traditional signup
        const response = await authService.signup(userData);
        
        if (response.success && response.data) {
          const developer = authService.getCurrentDeveloper();
          
          if (developer) {
            this.currentUser = this.convertDeveloperProfile(developer);
            this.authMethod = AuthMethods.EMAIL_PASSWORD;
            this.notifyListeners();
            
            return {
              success: true,
              message: response.message,
              user: this.currentUser,
              requiresVerification: AUTH_CONFIG.enableEmailVerification,
              requiresProfileCompletion: !developer.profile_complete,
            };
          }
        }
        
        return {
          success: false,
          message: response.message || 'Signup failed',
        };
      }
    } catch (error) {
      console.error('Signup error:', error);
      return {
        success: false,
        message: 'Signup failed due to network error',
      };
    }
  }

  /**
   * Unified logout method
   */
  async logout(): Promise<void> {
    try {
      if (this.authMethod === AuthMethods.KEYCLOAK_SSO) {
        await keycloakService.logout();
      } else {
        authService.logout();
      }
      
      this.currentUser = null;
      this.authMethod = null;
      this.notifyListeners();
    } catch (error) {
      console.error('Logout error:', error);
    }
  }

  /**
   * Get current user profile
   */
  getCurrentUser(): UnifiedUserProfile | null {
    return this.currentUser;
  }

  /**
   * Check if user is authenticated
   */
  isAuthenticated(): boolean {
    return this.currentUser?.isAuthenticated ?? false;
  }

  /**
   * Check if user has specific role
   */
  hasRole(role: string): boolean {
    return this.currentUser?.roles.includes(role) ?? false;
  }

  /**
   * Check if user has specific permission
   */
  hasPermission(permission: string): boolean {
    return this.currentUser?.permissions.includes(permission) ?? false;
  }

  /**
   * Get authentication method being used
   */
  getAuthMethod(): AuthMethod | null {
    return this.authMethod;
  }

  /**
   * Subscribe to authentication state changes
   */
  subscribe(listener: (user: UnifiedUserProfile | null) => void): () => void {
    this.listeners.push(listener);
    return () => {
      const index = this.listeners.indexOf(listener);
      if (index > -1) {
        this.listeners.splice(index, 1);
      }
    };
  }

  /**
   * Get access token for API calls
   */
  getAccessToken(): string | null {
    if (this.authMethod === AuthMethods.KEYCLOAK_SSO) {
      return keycloakService.getToken();
    } else {
      return authService.getSessionToken();
    }
  }

  /**
   * Refresh authentication token
   */
  async refreshToken(): Promise<boolean> {
    if (this.authMethod === AuthMethods.KEYCLOAK_SSO) {
      return await keycloakService.refreshToken();
    }
    // Traditional auth service doesn't have token refresh
    return false;
  }

  /**
   * Convert Keycloak profile to unified profile
   */
  private convertKeycloakProfile(keycloakUser: BpciUserProfile): UnifiedUserProfile {
    return {
      id: keycloakUser.id || keycloakUser.username || '',
      email: keycloakUser.email || '',
      username: keycloakUser.username,
      firstName: keycloakUser.firstName,
      lastName: keycloakUser.lastName,
      developerId: keycloakUser.developerId,
      organizationId: keycloakUser.organizationId,
      roles: keycloakUser.roles || ['bpi_user'],
      permissions: keycloakUser.permissions || ['explorer_access'],
      walletAddress: keycloakUser.walletAddress,
      walletActivated: !!keycloakUser.walletAddress,
      bpiBalance: keycloakUser.bpiBalance || 0,
      subscriptionTier: keycloakUser.subscriptionTier || 'free',
      isAuthenticated: true,
      profileComplete: !!(keycloakUser.firstName && keycloakUser.lastName),
      emailVerified: keycloakUser.emailVerified ?? false,
      accessToken: keycloakService.getToken() || undefined,
      refreshToken: keycloakService.getRefreshToken() || undefined,
      lastLogin: keycloakUser.lastLogin,
    };
  }

  /**
   * Convert developer profile to unified profile
   */
  private convertDeveloperProfile(developer: any): UnifiedUserProfile {
    return {
      id: developer.developer_id,
      email: developer.email,
      username: developer.email.split('@')[0],
      firstName: developer.name?.split(' ')[0],
      lastName: developer.name?.split(' ').slice(1).join(' '),
      developerId: developer.developer_id,
      roles: ['bpi_user', 'developer'],
      permissions: ['explorer_access', 'wallet_access'],
      walletAddress: developer.wallet_info?.wallet_address,
      walletActivated: developer.wallet_activated,
      bpiBalance: developer.wallet_info?.baby_coin_balance || 0,
      subscriptionTier: 'testnet',
      isAuthenticated: true,
      profileComplete: developer.profile_complete,
      emailVerified: true, // Assume verified if they can login
      sessionToken: authService.getSessionToken() || undefined,
      lastLogin: new Date(developer.last_active),
    };
  }

  /**
   * Notify all listeners of state changes
   */
  private notifyListeners(): void {
    this.listeners.forEach(listener => listener(this.currentUser));
  }
}

// Create singleton instance
export const unifiedAuthService = new UnifiedAuthService();

// Export types and service
export default unifiedAuthService;
export { AUTH_CONFIG };
