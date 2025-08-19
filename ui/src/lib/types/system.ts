// BPI System Types based on the component specifications

export interface SystemStatus {
	health: 'healthy' | 'warning' | 'error' | 'offline';
	uptime: number;
	version: string;
	nodeId: string;
	timestamp: string;
	services: ServiceStatus[];
}

export interface ServiceStatus {
	name: string;
	status: 'running' | 'stopped' | 'error' | 'starting';
	port?: number;
	pid?: number;
	memory?: number;
	cpu?: number;
	lastRestart?: string;
}

export interface NodeMetrics {
	cpu: {
		usage: number;
		cores: number;
		loadAverage: number[];
	};
	memory: {
		total: number;
		used: number;
		free: number;
		cached: number;
		percentage: number;
	};
	disk: {
		total: number;
		used: number;
		free: number;
		percentage: number;
	};
	network: {
		bytesIn: number;
		bytesOut: number;
		packetsIn: number;
		packetsOut: number;
		connections: number;
	};
}

export interface NetworkInfo {
	meshStatus: 'connected' | 'connecting' | 'disconnected' | 'error';
	peers: PeerInfo[];
	topology: NetworkTopology;
	latency: number;
	bandwidth: {
		upload: number;
		download: number;
	};
}

export interface PeerInfo {
	id: string;
	address: string;
	status: 'connected' | 'connecting' | 'disconnected';
	latency: number;
	lastSeen: string;
	version: string;
	capabilities: string[];
}

export interface NetworkTopology {
	nodes: NetworkNode[];
	edges: NetworkEdge[];
}

export interface NetworkNode {
	id: string;
	label: string;
	type: 'local' | 'peer' | 'gateway';
	status: 'online' | 'offline' | 'warning';
	x?: number;
	y?: number;
}

export interface NetworkEdge {
	from: string;
	to: string;
	weight: number;
	status: 'active' | 'inactive' | 'error';
}

// DockLock Container Types
export interface ContainerInfo {
	id: string;
	name: string;
	image: string;
	status: 'running' | 'stopped' | 'error' | 'starting' | 'stopping';
	created: string;
	ports: PortMapping[];
	resources: ContainerResources;
	health: HealthCheck;
}

export interface PortMapping {
	internal: number;
	external: number;
	protocol: 'tcp' | 'udp';
}

export interface ContainerResources {
	cpuUsage: number;
	memoryUsage: number;
	memoryLimit: number;
	networkIO: {
		rx: number;
		tx: number;
	};
	diskIO: {
		read: number;
		write: number;
	};
}

export interface HealthCheck {
	status: 'healthy' | 'unhealthy' | 'starting' | 'none';
	failingStreak: number;
	lastCheck: string;
}

// Security & Compliance Types
export interface SecurityStatus {
	deterministicCage: {
		active: boolean;
		violations: number;
		lastViolation?: string;
	};
	busBios: {
		mode: 'normal' | 'secure' | 'emergency' | 'maintenance';
		policyViolations: number;
		lastPolicyCheck: string;
	};
	encryption: {
		enabled: boolean;
		algorithm: string;
		keyRotation: string;
	};
}

// Wallet & Economics Types
export interface WalletStatus {
	address: string;
	balance: {
		gen: number;
		nex: number;
		flx: number;
		aur: number;
	};
	transactions: {
		pending: number;
		confirmed: number;
		failed: number;
	};
	staking: {
		amount: number;
		rewards: number;
		validators: number;
	};
}

// Event & Alert Types
export interface SystemEvent {
	id: string;
	timestamp: string;
	type: 'info' | 'warning' | 'error' | 'success';
	category: 'system' | 'network' | 'container' | 'security' | 'wallet';
	title: string;
	message: string;
	details?: Record<string, any>;
}

export interface SystemAlert {
	id: string;
	severity: 'low' | 'medium' | 'high' | 'critical';
	type: string;
	title: string;
	message: string;
	timestamp: string;
	acknowledged: boolean;
	resolved: boolean;
}
