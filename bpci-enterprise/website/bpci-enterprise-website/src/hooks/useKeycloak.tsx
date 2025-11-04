/**
 * React Hooks for Keycloak Integration
 * Provides easy-to-use hooks for authentication state and operations
 */

import { useState, useEffect, useContext, createContext } from 'react';
import type { ReactNode } from 'react';
import keycloakService from '../services/keycloakService';
import type { AuthState, BpciUserProfile } from '../services/keycloakService';

// Keycloak Context
interface KeycloakContextType {
  authState: AuthState;
  login: (redirectUri?: string) => Promise<void>;
  logout: (redirectUri?: string) => Promise<void>;
  register: (redirectUri?: string) => Promise<void>;
  updateProfile: () => Promise<BpciUserProfile | null>;
  hasRole: (role: string) => boolean;
  hasPermission: (permission: string) => boolean;
  getToken: () => string | null;
  refreshToken: () => Promise<boolean>;
  getAccountManagementUrl: () => string;
  isTokenExpired: () => boolean;
}

const KeycloakContext = createContext<KeycloakContextType | null>(null);

// Keycloak Provider Component
interface KeycloakProviderProps {
  children: ReactNode;
}

export const KeycloakProvider: React.FC<KeycloakProviderProps> = ({ children }) => {
  const [authState, setAuthState] = useState<AuthState>(keycloakService.getAuthState());

  useEffect(() => {
    // Initialize Keycloak
    const initKeycloak = async () => {
      try {
        await keycloakService.init();
      } catch (error) {
        console.error('Failed to initialize Keycloak:', error);
      }
    };

    initKeycloak();

    // Subscribe to auth state changes
    const unsubscribe = keycloakService.subscribe((newState) => {
      setAuthState(newState);
    });

    return unsubscribe;
  }, []);

  const contextValue: KeycloakContextType = {
    authState,
    login: async (redirectUri?: string) => {
      await keycloakService.login({ redirectUri });
    },
    logout: async (redirectUri?: string) => {
      await keycloakService.logout({ redirectUri });
    },
    register: async (redirectUri?: string) => {
      await keycloakService.register({ redirectUri });
    },
    updateProfile: () => keycloakService.updateProfile(),
    hasRole: (role: string) => keycloakService.hasRole(role),
    hasPermission: (permission: string) => keycloakService.hasPermission(permission),
    getToken: () => keycloakService.getToken(),
    refreshToken: () => keycloakService.refreshToken(),
    getAccountManagementUrl: () => keycloakService.getAccountManagementUrl(),
    isTokenExpired: () => keycloakService.isTokenExpired(),
  };

  return (
    <KeycloakContext.Provider value={contextValue}>
      {children}
    </KeycloakContext.Provider>
  );
};

// Main Keycloak Hook
export const useKeycloak = (): KeycloakContextType => {
  const context = useContext(KeycloakContext);
  if (!context) {
    throw new Error('useKeycloak must be used within a KeycloakProvider');
  }
  return context;
};

// Authentication Hook
export const useAuth = () => {
  const { authState, login, logout, register, updateProfile } = useKeycloak();
  
  return {
    isAuthenticated: authState.isAuthenticated,
    isLoading: authState.isLoading,
    user: authState.user,
    token: authState.token,
    roles: authState.roles,
    permissions: authState.permissions,
    login,
    logout,
    register,
    updateProfile,
  };
};

// User Profile Hook
export const useUserProfile = () => {
  const { authState, updateProfile } = useKeycloak();
  
  return {
    user: authState.user,
    updateProfile,
    isLoading: authState.isLoading,
  };
};

// Role-based Access Control Hook
export const useRBAC = () => {
  const { hasRole, hasPermission, authState } = useKeycloak();
  
  return {
    hasRole,
    hasPermission,
    roles: authState.roles,
    permissions: authState.permissions,
    isAdmin: hasRole('admin'),
    isDeveloper: hasRole('developer'),
    isEnterpriseUser: hasRole('enterprise_user'),
  };
};

// Token Management Hook
export const useToken = () => {
  const { getToken, refreshToken, isTokenExpired, authState } = useKeycloak();
  
  const [isRefreshing, setIsRefreshing] = useState(false);
  
  const ensureValidToken = async (): Promise<string | null> => {
    const token = getToken();
    
    if (!token) return null;
    
    if (isTokenExpired()) {
      setIsRefreshing(true);
      try {
        const refreshed = await refreshToken();
        if (refreshed) {
          return getToken();
        }
        return null;
      } finally {
        setIsRefreshing(false);
      }
    }
    
    return token;
  };
  
  return {
    token: authState.token,
    getToken,
    refreshToken,
    isTokenExpired,
    ensureValidToken,
    isRefreshing,
  };
};

// Protected Route Hook
export const useProtectedRoute = (requiredRole?: string, requiredPermission?: string) => {
  const { authState, hasRole, hasPermission } = useKeycloak();
  const [isAuthorized, setIsAuthorized] = useState<boolean | null>(null);
  
  useEffect(() => {
    if (authState.isLoading) {
      setIsAuthorized(null);
      return;
    }
    
    if (!authState.isAuthenticated) {
      setIsAuthorized(false);
      return;
    }
    
    let authorized = true;
    
    if (requiredRole && !hasRole(requiredRole)) {
      authorized = false;
    }
    
    if (requiredPermission && !hasPermission(requiredPermission)) {
      authorized = false;
    }
    
    setIsAuthorized(authorized);
  }, [authState, requiredRole, requiredPermission, hasRole, hasPermission]);
  
  return {
    isAuthorized,
    isLoading: authState.isLoading,
    isAuthenticated: authState.isAuthenticated,
    user: authState.user,
  };
};

// Account Management Hook
export const useAccountManagement = () => {
  const { getAccountManagementUrl, authState } = useKeycloak();
  
  const openAccountManagement = () => {
    const url = getAccountManagementUrl();
    window.open(url, '_blank');
  };
  
  return {
    openAccountManagement,
    accountManagementUrl: getAccountManagementUrl(),
    canManageAccount: authState.isAuthenticated,
  };
};
