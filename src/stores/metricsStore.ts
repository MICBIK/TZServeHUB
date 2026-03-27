import { create } from 'zustand';
import type { MetricHistoryResponse, MetricPoint } from '../types/metric';
import * as api from '../services/tauri';
import type { MetricHistoryOptions } from '../services/tauri';

interface MetricsStore {
  current: Record<string, MetricPoint[]>; // keyed by server_id
  loading: boolean;
  error: string | null;
  fetchMetrics: (serverId: string) => Promise<void>;
  fetchHistory: (
    serverId: string,
    key: string,
    from: number,
    to: number,
    options?: MetricHistoryOptions,
  ) => Promise<MetricHistoryResponse>;
}

export const useMetricsStore = create<MetricsStore>((set) => ({
  current: {},
  loading: false,
  error: null,

  fetchMetrics: async (serverId) => {
    set({ loading: true, error: null });
    try {
      const metrics = await api.getMetrics(serverId);
      set((s) => ({
        current: { ...s.current, [serverId]: metrics },
        loading: false,
        error: null,
      }));
    } catch (error) {
      set({
        loading: false,
        error: error instanceof Error ? error.message : 'Failed to load metrics',
      });
    }
  },

  fetchHistory: async (serverId, key, from, to, options) => {
    return api.getMetricHistory(serverId, key, from, to, options);
  },
}));
