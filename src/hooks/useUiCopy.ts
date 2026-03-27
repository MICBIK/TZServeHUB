import { useSettingsStore } from '../stores/settingsStore';
import { dictionaries, type UiCopyKey } from '../lib/uiCopy';
import type { LanguageMode } from '../services/tauri';

export function useUiCopy() {
  const language = useSettingsStore((state) => state.settings.language) as LanguageMode;
  const dictionary = dictionaries[language] ?? dictionaries['zh-CN'];

  return {
    language,
    t(key: UiCopyKey, variables?: Record<string, string | number>) {
      const template = dictionary[key];
      if (!variables) {
        return template;
      }

      return Object.entries(variables).reduce(
        (value, [name, replacement]) =>
          value.replaceAll(`{${name}}`, String(replacement)),
        template,
      );
    },
  };
}
