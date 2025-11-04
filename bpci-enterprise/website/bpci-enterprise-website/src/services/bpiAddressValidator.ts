/**
 * BPI Address Validation System for BPCI Enterprise Dashboard
 * Handles BPI address format validation, checksum verification, and input sanitization
 */

export interface BpiAddressValidationResult {
  isValid: boolean;
  error?: string;
  normalizedAddress?: string;
  addressType?: 'node' | 'wallet' | 'contract' | 'validator';
  network?: 'mainnet' | 'testnet' | 'development';
}

export interface BpiEndpointValidationResult {
  isValid: boolean;
  error?: string;
  normalizedEndpoint?: string;
  protocol?: 'http' | 'https' | 'ws' | 'wss';
  host?: string;
  port?: number;
  isReachable?: boolean;
}

class BpiAddressValidator {
  private static instance: BpiAddressValidator;

  // BPI address patterns
  private readonly ADDRESS_PATTERNS = {
    node: /^bpi:node:[a-fA-F0-9]{40}$/,
    wallet: /^bpi:wallet:[a-fA-F0-9]{40}$/,
    contract: /^bpi:contract:[a-fA-F0-9]{40}$/,
    validator: /^bpi:validator:[a-fA-F0-9]{40}$/,
    generic: /^bpi:[a-zA-Z]+:[a-fA-F0-9]{40}$/,
  };

  // Network prefixes
  private readonly NETWORK_PREFIXES = {
    mainnet: 'bpi:',
    testnet: 'bpit:',
    development: 'bpid:',
  };

  // Valid endpoint protocols
  private readonly VALID_PROTOCOLS = ['http', 'https', 'ws', 'wss'];

  private constructor() {}

  public static getInstance(): BpiAddressValidator {
    if (!BpiAddressValidator.instance) {
      BpiAddressValidator.instance = new BpiAddressValidator();
    }
    return BpiAddressValidator.instance;
  }

  /**
   * Validate BPI address format and checksum
   */
  public validateAddress(address: string): BpiAddressValidationResult {
    try {
      // Sanitize input
      const sanitizedAddress = this.sanitizeInput(address);
      
      if (!sanitizedAddress) {
        return {
          isValid: false,
          error: 'Address cannot be empty',
        };
      }

      // Check basic format
      if (!this.ADDRESS_PATTERNS.generic.test(sanitizedAddress)) {
        return {
          isValid: false,
          error: 'Invalid BPI address format. Expected format: bpi:type:address',
        };
      }

      // Extract components
      const parts = sanitizedAddress.split(':');
      if (parts.length !== 3) {
        return {
          isValid: false,
          error: 'Invalid BPI address structure',
        };
      }

      const [prefix, type, addressHash] = parts;
      
      // Determine network
      const network = this.getNetworkFromPrefix(prefix);
      if (!network) {
        return {
          isValid: false,
          error: 'Invalid network prefix',
        };
      }

      // Validate address type
      const addressType = this.getAddressType(type);
      if (!addressType) {
        return {
          isValid: false,
          error: `Invalid address type: ${type}`,
        };
      }

      // Validate address hash
      if (!this.isValidAddressHash(addressHash)) {
        return {
          isValid: false,
          error: 'Invalid address hash format',
        };
      }

      // Validate checksum
      if (!this.validateChecksum(sanitizedAddress)) {
        return {
          isValid: false,
          error: 'Invalid address checksum',
        };
      }

      return {
        isValid: true,
        normalizedAddress: sanitizedAddress,
        addressType,
        network,
      };
    } catch (error) {
      return {
        isValid: false,
        error: `Validation error: ${error instanceof Error ? error.message : 'Unknown error'}`,
      };
    }
  }

  /**
   * Validate BPI endpoint URL
   */
  public async validateEndpoint(endpoint: string): Promise<BpiEndpointValidationResult> {
    try {
      // Sanitize input
      const sanitizedEndpoint = this.sanitizeInput(endpoint);
      
      if (!sanitizedEndpoint) {
        return {
          isValid: false,
          error: 'Endpoint cannot be empty',
        };
      }

      // Parse URL
      let url: URL;
      try {
        url = new URL(sanitizedEndpoint);
      } catch {
        return {
          isValid: false,
          error: 'Invalid URL format',
        };
      }

      // Validate protocol
      const protocol = url.protocol.slice(0, -1) as 'http' | 'https' | 'ws' | 'wss';
      if (!this.VALID_PROTOCOLS.includes(protocol)) {
        return {
          isValid: false,
          error: `Invalid protocol: ${protocol}. Supported: ${this.VALID_PROTOCOLS.join(', ')}`,
        };
      }

      // Validate host
      if (!url.hostname) {
        return {
          isValid: false,
          error: 'Invalid hostname',
        };
      }

      // Validate port
      const port = url.port ? parseInt(url.port, 10) : this.getDefaultPort(protocol);
      if (port < 1 || port > 65535) {
        return {
          isValid: false,
          error: 'Invalid port number',
        };
      }

      // Check if endpoint is reachable (optional)
      const isReachable = await this.checkEndpointReachability(url);

      return {
        isValid: true,
        normalizedEndpoint: url.toString(),
        protocol,
        host: url.hostname,
        port,
        isReachable,
      };
    } catch (error) {
      return {
        isValid: false,
        error: `Endpoint validation error: ${error instanceof Error ? error.message : 'Unknown error'}`,
      };
    }
  }

  /**
   * Sanitize user input
   */
  public sanitizeInput(input: string): string {
    if (typeof input !== 'string') {
      return '';
    }

    return input
      .trim()
      .toLowerCase()
      .replace(/[^\w:.-]/g, '') // Remove special characters except : . -
      .substring(0, 200); // Limit length
  }

  /**
   * Generate BPI address from public key
   */
  public generateAddressFromPublicKey(publicKey: string, type: string = 'wallet'): string {
    try {
      // Validate public key format
      if (!/^[a-fA-F0-9]{64}$/.test(publicKey)) {
        throw new Error('Invalid public key format');
      }

      // Simple hash generation (in production, use proper cryptographic hash)
      const hash = this.simpleHash(publicKey).substring(0, 40);
      
      return `bpi:${type}:${hash}`;
    } catch (error) {
      throw new Error(`Address generation failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Extract address components
   */
  public parseAddress(address: string): { prefix: string; type: string; hash: string } | null {
    const parts = address.split(':');
    if (parts.length !== 3) {
      return null;
    }

    return {
      prefix: parts[0],
      type: parts[1],
      hash: parts[2],
    };
  }

  /**
   * Check if address is of specific type
   */
  public isAddressType(address: string, type: string): boolean {
    const parsed = this.parseAddress(address);
    return parsed?.type === type;
  }

  /**
   * Get network from address prefix
   */
  private getNetworkFromPrefix(prefix: string): 'mainnet' | 'testnet' | 'development' | null {
    switch (prefix) {
      case 'bpi':
        return 'mainnet';
      case 'bpit':
        return 'testnet';
      case 'bpid':
        return 'development';
      default:
        return null;
    }
  }

  /**
   * Get address type from type string
   */
  private getAddressType(type: string): 'node' | 'wallet' | 'contract' | 'validator' | null {
    const validTypes = ['node', 'wallet', 'contract', 'validator'];
    return validTypes.includes(type) ? type as any : null;
  }

  /**
   * Validate address hash format
   */
  private isValidAddressHash(hash: string): boolean {
    return /^[a-fA-F0-9]{40}$/.test(hash);
  }

  /**
   * Validate address checksum (simplified implementation)
   */
  private validateChecksum(address: string): boolean {
    try {
      // In production, implement proper checksum validation
      // For now, basic validation that address follows pattern
      const parsed = this.parseAddress(address);
      if (!parsed) return false;

      // Simple checksum: last 4 characters should match hash of first part
      const expectedChecksum = this.simpleHash(parsed.prefix + ':' + parsed.type).substring(0, 4);
      const actualChecksum = parsed.hash.substring(36, 40);
      
      return expectedChecksum === actualChecksum;
    } catch {
      return false;
    }
  }

  /**
   * Check endpoint reachability
   */
  private async checkEndpointReachability(url: URL): Promise<boolean> {
    try {
      // For HTTP/HTTPS endpoints
      if (url.protocol === 'http:' || url.protocol === 'https:') {
        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), 5000); // 5 second timeout

        const response = await fetch(`${url.origin}/health`, {
          method: 'HEAD',
          signal: controller.signal,
        });

        clearTimeout(timeoutId);
        return response.ok;
      }

      // For WebSocket endpoints, we can't easily test without connecting
      // Return true for now, actual connection will be tested later
      return true;
    } catch {
      return false;
    }
  }

  /**
   * Get default port for protocol
   */
  private getDefaultPort(protocol: string): number {
    switch (protocol) {
      case 'http':
        return 80;
      case 'https':
        return 443;
      case 'ws':
        return 80;
      case 'wss':
        return 443;
      default:
        return 8080;
    }
  }

  /**
   * Simple hash function (replace with proper cryptographic hash in production)
   */
  private simpleHash(input: string): string {
    let hash = 0;
    for (let i = 0; i < input.length; i++) {
      const char = input.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash; // Convert to 32-bit integer
    }
    
    // Convert to hex and pad
    return Math.abs(hash).toString(16).padStart(8, '0').repeat(5).substring(0, 40);
  }
}

// Validation utility functions
export const validateBpiAddress = (address: string): BpiAddressValidationResult => {
  return BpiAddressValidator.getInstance().validateAddress(address);
};

export const validateBpiEndpoint = async (endpoint: string): Promise<BpiEndpointValidationResult> => {
  return BpiAddressValidator.getInstance().validateEndpoint(endpoint);
};

export const sanitizeBpiInput = (input: string): string => {
  return BpiAddressValidator.getInstance().sanitizeInput(input);
};

export const generateBpiAddress = (publicKey: string, type?: string): string => {
  return BpiAddressValidator.getInstance().generateAddressFromPublicKey(publicKey, type);
};

export default BpiAddressValidator.getInstance();
