import { create } from 'zustand';
import type { AppSettings } from '../services/tauri';
import * as api from '../services/tauri';

interface SettingsStore {
  settings: AppSettings;
  loading: boolean;
  hydrated: boolean;
  error: string | null;
  fetchSettings: () => Promise<void>;
  updateSettings: (settings: AppSettings) => Promise<void>;
}

const defaultSettings: AppSettings = {
  default_polling_interval_sec: 10,
  data_retention_days: 7,
  theme: 'dark',
  language: 'zh-CN',
};

export const useSettingsStore = create<SettingsStore>((set) => ({
  settings: defaultSettings,
  loading: false,
  hydrated: false,
  error: null,

  fetchSettings: async () => {
    set({ loading: true, error: null });
    try {
      const settings = await api.getSettings();
      set({ settings, loading: false, hydrated: true, error: null });
    } catch (error) {
      set({
        loading: false,
        hydrated: true,
        error: error instanceof Error ? error.message : 'Failed to load settings',
      });
    }
  },

  updateSettings: async (settings) => {
    const updated = await api.updateSettings(settings);
    set({ settings: updated, error: null });
  },
}));
