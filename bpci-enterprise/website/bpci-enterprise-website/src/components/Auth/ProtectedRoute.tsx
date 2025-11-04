/**
 * Protected Route Component with Keycloak Integration
 * Provides role-based and permission-based route protection
 */

import React from 'react';
import { Navigate, useLocation } from 'react-router-dom';
import { Result, Button, Spin } from 'antd';
import { LockOutlined, UserOutlined, ExclamationCircleOutlined } from '@ant-design/icons';
import { useProtectedRoute, useAuth } from '../../hooks/useKeycloak';

interface ProtectedRouteProps {
  children: React.ReactNode;
  requiredRole?: string;
  requiredPermission?: string;
  fallbackPath?: string;
  showUnauthorized?: boolean;
}

const ProtectedRoute: React.FC<ProtectedRouteProps> = ({
  children,
  requiredRole,
  requiredPermission,
  fallbackPath = '/auth',
  showUnauthorized = true,
}) => {
  const location = useLocation();
  const { login } = useAuth();
  const { isAuthenticated, isAuthorized, isLoading } = useProtectedRoute(
    requiredRole,
    requiredPermission
  );

  // Show loading spinner while checking authentication
  if (isLoading) {
    return (
      <div style={{ 
        display: 'flex', 
        justifyContent: 'center', 
        alignItems: 'center', 
        minHeight: '50vh' 
      }}>
        <Spin size="large" />
      </div>
    );
  }

  // Redirect to authentication if not authenticated
  if (!isAuthenticated) {
    return (
      <Navigate 
        to={fallbackPath} 
        state={{ from: location.pathname }} 
        replace 
      />
    );
  }

  // Show unauthorized message if authenticated but not authorized
  if (!isAuthorized && showUnauthorized) {
    return (
      <Result
        status="403"
        title="Access Denied"
        subTitle={
          requiredRole 
            ? `You need the "${requiredRole}" role to access this page.`
            : requiredPermission
            ? `You need the "${requiredPermission}" permission to access this page.`
            : "You don't have permission to access this page."
        }
        icon={<LockOutlined />}
        extra={[
          <Button 
            type="primary" 
            key="home"
            onClick={() => window.location.href = '/'}
          >
            Go Home
          </Button>,
          <Button 
            key="profile"
            onClick={() => window.open('/auth/profile', '_blank')}
          >
            Manage Account
          </Button>
        ]}
      />
    );
  }

  // Redirect if not authorized and showUnauthorized is false
  if (!isAuthorized && !showUnauthorized) {
    return (
      <Navigate 
        to={fallbackPath} 
        state={{ 
          from: location.pathname,
          error: 'insufficient_permissions'
        }} 
        replace 
      />
    );
  }

  // Render protected content
  return <>{children}</>;
};

// Higher-order component for role-based protection
export const withRoleProtection = (
  Component: React.ComponentType<any>,
  requiredRole: string
) => {
  return (props: any) => (
    <ProtectedRoute requiredRole={requiredRole}>
      <Component {...props} />
    </ProtectedRoute>
  );
};

// Higher-order component for permission-based protection
export const withPermissionProtection = (
  Component: React.ComponentType<any>,
  requiredPermission: string
) => {
  return (props: any) => (
    <ProtectedRoute requiredPermission={requiredPermission}>
      <Component {...props} />
    </ProtectedRoute>
  );
};

// Admin-only route wrapper
export const AdminRoute: React.FC<{ children: React.ReactNode }> = ({ children }) => (
  <ProtectedRoute requiredRole="bpci-admin">
    {children}
  </ProtectedRoute>
);

// Developer-only route wrapper
export const DeveloperRoute: React.FC<{ children: React.ReactNode }> = ({ children }) => (
  <ProtectedRoute requiredRole="bpci-developer">
    {children}
  </ProtectedRoute>
);

// Enterprise-only route wrapper
export const EnterpriseRoute: React.FC<{ children: React.ReactNode }> = ({ children }) => (
  <ProtectedRoute requiredRole="enterprise-user">
    {children}
  </ProtectedRoute>
);

export default ProtectedRoute;
