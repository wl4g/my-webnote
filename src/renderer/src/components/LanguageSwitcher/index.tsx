import { langCodeList } from '@renderer/i18n';
import { langCodeAtom } from '@renderer/store/jotai';
import { DEFAULT_LANG_CODE } from '@renderer/utils/constant';
import { Select } from 'antd';
import { useAtom } from 'jotai';
import { useCallback, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { storageAdapter } from '../../store/storage';

export default function LanguageSwitcher() {
  const [langCode, setLangCode] = useAtom(langCodeAtom);
  const { i18n } = useTranslation();

  useEffect(() => {
    const code = storageAdapter.loadLangCode() || DEFAULT_LANG_CODE;
    i18n.changeLanguage(code);
    setLangCode(code);
  }, []);

  const onChange = useCallback((code: string) => {
    setLangCode(code);
    i18n.changeLanguage(code);
    storageAdapter.saveLangCode(code);
  }, []);

  return (
    <Select
      size="small"
      value={langCode}
      onClick={(e) => e.stopPropagation()}
      onChange={onChange}
      className="revezone-language-select items-center w-auto whitespace-nowrap"
      bordered={false}
    >
      {langCodeList.map((item) => {
        return (
          <Select.Option key={item.key} value={item.key}>
            {item.label}
          </Select.Option>
        );
      })}
    </Select>
  );
}
