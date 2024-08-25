export type NativeSettingsJSON = {
  daily_salary: number;
  start_time: string;
  end_time: string;
  currency_symbol: string;
  language: 'en' | 'zh';
};

export type Settings = {
  dailySalary: number;
  startTime: string;
  endTime: string;
  currencySymbol: string;
  language: string;
};
