import { useEffect, useCallback, useState } from 'react';
import { Button, Form, Input, Modal, message } from 'antd';
import { storageAdapter } from '@renderer/store/storage';
import { useTranslation } from 'react-i18next';

import './index.css';
import { useAtom } from 'jotai';
import { currentFileAtom } from '@renderer/store/jotai';

interface Props {
  open: boolean;
  closeModal: () => void;
}

const CustomFontModal = (props: Props) => {
  const { open, closeModal } = props;

  const { t } = useTranslation();

  const [fontName, setFontName] = useState();
  const [fontPath, setFontPath] = useState();
  const [fontFamilyName, setFontFamilyName] = useState<string>();
  const [currentFile, setCurrentFile] = useAtom(currentFileAtom);

  const loadCustomFonts = useCallback(() => {
    window.api && window.api.loadCustomFonts();
  }, []);

  useEffect(() => {
    window.api &&
      window.api.onLoadCustomFontSuccess(async (event, _fontName, _fontPath) => {
        setFontName(_fontName);
        setFontPath(_fontPath);
        setFontFamilyName(_fontName);
      });
  }, []);

  const onOk = useCallback(() => {
    if (!fontPath) {
      message.error('Please load font file');
      return;
    }

    if (!fontFamilyName) {
      message.error('Font name cannot be empty!');
      return;
    }

    window.api.registerCustomFont(fontFamilyName, fontPath);

    storageAdapter.addBoardCustomFont(fontFamilyName);

    const prevFile = currentFile;

    setCurrentFile(undefined);
    setTimeout(() => {
      setCurrentFile(prevFile);
    }, 0);

    closeModal();
  }, [fontPath, fontFamilyName, currentFile]);

  return (
    <Modal
      className="mywebnote-custom-font-modal"
      title={t('customFontModal.title')}
      open={open}
      onOk={onOk}
      onCancel={() => closeModal()}
    >
      <Form labelCol={{ span: 6 }} className="mt-6">
        <Form.Item label={t('customFontModal.fontFile')}>
          <p>
            <span className="mr-2">{fontName}</span>
            <Button onClick={loadCustomFonts}>{t('customFontModal.load')}</Button>
          </p>
        </Form.Item>
        <Form.Item label={t('customFontModal.fontName')}>
          <Input value={fontFamilyName} onChange={(e) => setFontFamilyName(e.target.value)} />
        </Form.Item>
      </Form>
    </Modal>
  );
};

export default CustomFontModal;
