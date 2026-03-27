export interface ServerConfig {
  id: string;
  name: string;
  host: string;
  port: number;
  adapter_type: 'node_exporter' | 'glances' | 'go_agent';
  access_method: 'private' | 'tunnel' | 'gateway';
  polling_interval_sec: number;
  enabled: boolean;
  auth_token?: string | null;
  auth_type?: 'token' | 'ssh_key' | 'password' | 'none';
  ssh_key_path?: string | null;
  ssh_passphrase?: string | null;
  password?: string | null;
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
  auth_token?: string;
  auth_type?: 'token' | 'ssh_key' | 'password';
  ssh_key?: string;
  ssh_passphrase?: string;
  password?: string;
}
