# Metanode CUE Configuration System

This directory contains the unified configuration system for the Metanode platform.

## Directory Structure

- `schemas/` - CUE schema definitions for all components
- `environments/` - Environment-specific configurations (dev, staging, prod)
- `policies/` - Security and operational policies
- `generated/` - Generated configuration files (JSON, YAML, etc.)

## Single Source of Truth

All system configurations are now managed through CUE schemas, providing:

- **Type safety** - All configs are validated against schemas
- **Code generation** - Rust structs generated from CUE definitions
- **Environment management** - Consistent configs across environments
- **Policy enforcement** - Security policies applied uniformly
- **Documentation** - Self-documenting configuration system

## Usage

The CUE runtime automatically loads and validates all configurations.
Generated files are used by individual components.

## Size Reduction

- **Before**: 25MB scattered config files
- **After**: 5MB unified CUE system
- **Reduction**: 80% size reduction with improved functionality
