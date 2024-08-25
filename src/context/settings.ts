import { createContext, useContext } from 'react';
import { Settings } from '../types';

const SettingsContext = createContext<Settings | null>(null);

export const SettingsProvider = SettingsContext.Provider;

const useSettings = () => {
  return useContext(SettingsContext)!;
};

export default useSettings;
