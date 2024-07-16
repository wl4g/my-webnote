import { useCallback, useEffect, useState, useRef } from 'react';
import { Menu, Dropdown } from 'antd';
import { menuIndexeddbStorage } from '@renderer/store/menuIndexeddb';
import type { RevezoneFile, RevezoneFolder, OnFolderOrFileAddProps } from '@renderer/types/file';
import { storageAdapter } from '../../store/storage';
import { useAtom } from 'jotai';
import { currentFileAtom, currentFolderIdAtom } from '@renderer/store/jotai';
import EditableText from '../EditableText';
import { blocksuiteStorage } from '@renderer/store/blocksuite';
// import useBlocksuitePageTitle from '@renderer/hooks/useBlocksuitePageTitle';
import OperationBar from '../OperationBar';
import moment from 'moment';
import RevezoneLogo from '../RevezoneLogo';

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

  // --- Storage class switch. ---
  const handleChangeStorageProvider = (type: string) => {
    console.debug('Using storage provider :', type);
    storageAdapter.setupStorageType(type);
    setCurrentStorageProvider(type);
  };

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

  // Set up current storage provider state.
  const lastStorageProvider = localStorage.getItem('revezone.storageType');
  const [currentStorageProvider, setCurrentStorageProvider] = useState(
    lastStorageProvider || 'CLOUD'
  ); // Default

  const [openFileNames, setOpenFileNames] = useState<string[]>([]);
  useEffect(() => {
    const asyncSetOpenFileNames = async () => {
      const docs = await storageAdapter.searchFileNames();
      setOpenFileNames(docs);
    };
    asyncSetOpenFileNames();
  }, []);

  const [selectedKeys, setSelectedKeys] = useState<string[]>([]);
  const [currentFile, setCurrentFile] = useAtom(currentFileAtom);
  const [currentFolderId, setCurrentFolderId] = useAtom(currentFolderIdAtom);
  const [editableTextState, setEditableTextState] = useState<{ [key: string]: boolean }>({});
  const firstRenderRef = useRef(false);
  const { fileTree, getFileTree } = useFileTree();

  const onFolderOrFileAdd = useCallback(
    ({ fileId, folderId, type }: OnFolderOrFileAddProps) => {
      console.log('onFolderOrFileAdd :: fileId:', fileId, ', folderId:', folderId, ', type:', type);

      setOpenFileNames([...openFileNames, folderId]);
      updateEditableTextState(fileId || folderId, false, editableTextState);
      if (type === 'file') {
        addSelectedKeys(fileId ? [fileId] : []);
      } else if (type === 'folder') {
        resetMenu();
        setCurrentFile(undefined);
        setSelectedKeys([folderId]);
      }
    },
    [openFileNames, editableTextState]
  );

  const [addFile] = useAddFile({ onAdd: onFolderOrFileAdd });
  // const [pageTitle] = useBlocksuitePageTitle({ getFileTree });

  useEffect(() => {
    !collapsed && getFileTree();
  }, [menuIndexeddbStorage, collapsed]);

  useEffect(() => {
    if (firstRenderRef.current === true || !fileTree?.length) return;
    firstRenderRef.current = true;

    // const file = currentFileId ? getFileById(currentFileId, fileTree) : undefined;
    // setCurrentFile(file);
    const currentFileId = storageAdapter.loadCurrentFile();
    currentFileId.then((res) => {
      if (!res) return;
      setCurrentFile(getFileById(res, fileTree));
    });
  }, [fileTree]);

  useEffect(() => {
    if (firstRenderRef.current === false) return;
    storageAdapter.saveCurrentFile(currentFile?.id);
    setSelectedKeys(currentFile?.id ? [currentFile.id] : []);
  }, [currentFile?.id]);

  useEffect(() => {
    if (!currentFile) {
      return;
    }
    const folderId = getFolderIdByFileId(currentFile.id, fileTree);
    setCurrentFolderId(folderId);
  }, [currentFile, fileTree]);

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

  const deleteFile = useCallback(
    async (file: RevezoneFile) => {
      await menuIndexeddbStorage.deleteFile(file);
      console.log('onDeletedFile :: file:', file);

      switch (file.type) {
        case 'board':
          await boardIndexeddbStorage.deleteBoard(file.id);
          break;
        case 'note':
          await blocksuiteStorage.deletePage(file.id);
          break;
      }
      setCurrentFile(undefined);
      await getFileTree();
    },
    [menuIndexeddbStorage, currentFile]
  );

  const updateEditableTextState = useCallback((id: string, value: boolean, editableTextState) => {
    const newEditableTextState = { ...editableTextState };
    newEditableTextState[id] = value;
    setEditableTextState(newEditableTextState);
  }, []);

  const deleteFolder = useCallback(
    async (folderId: string) => {
      await menuIndexeddbStorage.deleteFolder(folderId);
      await getFileTree();
      console.log('onDeletedFolder :: folderId:', folderId);
    },
    [menuIndexeddbStorage]
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

  const resetMenu = useCallback(() => {
    setCurrentFile(undefined);
    setCurrentFolderId(undefined);
    setSelectedKeys([]);
  }, []);

  const onOpenFolderChanged = useCallback(
    (keys) => {
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
      storageAdapter.saveOpenFileNames(keys);

      // only while openFileNames increase
      if (changeType === 'expand') {
        const folderId = keys?.length ? keys[keys.length - 1] : undefined;
        if (currentFolderId !== folderId) {
          resetMenu();
          setCurrentFolderId(folderId);
          setSelectedKeys([folderId]);
        }
      }
    },
    [openFileNames, currentFolderId]
  );

  const onSelectedFile = useCallback(
    ({ key }) => {
      const fileId = key?.startsWith('file_') ? key : undefined;
      console.log('onSelectedFile :: fileId:', fileId, ', key:', key);

      if (!fileId) return;
      const folderId = getFolderIdByFileId(fileId, fileTree);
      resetMenu();

      const file = getFileById(fileId, fileTree);
      setCurrentFile(file);
      setCurrentFolderId(folderId);
      addSelectedKeys([key, folderId]);
    },
    [fileTree]
  );

  const onFileNameChanged = useCallback(
    async (text: string, file: RevezoneFile) => {
      console.log('onFileNameChanged :: text:', text, ', file:', file);

      await menuIndexeddbStorage.updateFileName(file, text);
      updateEditableTextState(file.id, true, editableTextState);

      setSelectedKeys([file.id]);

      setCurrentFile({ ...file, name: text });

      await getFileTree();
    },
    [editableTextState]
  );

  const onFolderNameChanged = useCallback(
    (folder: RevezoneFolder, text: string) => {
      console.log('onFolderNameChanged :: text:', text, ', folder:', folder);

      menuIndexeddbStorage.updateFolderName(folder, text);
      updateEditableTextState(folder.id, true, editableTextState);
    },
    [editableTextState]
  );

  const onEditableTextEdit = useCallback(
    (id: string) => {
      updateEditableTextState(id, false, editableTextState);
    },
    [editableTextState]
  );

  return (
    <div className="revezone-menu-container">
      <div className="flex flex-col mb-1 pl-5 pr-8 pt-0 justify-between">
        <div className="flex items-center">
          <RevezoneLogo size="small" onClick={() => resetMenu()} />
          <span>&nbsp;-&nbsp;{t('text.alpha')}</span>
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
      <OperationBar size="small" folderId={currentFolderId} onAdd={onFolderOrFileAdd} />
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
            key: folder.id,
            icon: <Folder className="w-3" />,
            label: (
              <Dropdown menu={{ items: getFolderContextMenu(folder) }} trigger={['contextMenu']}>
                <div
                  className="flex items-center justify-between"
                  onClick={() => addSelectedKeys([folder.id])}
                >
                  <EditableText
                    isPreview={editableTextState[folder.id]}
                    text={folder.name}
                    defaultText="Untitled"
                    onSave={(text) => onFolderNameChanged(folder, text)}
                    onEdit={() => onEditableTextEdit(folder.id)}
                  />
                </div>
              </Dropdown>
            ),
            children: folder?.children?.map((file) => {
              return {
                key: file.id,
                label: (
                  <Dropdown
                    menu={{ items: getFileContextMenu(file, folder) }}
                    trigger={['contextMenu']}
                  >
                    <div
                      className="flex items-center justify-between"
                      key={`${file.id}_${file.name}`}
                    >
                      <EditableText
                        isPreview={editableTextState[file.id]}
                        type={file.type}
                        text={file.name}
                        extraText={moment(file.gmtModified).format('YYYY-MM-DD HH:mm:ss')}
                        defaultText="Untitled"
                        onSave={(text) => onFileNameChanged(text, file)}
                        onEdit={() => onEditableTextEdit(file.id)}
                      />
                    </div>
                  </Dropdown>
                )
              };
            })
          }))}
        />
      </div>
    </div>
  );
}
