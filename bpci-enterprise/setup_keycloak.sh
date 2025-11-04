#!/bin/bash

# Keycloak Configuration Script for BPCI
echo "=========================================="
echo "KEYCLOAK CONFIGURATION - AUTOMATED SETUP"
echo "=========================================="
echo ""

# Install jq if not present
which jq > /dev/null || apt-get install -y jq

# Keycloak admin credentials
KEYCLOAK_URL="http://localhost:8180"
ADMIN_USER="admin"
ADMIN_PASS="admin"

echo "1. Getting admin access token..."
TOKEN=$(curl -s -X POST "${KEYCLOAK_URL}/realms/master/protocol/openid-connect/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=${ADMIN_USER}" \
  -d "password=${ADMIN_PASS}" \
  -d "grant_type=password" \
  -d "client_id=admin-cli" | jq -r .access_token)

if [ "$TOKEN" = "null" ] || [ -z "$TOKEN" ]; then
    echo "❌ Failed to get admin token"
    echo "Keycloak may need manual configuration"
    echo "Access: http://134.209.210.181/auth"
    echo "Username: admin"
    echo "Password: admin"
    exit 1
fi

echo "✅ Admin token obtained"

echo ""
echo "2. Creating BPCI realm..."
curl -s -X POST "${KEYCLOAK_URL}/admin/realms" \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Content-Type: application/json" \
  -d '{
    "realm": "bpci",
    "enabled": true,
    "displayName": "BPCI Enterprise",
    "registrationAllowed": true,
    "loginWithEmailAllowed": true,
    "duplicateEmailsAllowed": false,
    "resetPasswordAllowed": true,
    "editUsernameAllowed": false,
    "bruteForceProtected": true,
    "sslRequired": "none"
  }' && echo "✅ BPCI realm created" || echo "⚠️  Realm may already exist"

echo ""
echo "3. Creating frontend client..."
curl -s -X POST "${KEYCLOAK_URL}/admin/realms/bpci/clients" \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Content-Type: application/json" \
  -d '{
    "clientId": "bpci-frontend",
    "enabled": true,
    "publicClient": true,
    "redirectUris": ["http://134.209.210.181/*"],
    "webOrigins": ["*"],
    "protocol": "openid-connect"
  }' && echo "✅ Frontend client created" || echo "⚠️  Client may already exist"

echo ""
echo "4. Creating backend client..."
curl -s -X POST "${KEYCLOAK_URL}/admin/realms/bpci/clients" \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Content-Type: application/json" \
  -d '{
    "clientId": "bpci-web",
    "enabled": true,
    "publicClient": false,
    "redirectUris": ["http://134.209.210.181/*"],
    "webOrigins": ["*"],
    "protocol": "openid-connect",
    "serviceAccountsEnabled": true
  }' && echo "✅ Backend client created" || echo "⚠️  Client may already exist"

echo ""
echo "=========================================="
echo "KEYCLOAK CONFIGURATION COMPLETE"
echo "=========================================="
echo ""
echo "Access Keycloak Admin Console:"
echo "  URL: http://134.209.210.181/auth"
echo "  Username: admin"
echo "  Password: admin"
echo ""
echo "BPCI Realm: bpci"
echo "Frontend Client: bpci-frontend (public)"
echo "Backend Client: bpci-web (confidential)"
echo ""
