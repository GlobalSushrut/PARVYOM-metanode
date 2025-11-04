/**
 * Role-Based Access Control (RBAC) Component for BPCI Enterprise Dashboard
 * Manages user permissions and dashboard section access based on roles
 */

import React, { createContext, useContext, useEffect, useState } from 'react';
import { Alert, Spin } from 'antd';
import AuthTokenManager from '../../services/authTokenManager';
import type { AuthUser, UserRole } from '../../services/authTokenManager';

// Role definitions with permissions
export const ROLES = {
  admin: {
    role: 'admin' as const,
    permissions: [
      'system:read',
      'system:write',
      'system:admin',
      'bpi:connect',
      'bpi:configure',
      'bpi:monitor',
      'users:manage',
      'registry:admin',
      'auction:admin',
      'security:admin',
    ],
    bpiAccess: true,
    dashboardSections: [
      'system-overview',
      'bpi-integration',
      'registry-management',
      'auction-monitoring',
      'user-management',
      'security-panel',
      'system-logs',
      'configuration',
    ],
  },
  validator: {
    role: 'validator' as const,
    permissions: [
      'system:read',
      'bpi:connect',
      'bpi:monitor',
      'registry:read',
      'registry:write',
      'auction:participate',
      'node:manage',
    ],
    bpiAccess: true,
    dashboardSections: [
      'system-overview',
      'bpi-integration',
      'registry-management',
      'auction-monitoring',
      'node-management',
      'validator-metrics',
    ],
  },
  user: {
    role: 'user' as const,
    permissions: [
      'system:read',
      'bpi:monitor',
      'registry:read',
      'wallet:manage',
      'profile:manage',
    ],
    bpiAccess: false,
    dashboardSections: [
      'system-overview',
      'registry-view',
      'wallet-management',
      'profile-settings',
    ],
  },
  guest: {
    role: 'guest' as const,
    permissions: [
      'system:read',
    ],
    bpiAccess: false,
    dashboardSections: [
      'public-metrics',
    ],
  },
} as const;

// Permission categories
export const PERMISSIONS = {
  SYSTEM: {
    READ: 'system:read',
    WRITE: 'system:write',
    ADMIN: 'system:admin',
  },
  BPI: {
    CONNECT: 'bpi:connect',
    CONFIGURE: 'bpi:configure',
    MONITOR: 'bpi:monitor',
  },
  REGISTRY: {
    READ: 'registry:read',
    WRITE: 'registry:write',
    ADMIN: 'registry:admin',
  },
  AUCTION: {
    PARTICIPATE: 'auction:participate',
    ADMIN: 'auction:admin',
  },
  USERS: {
    MANAGE: 'users:manage',
  },
  SECURITY: {
    ADMIN: 'security:admin',
  },
  NODE: {
    MANAGE: 'node:manage',
  },
  WALLET: {
    MANAGE: 'wallet:manage',
  },
  PROFILE: {
    MANAGE: 'profile:manage',
  },
} as const;

// Dashboard sections
export const DASHBOARD_SECTIONS = {
  SYSTEM_OVERVIEW: 'system-overview',
  BPI_INTEGRATION: 'bpi-integration',
  REGISTRY_MANAGEMENT: 'registry-management',
  REGISTRY_VIEW: 'registry-view',
  AUCTION_MONITORING: 'auction-monitoring',
  USER_MANAGEMENT: 'user-management',
  SECURITY_PANEL: 'security-panel',
  SYSTEM_LOGS: 'system-logs',
  CONFIGURATION: 'configuration',
  NODE_MANAGEMENT: 'node-management',
  VALIDATOR_METRICS: 'validator-metrics',
  WALLET_MANAGEMENT: 'wallet-management',
  PROFILE_SETTINGS: 'profile-settings',
  PUBLIC_METRICS: 'public-metrics',
} as const;

interface RBACContextType {
  user: AuthUser | null;
  loading: boolean;
  hasPermission: (permission: string) => boolean;
  hasAnyPermission: (permissions: string[]) => boolean;
  hasAllPermissions: (permissions: string[]) => boolean;
  canAccessSection: (section: string) => boolean;
  hasBpiAccess: () => boolean;
  isRole: (role: string) => boolean;
  refreshAuth: () => Promise<void>;
}

const RBACContext = createContext<RBACContextType | undefined>(undefined);

export const useRBAC = (): RBACContextType => {
  const context = useContext(RBACContext);
  if (!context) {
    throw new Error('useRBAC must be used within an RBACProvider');
  }
  return context;
};

interface RBACProviderProps {
  children: React.ReactNode;
}

export const RBACProvider: React.FC<RBACProviderProps> = ({ children }) => {
  const [user, setUser] = useState<AuthUser | null>(null);
  const [loading, setLoading] = useState(true);

  const loadUser = async () => {
    try {
      setLoading(true);
      const currentUser = AuthTokenManager.getCurrentUser();
      
      if (currentUser) {
        // Validate token
        const isValid = await AuthTokenManager.validateToken();
        if (isValid) {
          setUser(currentUser);
        } else {
          setUser(null);
        }
      } else {
        setUser(null);
      }
    } catch (error) {
      console.error('Failed to load user:', error);
      setUser(null);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadUser();
  }, []);

  const hasPermission = (permission: string): boolean => {
    return user?.role.permissions.includes(permission) || false;
  };

  const hasAnyPermission = (permissions: string[]): boolean => {
    return permissions.some(permission => hasPermission(permission));
  };

  const hasAllPermissions = (permissions: string[]): boolean => {
    return permissions.every(permission => hasPermission(permission));
  };

  const canAccessSection = (section: string): boolean => {
    return user?.role.dashboardSections.includes(section) || false;
  };

  const hasBpiAccess = (): boolean => {
    return user?.role.bpiAccess || false;
  };

  const isRole = (role: string): boolean => {
    return user?.role.role === role;
  };

  const refreshAuth = async (): Promise<void> => {
    await loadUser();
  };

  const contextValue: RBACContextType = {
    user,
    loading,
    hasPermission,
    hasAnyPermission,
    hasAllPermissions,
    canAccessSection,
    hasBpiAccess,
    isRole,
    refreshAuth,
  };

  return (
    <RBACContext.Provider value={contextValue}>
      {children}
    </RBACContext.Provider>
  );
};

interface ProtectedComponentProps {
  children: React.ReactNode;
  permission?: string;
  permissions?: string[];
  requireAll?: boolean;
  section?: string;
  role?: string;
  requireBpiAccess?: boolean;
  fallback?: React.ReactNode;
  showError?: boolean;
}

export const ProtectedComponent: React.FC<ProtectedComponentProps> = ({
  children,
  permission,
  permissions = [],
  requireAll = false,
  section,
  role,
  requireBpiAccess = false,
  fallback = null,
  showError = false,
}) => {
  const { user, loading, hasPermission, hasAnyPermission, hasAllPermissions, canAccessSection, hasBpiAccess, isRole } = useRBAC();

  if (loading) {
    return <Spin size="small" />;
  }

  if (!user) {
    return showError ? (
      <Alert
        message="Authentication Required"
        description="Please log in to access this feature."
        type="warning"
        showIcon
      />
    ) : fallback;
  }

  // Check role requirement
  if (role && !isRole(role)) {
    return showError ? (
      <Alert
        message="Insufficient Role"
        description={`This feature requires ${role} role.`}
        type="error"
        showIcon
      />
    ) : fallback;
  }

  // Check BPI access requirement
  if (requireBpiAccess && !hasBpiAccess()) {
    return showError ? (
      <Alert
        message="BPI Access Required"
        description="This feature requires BPI infrastructure access."
        type="error"
        showIcon
      />
    ) : fallback;
  }

  // Check section access
  if (section && !canAccessSection(section)) {
    return showError ? (
      <Alert
        message="Section Access Denied"
        description="You don't have permission to access this dashboard section."
        type="error"
        showIcon
      />
    ) : fallback;
  }

  // Check single permission
  if (permission && !hasPermission(permission)) {
    return showError ? (
      <Alert
        message="Permission Denied"
        description={`This feature requires ${permission} permission.`}
        type="error"
        showIcon
      />
    ) : fallback;
  }

  // Check multiple permissions
  if (permissions.length > 0) {
    const hasAccess = requireAll 
      ? hasAllPermissions(permissions)
      : hasAnyPermission(permissions);

    if (!hasAccess) {
      return showError ? (
        <Alert
          message="Permission Denied"
          description={`This feature requires ${requireAll ? 'all' : 'one'} of the following permissions: ${permissions.join(', ')}`}
          type="error"
          showIcon
        />
      ) : fallback;
    }
  }

  return <>{children}</>;
};

interface RoleGuardProps {
  allowedRoles: string[];
  children: React.ReactNode;
  fallback?: React.ReactNode;
}

export const RoleGuard: React.FC<RoleGuardProps> = ({
  allowedRoles,
  children,
  fallback = null,
}) => {
  const { user, loading } = useRBAC();

  if (loading) {
    return <Spin size="small" />;
  }

  if (!user || !allowedRoles.includes(user.role.role)) {
    return fallback;
  }

  return <>{children}</>;
};

interface BpiAccessGuardProps {
  children: React.ReactNode;
  fallback?: React.ReactNode;
}

export const BpiAccessGuard: React.FC<BpiAccessGuardProps> = ({
  children,
  fallback = null,
}) => {
  const { hasBpiAccess, loading } = useRBAC();

  if (loading) {
    return <Spin size="small" />;
  }

  if (!hasBpiAccess()) {
    return fallback;
  }

  return <>{children}</>;
};

// Utility function to get role configuration
export const getRoleConfig = (roleName: string): UserRole | null => {
  const role = ROLES[roleName as keyof typeof ROLES];
  return role ? { 
    ...role, 
    permissions: [...role.permissions],
    dashboardSections: [...role.dashboardSections]
  } : null;
};

// Utility function to check if role exists
export const isValidRole = (roleName: string): boolean => {
  return roleName in ROLES;
};

export default RBACProvider;
