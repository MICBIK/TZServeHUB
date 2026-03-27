import type { ServerConfig } from '../types/server';
import type { UiCopyKey } from './uiCopy';

const adapterLabelKeys: Record<ServerConfig['adapter_type'], UiCopyKey> = {
  go_agent: 'adapter_go_agent',
  node_exporter: 'adapter_node_exporter',
  glances: 'adapter_glances',
};

const accessMethodLabelKeys: Record<ServerConfig['access_method'], UiCopyKey> = {
  private: 'access_method_private',
  tunnel: 'access_method_tunnel',
  gateway: 'access_method_gateway',
};

export function getAdapterLabelKey(adapter: ServerConfig['adapter_type']): UiCopyKey {
  return adapterLabelKeys[adapter];
}

export function getAccessMethodLabelKey(method: ServerConfig['access_method']): UiCopyKey {
  return accessMethodLabelKeys[method];
}
