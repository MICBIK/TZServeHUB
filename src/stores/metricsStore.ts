import { create } from 'zustand';
import type { MetricPoint } from '../types/metric';
import * as api from '../services/tauri';

interface MetricsStore {
  current: Record<string, MetricPoint[]>; // keyed by server_id
  loading: boolean;
  fetchMetrics: (serverId: string) => Promise<void>;
  fetchHistory: (serverId: string, key: string, from: number, to: number) => Promise<MetricPoint[]>;
}

export const useMetricsStore = create<MetricsStore>((set) => ({
  current: {},
  loading: false,

  fetchMetrics: async (serverId) => {
    set({ loading: true });
    try {
      const metrics = await api.getMetrics(serverId);
      set((s) => ({
        current: { ...s.current, [serverId]: metrics },
        loading: false,
      }));
    } catch {
      set({ loading: false });
    }
  },

  fetchHistory: async (serverId, key, from, to) => {
    return api.getMetricHistory(serverId, key, from, to);
  },
}));
