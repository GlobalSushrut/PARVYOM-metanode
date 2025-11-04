/**
 * Unified Authentication Hook for BPCI Enterprise
 * Provides React integration for unified authentication service
 * Manages authentication state, user profile, and auth methods
 */

import { useState, useEffect, useCallback } from 'react';
import { unifiedAuthService, AuthMethods } from '../services/unifiedAuthService';
import type { AuthMethod, UnifiedUserProfile, UnifiedAuthResponse } from '../services/unifiedAuthService';
import type { DeveloperLoginRequest, DeveloperSignupRequest } from '../services/authService';

interface UseUnifiedAuthReturn {
  // Authentication state
  user: UnifiedUserProfile | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  authMethod: AuthMethod | null;
  
  // Authentication actions
  login: (credentials: DeveloperLoginRequest, method?: AuthMethod) => Promise<UnifiedAuthResponse>;
  signup: (userData: DeveloperSignupRequest) => Promise<UnifiedAuthResponse>;
  logout: () => Promise<void>;
  
  // Token management
  getAccessToken: () => string | null;
  refreshToken: () => Promise<boolean>;
  
  // Permission checks
  hasRole: (role: string) => boolean;
  hasPermission: (permission: string) => boolean;
  
  // Profile checks
  isProfileComplete: boolean;
  isEmailVerified: boolean;
  isWalletActivated: boolean;
  
  // Subscription info
  subscriptionTier: 'free' | 'testnet' | 'pilot' | 'enterprise';
  bpiBalance: number;
}

/**
 * Custom hook for unified authentication
 */
export const useUnifiedAuth = (): UseUnifiedAuthReturn => {
  const [user, setUser] = useState<UnifiedUserProfile | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [authMethod, setAuthMethod] = useState<AuthMethod | null>(null);

  useEffect(() => {
    // Initialize authentication state
    const initializeAuth = async () => {
      try {
        setIsLoading(true);
        
        // Get current user from service
        const currentUser = unifiedAuthService.getCurrentUser();
        const currentMethod = unifiedAuthService.getAuthMethod();
        
        setUser(currentUser);
        setAuthMethod(currentMethod);
      } catch (error) {
        console.error('Failed to initialize auth:', error);
      } finally {
        setIsLoading(false);
      }
    };

    initializeAuth();

    // Subscribe to authentication state changes
    const unsubscribe = unifiedAuthService.subscribe((updatedUser) => {
      setUser(updatedUser);
      setAuthMethod(unifiedAuthService.getAuthMethod());
      setIsLoading(false);
    });

    return unsubscribe;
  }, []);

  /**
   * Login with email/password or SSO
   */
  const login = useCallback(async (
    credentials: DeveloperLoginRequest,
    method: AuthMethod = AuthMethods.EMAIL_PASSWORD
  ): Promise<UnifiedAuthResponse> => {
    setIsLoading(true);
    try {
      const response = await unifiedAuthService.login(credentials, method);
      return response;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Sign up new user
   */
  const signup = useCallback(async (userData: DeveloperSignupRequest): Promise<UnifiedAuthResponse> => {
    setIsLoading(true);
    try {
      const response = await unifiedAuthService.signup(userData);
      return response;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Logout user
   */
  const logout = useCallback(async (): Promise<void> => {
    setIsLoading(true);
    try {
      await unifiedAuthService.logout();
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Get access token for API calls
   */
  const getAccessToken = useCallback((): string | null => {
    return unifiedAuthService.getAccessToken();
  }, []);

  /**
   * Refresh authentication token
   */
  const refreshToken = useCallback(async (): Promise<boolean> => {
    return await unifiedAuthService.refreshToken();
  }, []);

  /**
   * Check if user has specific role
   */
  const hasRole = useCallback((role: string): boolean => {
    return unifiedAuthService.hasRole(role);
  }, []);

  /**
   * Check if user has specific permission
   */
  const hasPermission = useCallback((permission: string): boolean => {
    return unifiedAuthService.hasPermission(permission);
  }, []);

  // Computed properties
  const isAuthenticated = user?.isAuthenticated ?? false;
  const isProfileComplete = user?.profileComplete ?? false;
  const isEmailVerified = user?.emailVerified ?? false;
  const isWalletActivated = user?.walletActivated ?? false;
  const subscriptionTier = user?.subscriptionTier ?? 'free';
  const bpiBalance = user?.bpiBalance ?? 0;

  return {
    // Authentication state
    user,
    isAuthenticated,
    isLoading,
    authMethod,
    
    // Authentication actions
    login,
    signup,
    logout,
    
    // Token management
    getAccessToken,
    refreshToken,
    
    // Permission checks
    hasRole,
    hasPermission,
    
    // Profile checks
    isProfileComplete,
    isEmailVerified,
    isWalletActivated,
    
    // Subscription info
    subscriptionTier,
    bpiBalance,
  };
};

export default useUnifiedAuth;
