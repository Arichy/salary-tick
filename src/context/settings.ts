import { createContext, useContext } from 'react';
import { Settings } from '../types';

const SettingsContext = createContext<Settings>({
  dailySalary: 0,
  startTime: '09:00',
  endTime: '17:00',
  currencySymbol: '$',
});

export const SettingsProvider = SettingsContext.Provider;

const useSettings = () => {
  return useContext(SettingsContext)!;
};

export default useSettings;
