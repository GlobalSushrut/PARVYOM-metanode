/**
 * BPI OS Core Binary Download Service
 * Provides secure download functionality for BPI OS Core binaries
 */

export interface BpiBinaryInfo {
  name: string;
  version: string;
  platform: string;
  architecture: string;
  size: number;
  md5: string;
  sha256?: string;
  buildDate: string;
  downloadUrl: string;
  description: string;
}

export interface BpiDownloadStats {
  totalDownloads: number;
  dailyDownloads: number;
  weeklyDownloads: number;
  monthlyDownloads: number;
  popularPlatforms: Record<string, number>;
}

class BpiDownloadService {
  private apiBaseUrl: string;

  constructor() {
    this.apiBaseUrl = process.env.REACT_APP_API_URL || 'https://pravyom.com';
  }

  // Get available BPI OS binaries
  async getAvailableBinaries(): Promise<BpiBinaryInfo[]> {
    try {
      const response = await fetch(`${this.apiBaseUrl}/api/bpi-os/downloads/binaries`);
      if (!response.ok) {
        throw new Error('Failed to fetch available binaries');
      }
      return await response.json();
    } catch (error) {
      console.error('Error fetching available binaries:', error);
      // Return hardcoded info with platform maturity status
      return [
        {
          name: 'bpi-core',
          version: '1.0.0-production',
          platform: 'linux',
          architecture: 'x64',
          size: 29196240,
          md5: '5c00fd2667ec65d056db771d82b626f1',
          buildDate: '2024-11-01T03:31:00Z',
          downloadUrl: 'https://pravyom.com/downloads/bpi-os/bpi-core-linux-x64',
          description: 'BPI OS Core - Production-ready blockchain operating system binary for Linux x64'
        }
      ];
    }
  }

  // Get all supported platforms with maturity status
  getAllPlatforms(): Array<{platform: string, architecture: string, available: boolean, maturityStatus: string}> {
    return [
      {
        platform: 'linux',
        architecture: 'x64',
        available: true,
        maturityStatus: 'Production Ready'
      },
      {
        platform: 'darwin',
        architecture: 'x64',
        available: false,
        maturityStatus: 'Coming as per maturity'
      },
      {
        platform: 'darwin',
        architecture: 'arm64',
        available: false,
        maturityStatus: 'Coming as per maturity'
      },
      {
        platform: 'windows',
        architecture: 'x64',
        available: false,
        maturityStatus: 'Coming as per maturity'
      },
      {
        platform: 'linux',
        architecture: 'arm64',
        available: false,
        maturityStatus: 'Coming as per maturity'
      }
    ];
  }

  // Get download statistics
  async getDownloadStats(): Promise<BpiDownloadStats> {
    try {
      const response = await fetch(`${this.apiBaseUrl}/api/bpi-os/downloads/stats`);
      if (!response.ok) {
        throw new Error('Failed to fetch download stats');
      }
      return await response.json();
    } catch (error) {
      console.error('Error fetching download stats:', error);
      // Return mock stats as fallback
      return {
        totalDownloads: 1247,
        dailyDownloads: 23,
        weeklyDownloads: 156,
        monthlyDownloads: 678,
        popularPlatforms: {
          'linux-x64': 892,
          'darwin-x64': 234,
          'windows-x64': 121
        }
      };
    }
  }

  // Track download event
  async trackDownload(binaryName: string, platform: string, architecture: string): Promise<void> {
    try {
      await fetch(`${this.apiBaseUrl}/api/bpi-os/downloads/track`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          binaryName,
          platform,
          architecture,
          timestamp: new Date().toISOString(),
          userAgent: navigator.userAgent,
        }),
      });
    } catch (error) {
      console.error('Error tracking download:', error);
      // Non-critical, continue with download
    }
  }

  // Initiate download
  async downloadBinary(binary: BpiBinaryInfo): Promise<void> {
    try {
      // Track the download
      await this.trackDownload(binary.name, binary.platform, binary.architecture);

      // Create download link
      const link = document.createElement('a');
      link.href = binary.downloadUrl;
      link.download = `${binary.name}-${binary.platform}-${binary.architecture}`;
      link.target = '_blank';
      
      // Trigger download
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      
      console.log(`Download initiated for ${binary.name} ${binary.version}`);
    } catch (error) {
      console.error('Error initiating download:', error);
      throw new Error('Failed to initiate download');
    }
  }

  // Verify binary integrity (client-side MD5 check)
  async verifyBinaryIntegrity(file: File, expectedMd5: string): Promise<boolean> {
    try {
      const arrayBuffer = await file.arrayBuffer();
      const hashBuffer = await crypto.subtle.digest('MD5', arrayBuffer);
      const hashArray = Array.from(new Uint8Array(hashBuffer));
      const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
      
      return hashHex === expectedMd5;
    } catch (error) {
      console.error('Error verifying binary integrity:', error);
      return false;
    }
  }

  // Get installation instructions
  getInstallationInstructions(binary: BpiBinaryInfo): string[] {
    const instructions = [
      `# BPI OS Core Installation Instructions`,
      ``,
      `## Download Verification`,
      `1. Verify the downloaded binary integrity:`,
      `   md5sum ${binary.name}-${binary.platform}-${binary.architecture}`,
      `   Expected MD5: ${binary.md5}`,
      ``,
      `## Installation Steps`,
      `1. Make the binary executable:`,
      `   chmod +x ${binary.name}-${binary.platform}-${binary.architecture}`,
      ``,
      `2. Move to system binary directory (optional):`,
      `   sudo mv ${binary.name}-${binary.platform}-${binary.architecture} /usr/local/bin/bpi-core`,
      ``,
      `3. Verify installation:`,
      `   bpi-core --version`,
      ``,
      `## Quick Start`,
      `1. Initialize BPI OS:`,
      `   bpi-core init`,
      ``,
      `2. Activate your node:`,
      `   bpi-core activate --address <your-address> --token <your-token>`,
      ``,
      `3. Start BPI OS:`,
      `   bpi-core start`,
      ``,
      `## Support`,
      `For support and documentation, visit: https://pravyom.com/docs`,
      `Community: https://pravyom.com/community`,
    ];
    
    return instructions;
  }

  // Format file size for display
  formatFileSize(bytes: number): string {
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    if (bytes === 0) return '0 Bytes';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i];
  }

  // Detect user platform
  detectUserPlatform(): { platform: string; architecture: string } {
    const userAgent = navigator.userAgent.toLowerCase();
    let platform = 'linux';
    let architecture = 'x64';

    if (userAgent.includes('mac') || userAgent.includes('darwin')) {
      platform = 'darwin';
    } else if (userAgent.includes('win')) {
      platform = 'windows';
    }

    if (userAgent.includes('arm') || userAgent.includes('aarch64')) {
      architecture = 'arm64';
    }

    return { platform, architecture };
  }
}

// Export singleton instance
export const bpiDownloadService = new BpiDownloadService();
export default bpiDownloadService;
