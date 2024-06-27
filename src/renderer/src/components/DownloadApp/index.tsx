import { DownloadCloud } from 'lucide-react';
import { isInRevezoneApp } from '@renderer/utils/navigator';
import { useTranslation } from 'react-i18next';

interface Props {
  className?: string;
  from: 'bottombar' | 'welcomepage' | 'systemsetting';
}

export default function DownloadApp({ className = '', from }: Props) {
  const { t } = useTranslation();

  return (
    <a
      className={`mr-2 flex items-center ${className}`}
      href="https://github.com/wl4g-collect/revezone/releases"
      target="_blank"
      rel="noreferrer"
      title={t('welcome.downloadApp')}
    >
      <DownloadCloud
        className={`w-4 h-4 ${!isInRevezoneApp ? 'animate-bounce' : null}`}
      ></DownloadCloud>
    </a>
  );
}
