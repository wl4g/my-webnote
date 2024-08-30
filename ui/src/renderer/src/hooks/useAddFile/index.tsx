import { useCallback } from 'react';
import { FileTree, MyWebnoteFileType, OnFolderOrFileAddProps } from '@renderer/types/file';
import { fileTreeAtom, currentFileAtom } from '@renderer/store/jotai';
import { useAtom } from 'jotai';
import { menuIndexeddbStorage } from '@renderer/store/menuIndexeddb';
import { useTranslation } from 'react-i18next';
import { boardIndexeddbStorage } from '@renderer/store/boardIndexeddb';
import { blocksuiteStorage } from '@renderer/store/blocksuite';

interface Props {
  onAdd?: ({ fileKey, folderKey, type }: OnFolderOrFileAddProps) => void;
}

const DEFAULT_BORAD_DATA = '{}';

export default function useAddFile({ onAdd }: Props) {
  const [, setCurrentFile] = useAtom(currentFileAtom);
  const [, setFileTree] = useAtom(fileTreeAtom);
  const { t } = useTranslation();

  const addFile = useCallback(
    async (folderKey: string | undefined, type: MyWebnoteFileType, fileTree: FileTree) => {
      let _folderKey = folderKey || fileTree?.[0]?.key;

      if (!_folderKey) {
        _folderKey = (await menuIndexeddbStorage.addFolder(t('text.defaultFolder')))?.key;
      }

      const file = await menuIndexeddbStorage.addFile(_folderKey, type);

      if (type === 'Board') {
        await boardIndexeddbStorage.addBoard(file.key, DEFAULT_BORAD_DATA);
      } else if (type === 'Note') {
        await blocksuiteStorage.addPage(file.key);
      }

      const tree = await menuIndexeddbStorage.getFileTree();
      setFileTree(tree);

      setCurrentFile(file);

      onAdd?.({ fileKey: file.key, folderKey: _folderKey, type: 'file' });
    },
    []
  );

  return [addFile];
}
