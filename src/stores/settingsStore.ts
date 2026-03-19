import { create } from 'zustand';
import type { AppSettings } from '../services/tauri';
import * as api from '../services/tauri';

interface SettingsStore {
  settings: AppSettings;
  loading: boolean;
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

  fetchSettings: async () => {
    set({ loading: true });
    try {
      const settings = await api.getSettings();
      set({ settings, loading: false });
    } catch {
      set({ loading: false });
    }
  },

  updateSettings: async (settings) => {
    const updated = await api.updateSettings(settings);
    set({ settings: updated });
  },
}));
