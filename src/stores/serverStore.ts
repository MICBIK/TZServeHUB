import { create } from 'zustand';
import type { ServerConfig } from '../types/server';
import * as api from '../services/tauri';

interface ServerStore {
  servers: ServerConfig[];
  activeServerId: string | null;
  loading: boolean;
  fetchServers: () => Promise<void>;
  addServer: (name: string, host: string, port: number) => Promise<void>;
  removeServer: (id: string) => Promise<void>;
  setActiveServer: (id: string | null) => void;
}

export const useServerStore = create<ServerStore>((set, get) => ({
  servers: [],
  activeServerId: null,
  loading: false,

  fetchServers: async () => {
    set({ loading: true });
    try {
      const servers = await api.listServers();
      set({ servers, loading: false });
    } catch {
      set({ loading: false });
    }
  },

  addServer: async (name, host, port) => {
    const server = await api.addServer(name, host, port);
    set((s) => ({ servers: [...s.servers, server] }));
  },

  removeServer: async (id) => {
    await api.removeServer(id);
    set((s) => ({
      servers: s.servers.filter((srv) => srv.id !== id),
      activeServerId: s.activeServerId === id ? null : s.activeServerId,
    }));
  },

  setActiveServer: (id) => set({ activeServerId: id }),
}));
