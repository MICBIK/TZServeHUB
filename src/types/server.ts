export interface ServerConfig {
  id: string;
  name: string;
  host: string;
  port: number;
  adapter_type: 'node_exporter' | 'glances' | 'go_agent';
  access_method: 'private' | 'tunnel' | 'gateway';
  polling_interval_sec: number;
  enabled: boolean;
  created_at: number;
  updated_at: number;
}

export interface ServerFormData {
  name: string;
  host: string;
  port: number;
  adapter_type: ServerConfig['adapter_type'];
  access_method: ServerConfig['access_method'];
  polling_interval_sec: number;
}
