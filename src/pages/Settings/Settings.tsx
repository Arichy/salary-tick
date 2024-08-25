import { Button, Center, NumberInput, Select, Stack } from '@mantine/core';
import { TimeInput } from '@mantine/dates';
import { useForm } from '@mantine/form';
import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useLayoutEffect } from 'react';
import useSettings from '../../context/settings';

type NativeSettingsJSON = {
  daily_salary: number;
  start_time: string;
  end_time: string;
  currency_symbol: string;
};

type Settings = {
  dailySalary: number;
  startTime: string;
  endTime: string;
  currencySymbol: string;
};

const currencySymbols = {
  dollar: '$', // USD, CAD, AUD, NZD, HKD, SGD, etc.
  yuan: '¥', // Chinese Yuan apanese Yen
  euro: '€', // Euro
  pound: '£', // British Pound
  rupee: '₹', // Indian Rupee
  won: '₩', // South Korean Won
  ruble: '₽', // Russian Ruble
  lira: '₺', // Turkish Lira
  real: 'R$', // Brazilian Real
  dong: '₫', // Vietnamese Dong
  baht: '฿', // Thai Baht
};
export default function Settings() {
  const settings = useSettings();
  useEffect(() => {
    form.setValues(settings);
  }, [settings]);

  const form = useForm<Settings>({
    mode: 'controlled',
    initialValues: settings,
    validate: {
      dailySalary: value => (value < 0 ? 'Please don’t pay for work!' : null),
    },
  });

  return (
    <div>
      <form
        style={{ width: 200 }}
        onSubmit={form.onSubmit(values => {
          console.log(values);
          invoke('save_settings', values)
            .then(res => {
              console.log(res);
            })
            .catch(err => {
              console.log(err);
            });
        })}
      >
        <Stack>
          <NumberInput label="Daily Salary" {...form.getInputProps('dailySalary')} />
          <TimeInput label="Start Time" {...form.getInputProps('startTime')} />
          <TimeInput label="End Time" {...form.getInputProps('endTime')} />
          <Select
            label="Currency Symbol"
            data={Object.values(currencySymbols)}
            {...form.getInputProps('currencySymbol')}
          />
        </Stack>

        <Button variant="outline" mt="md" type="submit">
          Save
        </Button>
      </form>
    </div>
  );
}
