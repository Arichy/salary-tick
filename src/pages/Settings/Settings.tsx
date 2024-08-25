import { Button, NumberInput, Select, Stack } from '@mantine/core';
import { TimeInput } from '@mantine/dates';
import { useForm } from '@mantine/form';
import { invoke } from '@tauri-apps/api/tauri';
import useSettings from '../../context/settings';
import { Settings as ISettings } from '../../types';
import { useTranslation } from 'react-i18next';
import { I18N_KEYS } from '../../i18n';

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

  const { t } = useTranslation();

  const form = useForm<ISettings>({
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

          queueMicrotask(() => {
            invoke('save_settings', values)
              .then(res => {
                console.log(res);
              })
              .catch(err => {
                console.log(err);
              });
          });
        })}
      >
        <Stack>
          <NumberInput label={t(I18N_KEYS.dailySalary)} {...form.getInputProps('dailySalary')} />
          <TimeInput label={t(I18N_KEYS.startTime)} {...form.getInputProps('startTime')} />
          <TimeInput label={t(I18N_KEYS.endTime)} {...form.getInputProps('endTime')} />
          <Select
            label={t(I18N_KEYS.currencySymbol)}
            data={Object.values(currencySymbols)}
            allowDeselect={false}
            {...form.getInputProps('currencySymbol')}
          />

          <Select
            label={t(I18N_KEYS.language)}
            data={[
              { label: 'English', value: 'en' },
              { label: '简体中文', value: 'zh' },
            ]}
            allowDeselect={false}
            {...form.getInputProps('language')}
          />
        </Stack>

        <Button variant="outline" mt="md" type="submit">
          Save
        </Button>
      </form>
    </div>
  );
}
