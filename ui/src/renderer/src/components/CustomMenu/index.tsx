import { useCallback, useEffect, useState, useRef } from 'react';
import { Menu, Dropdown } from 'antd';
import { menuIndexeddbStorage } from '@renderer/store/menuIndexeddb';
import type { MyWebnoteFile, MyWebnoteFolder, OnFolderOrFileAddProps } from '@renderer/types/file';
import { storageAdapter } from '../../store/storage';
import { useAtom } from 'jotai';
import { currentFileAtom, currentFolderIdAtom } from '@renderer/store/jotai';
import EditableText from '../EditableText';
import { blocksuiteStorage } from '@renderer/store/blocksuite';
import OperationBar from '../OperationBar';
import moment from 'moment';
import MyWebnoteLogo from '../MyWebnoteLogo';

import './index.css';
import { getFileById, getFolderIdByFileId } from '@renderer/utils/file';
import { Folder, HardDrive, UploadCloud } from 'lucide-react';
import useAddFile from '@renderer/hooks/useAddFile';
import useFileContextMenu from '@renderer/hooks/useFileContextMenu';
import useFolderContextMenu from '@renderer/hooks/useFolderContextMenu';
import useFileTree from '@renderer/hooks/useFileTree';
import { useTranslation } from 'react-i18next';
import LanguageSwitcher from '../LanguageSwitcher/index';
import { boardIndexeddbStorage } from '@renderer/store/boardIndexeddb';
import PublicBetaNotice from '@renderer/components/PublicBetaNotice';

interface Props {
  collapsed: boolean;
}

export default function CustomMenu({ collapsed }: Props) {
  const { t } = useTranslation();

  const [currentStorageProvider, setCurrentStorageProvider] = useState(
    localStorage.getItem('mywebnote.storageType') || 'CLOUD'
  );
  const [openFileNames, setOpenFileNames] = useState<string[]>([]);
  const [selectedKeys, setSelectedKeys] = useState<string[]>([]);
  const [currentFile, setCurrentFile] = useAtom(currentFileAtom);
  const [currentFolderId, setCurrentFolderId] = useAtom(currentFolderIdAtom);
  const [editableTextState, setEditableTextState] = useState<{ [key: string]: boolean }>({});
  const firstRenderRef = useRef(false);
  const { fileTree, getFileTree } = useFileTree();

  const handleChangeStorageProvider = async (type: string) => {
    console.debug('Using storage provider :', type);
    await storageAdapter.setupStorageType(type);
    setCurrentStorageProvider(type);
    localStorage.setItem('mywebnote.storageType', type);
  };

  const resetMenu = useCallback(() => {
    setCurrentFile(undefined);
    setCurrentFolderId(undefined);
    setSelectedKeys([]);
  }, [setCurrentFile, setCurrentFolderId]);

  const addSelectedKeys = useCallback(
    (keys: string[] | undefined) => {
      if (!keys) return;

      let newKeys = selectedKeys;
      keys.forEach((key: string) => {
        const type = key?.startsWith('folder_') ? 'folder' : 'file';
        newKeys = type ? newKeys.filter((_key) => !_key?.startsWith(type)) : newKeys;
      });

      newKeys = Array.from(new Set([...newKeys, ...keys])).filter((_key) => !!_key);

      setSelectedKeys(newKeys);
    },
    [selectedKeys]
  );

  const onFolderOrFileAdd = useCallback(
    ({ fileKey, folderKey, type }: OnFolderOrFileAddProps) => {
      console.log(
        'onFolderOrFileAdd :: fileKey:',
        fileKey,
        ', folderKey:',
        folderKey,
        ', type:',
        type
      );

      setOpenFileNames((prev) => [...prev, folderKey]);
      updateEditableTextState(fileKey || folderKey, false);
      if (type === 'file') {
        addSelectedKeys(fileKey ? [fileKey] : []);
      } else if (type === 'folder') {
        resetMenu();
        setCurrentFile(undefined);
        setSelectedKeys([folderKey]);
      }
    },
    [openFileNames, addSelectedKeys, resetMenu, setCurrentFile]
  );

  const [addFile] = useAddFile({ onAdd: onFolderOrFileAdd });

  useEffect(() => {
    const fetchOpenFileNames = async () => {
      try {
        const folderNames = await storageAdapter
          .searchFolders(null)
          .then((folders) => folders.map((f: MyWebnoteFolder) => f.key));
        const fileNames = await storageAdapter.searchFileNames();
        setOpenFileNames(folderNames);
      } catch (error) {
        console.error('Error fetching open file names:', error);
      }
    };
    fetchOpenFileNames();
  }, []);

  useEffect(() => {
    if (!collapsed) {
      getFileTree();
    }
  }, [menuIndexeddbStorage, collapsed, getFileTree]);

  useEffect(() => {
    if (firstRenderRef.current === true || !fileTree?.length) return;
    firstRenderRef.current = true;

    const fetchCurrentFile = async () => {
      try {
        const currentFileId = await storageAdapter.loadCurrentFile();
        if (currentFileId) {
          setCurrentFile(getFileById(currentFileId, fileTree));
        }
      } catch (error) {
        console.error('Error loading current file:', error);
      }
    };
    fetchCurrentFile();
  }, [fileTree, setCurrentFile]);

  useEffect(() => {
    if (firstRenderRef.current === false) return;
    const saveCurrentFile = async () => {
      try {
        setSelectedKeys(currentFile?.key ? [currentFile.key] : []);
        // TODO:
        //await storageAdapter.saveCurrentFile(currentFile?.id);
      } catch (error) {
        console.error('Error saving current file:', error);
      }
    };
    saveCurrentFile();
  }, [currentFile?.key]);

  useEffect(() => {
    if (!currentFile) return;
    const folderKey = getFolderIdByFileId(currentFile.key, fileTree);
    setCurrentFolderId(folderKey);
  }, [currentFile, fileTree, setCurrentFolderId]);

  const deleteFile = useCallback(
    async (file: MyWebnoteFile) => {
      await menuIndexeddbStorage.deleteFile(file);
      console.log('onDeletedFile :: file:', file);

      switch (file.type) {
        case 'Board':
          await boardIndexeddbStorage.deleteBoard(file.key);
          break;
        case 'Note':
          await blocksuiteStorage.deletePage(file.key);
          break;
      }
      setCurrentFile(undefined);
      await getFileTree();
    },
    [menuIndexeddbStorage, setCurrentFile, getFileTree]
  );

  const updateEditableTextState = useCallback((id: string, value: boolean) => {
    setEditableTextState((prev) => ({ ...prev, [id]: value }));
  }, []);

  const deleteFolder = useCallback(
    async (folderKey: string) => {
      await menuIndexeddbStorage.deleteFolder(folderKey);
      await getFileTree();
      console.log('onDeletedFolder :: folderKey:', folderKey);
    },
    [menuIndexeddbStorage, getFileTree]
  );

  const [getFileContextMenu] = useFileContextMenu({
    editableTextState,
    deleteFile,
    updateEditableTextState
  });

  const [getFolderContextMenu] = useFolderContextMenu({
    fileTree,
    editableTextState,
    updateEditableTextState,
    addFile,
    deleteFolder
  });

  const onOpenFolderChanged = useCallback(
    async (keys: string[]) => {
      const folderKeys = keys.filter((key) => key.startsWith('folder_'));
      const openFolderKeys = openFileNames.filter((key) => key.startsWith('folder_'));
      const diffNum = folderKeys?.length - openFolderKeys.length;

      let changeType;
      switch (true) {
        case diffNum === 0:
          changeType = 'unchanged';
          break;
        case diffNum > 0:
          changeType = 'expand';
          break;
        default:
          changeType = 'collapse';
          break;
      }
      console.log(
        'onOpenFolderChanged :: changeType:',
        changeType,
        ', folderKeys:',
        folderKeys,
        ', openFolderKeys:',
        openFolderKeys
      );

      setOpenFileNames(keys);
      await storageAdapter.saveOpenFileNames(keys);

      if (changeType === 'expand') {
        const folderKey = keys?.length ? keys[keys.length - 1] : undefined;
        if (currentFolderId !== folderKey && folderKey) {
          resetMenu();
          setCurrentFolderId(folderKey);
          setSelectedKeys([folderKey]);
        }
      }
    },
    [openFileNames, currentFolderId, resetMenu, setCurrentFolderId]
  );

  const onSelectedFile = useCallback(
    ({ key }: { key: string }) => {
      const fileKey = key?.startsWith('file_') ? key : undefined;
      console.log('onSelectedFile :: fileKey:', fileKey, ', key:', key);

      if (!fileKey) return;
      const folderKey = getFolderIdByFileId(fileKey, fileTree);
      resetMenu();

      const file = getFileById(fileKey, fileTree);
      setCurrentFile(file);
      setCurrentFolderId(folderKey);
      addSelectedKeys([key, folderKey]);

      // TODO:
      //storageAdapter.saveCurrentFile(fileKey, file?.type || '', file?.name, folderKey);
    },
    [fileTree, resetMenu, setCurrentFile, setCurrentFolderId, addSelectedKeys]
  );

  const onFileNameChanged = useCallback(
    async (text: string, file: MyWebnoteFile) => {
      console.log('onFileNameChanged :: text:', text, ', file:', file);

      await menuIndexeddbStorage.updateFileName(file, text);
      updateEditableTextState(file.key, true);

      setSelectedKeys([file.key]);
      setCurrentFile({ ...file, name: text });
      await getFileTree();

      await storageAdapter.saveCurrentFile(
        file.key,
        file.type,
        text,
        'xxxxxx-see-onFolderOrFileAdd', // TODO:
        ''
      );
    },
    [setCurrentFile, getFileTree]
  );

  const onFolderNameChanged = useCallback(async (folder: MyWebnoteFolder, text: string) => {
    console.log('onFolderNameChanged :: text:', text, ', folder:', folder);

    await menuIndexeddbStorage.updateFolderName(folder, text);
    updateEditableTextState(folder.key, true);

    await storageAdapter.saveFolder(folder.key, text);
  }, []);

  const onEditableTextEdit = useCallback((id: string) => {
    updateEditableTextState(id, false);
  }, []);

  const storageProviderItems = [
    {
      key: 'CLOUD',
      icon: <UploadCloud className="w-4 mr-1"></UploadCloud>,
      label: t('storage.cloud'),
      disabled: false,
      onClick: () => handleChangeStorageProvider('CLOUD')
    },
    {
      key: 'LOCAL',
      icon: <HardDrive className="w-4 mr-1"></HardDrive>,
      label: t('storage.local'),
      disabled: false,
      onClick: () => handleChangeStorageProvider('LOCAL')
    }
  ];

  const storageProviderTextMap = {
    CLOUD: t('storage.cloud'),
    LOCAL: t('storage.local')
  };

  return (
    <div className="mywebnote-menu-container">
      <div className="flex flex-col mb-1 pl-5 pr-8 pt-0 justify-between">
        <div className="flex items-center">
          <MyWebnoteLogo size="small" onClick={() => resetMenu()} />
          <span>&nbsp;-&nbsp;{t('text.beta')}</span>
          <PublicBetaNotice />
        </div>
        <div className="flex justify-start">
          <div className="mr-2 whitespace-nowrap">
            <Dropdown menu={{ items: storageProviderItems }}>
              <span className="text-slate-500 flex items-center cursor-pointer">
                {storageProviderTextMap[currentStorageProvider]}
                <HardDrive className="w-4 mr-1"></HardDrive>
              </span>
            </Dropdown>
          </div>
          <LanguageSwitcher></LanguageSwitcher>
        </div>
      </div>
      <OperationBar size="small" folderKey={currentFolderId} onAdd={onFolderOrFileAdd} />
      <div className="menu-list border-t border-slate-100">
        <Menu
          theme="light"
          mode="inline"
          selectedKeys={selectedKeys}
          openKeys={openFileNames}
          onOpenChange={onOpenFolderChanged}
          onSelect={onSelectedFile}
          style={{ border: 'none' }}
          items={fileTree?.map((folder) => ({
            key: folder.key,
            icon: <Folder className="w-3" />,
            label: (
              <Dropdown menu={{ items: getFolderContextMenu(folder) }} trigger={['contextMenu']}>
                <div
                  className="flex items-center justify-between"
                  onClick={() => addSelectedKeys([folder.key])}
                >
                  <EditableText
                    isPreview={editableTextState[folder.key]}
                    text={folder.name}
                    defaultText="Untitled"
                    onSave={(text) => onFolderNameChanged(folder, text)}
                    onEdit={() => onEditableTextEdit(folder.key)}
                  />
                </div>
              </Dropdown>
            ),
            children: folder?.children?.map((file) => ({
              key: file.key,
              label: (
                <Dropdown
                  menu={{ items: getFileContextMenu(file, folder) }}
                  trigger={['contextMenu']}
                >
                  <div
                    className="flex items-center justify-between"
                    key={`${file.key}_${file.name}`}
                  >
                    <EditableText
                      isPreview={editableTextState[file.key]}
                      type={file.type}
                      text={file.name}
                      extraText={moment(file.gmtModified).format('YYYY-MM-DD HH:mm:ss')}
                      defaultText="Untitled"
                      onSave={(text) => onFileNameChanged(text, file)}
                      onEdit={() => onEditableTextEdit(file.key)}
                    />
                  </div>
                </Dropdown>
              )
            }))
          }))}
        />
      </div>
    </div>
  );
}
