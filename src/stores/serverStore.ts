import { create } from 'zustand';
import type { ServerConfig, ServerFormData } from '../types/server';
import * as api from '../services/tauri';

interface ServerStore {
  servers: ServerConfig[];
  activeServerId: string | null;
  loading: boolean;
  hydrated: boolean;
  error: string | null;
  fetchServers: () => Promise<void>;
  refreshServerStatus: () => Promise<void>;
  addServer: (input: ServerFormData) => Promise<void>;
  removeServer: (id: string) => Promise<void>;
  setActiveServer: (id: string | null) => void;
}

export const useServerStore = create<ServerStore>((set) => ({
  servers: [],
  activeServerId: null,
  loading: false,
  hydrated: false,
  error: null,

  fetchServers: async () => {
    set({ loading: true, error: null });
    try {
      const servers = await api.listServers();
      set((state) => {
        const activeServerId =
          state.activeServerId && servers.some((server) => server.id === state.activeServerId)
            ? state.activeServerId
            : servers[0]?.id ?? null;

        return {
          servers,
          activeServerId,
          loading: false,
          hydrated: true,
          error: null,
        };
      });
    } catch (error) {
      set({
        loading: false,
        hydrated: true,
        error: error instanceof Error ? error.message : 'Failed to load servers',
      });
    }
  },

  refreshServerStatus: async () => {
    try {
      const servers = await api.listServers();
      set((state) => {
        const activeServerId =
          state.activeServerId && servers.some((server) => server.id === state.activeServerId)
            ? state.activeServerId
            : servers[0]?.id ?? null;

        return { servers, activeServerId };
      });
    } catch {
      // Silent refresh — don't overwrite existing error state
    }
  },

  addServer: async (input) => {
    const server = await api.addServer(input);
    set((state) => ({
      servers: [...state.servers, server],
      activeServerId: state.activeServerId ?? server.id,
      error: null,
    }));
  },

  removeServer: async (id) => {
    await api.removeServer(id);
    set((state) => {
      const servers = state.servers.filter((server) => server.id !== id);
      const activeServerId =
        state.activeServerId === id ? servers[0]?.id ?? null : state.activeServerId;

      return {
        servers,
        activeServerId,
        error: null,
      };
    });
  },

  setActiveServer: (id) => set({ activeServerId: id }),
}));
