import { useCallback } from 'react';
import { FolderEdit, Trash2, FileType, Palette } from 'lucide-react';
import { FileTree, MyWebnoteFolder, MyWebnoteFileType } from '@renderer/types/file';
import { EditableTextState } from '@renderer/types/menu';
import { useTranslation } from 'react-i18next';

interface Props {
  fileTree: FileTree;
  editableTextState: EditableTextState;
  addFile: (folderKey: string | undefined, type: MyWebnoteFileType, fileTree: FileTree) => void;
  updateEditableTextState: (
    key: string,
    value: boolean,
    editableTextState: EditableTextState
  ) => void;
  deleteFolder: (folderKey: string) => void;
}

export default function useFolderContextMenu(props: Props) {
  const { editableTextState, fileTree, addFile, updateEditableTextState, deleteFolder } = props;
  const { t } = useTranslation();

  const getFolderContextMenu = useCallback(
    (folder: MyWebnoteFolder) => [
      {
        key: 'addnote',
        label: t('operation.addNote'),
        icon: <FileType className="w-4" />,
        onClick: ({ domEvent }) => {
          domEvent.stopPropagation();
          addFile(folder.key, 'Note', fileTree);
        }
      },
      {
        key: 'addboard',
        label: t('operation.addBoard'),
        icon: <Palette className="w-4" />,
        onClick: ({ domEvent }) => {
          domEvent.stopPropagation();
          addFile(folder.key, 'Board', fileTree);
        }
      },
      {
        key: 'rename',
        label: t('operation.rename'),
        icon: <FolderEdit className="w-4" />,
        onClick: ({ domEvent }) => {
          domEvent.stopPropagation();
          console.log('rename');
          updateEditableTextState(folder.key, false, editableTextState);
        }
      },
      {
        key: 'delete',
        label: t('operation.delete'),
        icon: <Trash2 className="w-4"></Trash2>,
        onClick: ({ domEvent }) => {
          domEvent.stopPropagation();
          deleteFolder(folder.key);
        }
      }
    ],
    [editableTextState]
  );

  return [getFolderContextMenu];
}
