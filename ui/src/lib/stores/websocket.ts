import { writable } from 'svelte/store';
import { systemStore } from './system';
import type { SystemEvent, SystemAlert } from '$lib/types/system';

interface WebSocketState {
	connected: boolean;
	connecting: boolean;
	error: string | null;
	lastMessage: Date | null;
	reconnectAttempts: number;
}

const initialState: WebSocketState = {
	connected: false,
	connecting: false,
	error: null,
	lastMessage: null,
	reconnectAttempts: 0
};

function createWebSocketStore() {
	const { subscribe, set, update } = writable<WebSocketState>(initialState);
	
	let ws: WebSocket | null = null;
	let reconnectTimer: ReturnType<typeof setTimeout> | null = null;
	let heartbeatTimer: ReturnType<typeof setTimeout> | null = null;
	
	const maxReconnectAttempts = 10;
	const reconnectDelay = 5000; // 5 seconds
	const heartbeatInterval = 30000; // 30 seconds

	return {
		subscribe,
		
		// Connect to BPI gateway WebSocket
		connect() {
			if (ws?.readyState === WebSocket.OPEN) return;
			
			update(state => ({ ...state, connecting: true, error: null }));
			
			try {
				// Use secure WebSocket if HTTPS, otherwise regular WebSocket
				const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
				const wsUrl = `${protocol}//${window.location.host}/ws`;
				
				ws = new WebSocket(wsUrl);
				
				ws.onopen = () => {
					console.log('BPI WebSocket connected');
					update(state => ({ 
						...state, 
						connected: true, 
						connecting: false, 
						error: null, 
						reconnectAttempts: 0 
					}));
					
					// Start heartbeat
					this.startHeartbeat();
				};
				
				ws.onmessage = (event) => {
					try {
						const message = JSON.parse(event.data);
						this.handleMessage(message);
						
						update(state => ({ 
							...state, 
							lastMessage: new Date() 
						}));
					} catch (error) {
						console.error('Failed to parse WebSocket message:', error);
					}
				};
				
				ws.onclose = (event) => {
					console.log('BPI WebSocket disconnected:', event.code, event.reason);
					update(state => ({ 
						...state, 
						connected: false, 
						connecting: false 
					}));
					
					this.stopHeartbeat();
					
					// Attempt reconnection if not intentionally closed
					if (event.code !== 1000) {
						this.scheduleReconnect();
					}
				};
				
				ws.onerror = (error) => {
					console.error('BPI WebSocket error:', error);
					update(state => ({ 
						...state, 
						error: 'WebSocket connection failed',
						connecting: false 
					}));
				};
				
			} catch (error) {
				update(state => ({ 
					...state, 
					error: `Failed to create WebSocket: ${error}`,
					connecting: false 
				}));
			}
		},
		
		// Disconnect WebSocket
		disconnect() {
			if (reconnectTimer) {
				clearTimeout(reconnectTimer);
				reconnectTimer = null;
			}
			
			this.stopHeartbeat();
			
			if (ws) {
				ws.close(1000, 'Client disconnect');
				ws = null;
			}
			
			set(initialState);
		},
		
		// Send message to server
		send(message: any) {
			if (ws?.readyState === WebSocket.OPEN) {
				ws.send(JSON.stringify(message));
				return true;
			}
			return false;
		},
		
		// Handle incoming messages
		handleMessage(message: any) {
			switch (message.type) {
				case 'system_status':
					systemStore.setStatus(message.data);
					break;
					
				case 'system_metrics':
					systemStore.setMetrics(message.data);
					break;
					
				case 'system_event':
					this.handleSystemEvent(message.data);
					break;
					
				case 'system_alert':
					this.handleSystemAlert(message.data);
					break;
					
				case 'pong':
					// Heartbeat response
					break;
					
				default:
					console.log('Unknown WebSocket message type:', message.type);
			}
		},
		
		// Handle system events
		handleSystemEvent(event: SystemEvent) {
			// Add to events store (to be implemented)
			console.log('System event:', event);
		},
		
		// Handle system alerts
		handleSystemAlert(alert: SystemAlert) {
			// Add to alerts store (to be implemented)
			console.log('System alert:', alert);
			
			// Show critical alerts immediately
			if (alert.severity === 'critical') {
				systemStore.setError(`Critical Alert: ${alert.title}`);
			}
		},
		
		// Start heartbeat to keep connection alive
		startHeartbeat() {
			this.stopHeartbeat();
			
			heartbeatTimer = setInterval(() => {
				if (ws?.readyState === WebSocket.OPEN) {
					this.send({ type: 'ping', timestamp: Date.now() });
				}
			}, heartbeatInterval);
		},
		
		// Stop heartbeat
		stopHeartbeat() {
			if (heartbeatTimer) {
				clearInterval(heartbeatTimer);
				heartbeatTimer = null;
			}
		},
		
		// Schedule reconnection attempt
		scheduleReconnect() {
			update(state => {
				if (state.reconnectAttempts >= maxReconnectAttempts) {
					return {
						...state,
						error: 'Maximum reconnection attempts reached'
					};
				}
				
				return {
					...state,
					reconnectAttempts: state.reconnectAttempts + 1
				};
			});
			
			reconnectTimer = setTimeout(() => {
				console.log('Attempting to reconnect WebSocket...');
				this.connect();
			}, reconnectDelay * Math.min(4, Math.pow(2, initialState.reconnectAttempts))); // Exponential backoff
		}
	};
}

export const websocketStore = createWebSocketStore();
