import { useCallback } from 'react';
import { FileEdit, Trash2, ClipboardCopy } from 'lucide-react';
import { EditableTextState } from '@renderer/types/menu';
import { useTranslation } from 'react-i18next';
import { MyWebnoteFile, MyWebnoteFolder } from '@renderer/types/file';

interface Props {
  editableTextState: EditableTextState;
  updateEditableTextState: (
    id: string,
    value: boolean,
    editableTextState: EditableTextState
  ) => void;
  deleteFile: (file: MyWebnoteFile, folderId: string) => void;
}

export default function useFileContextMenu(props: Props) {
  const { editableTextState, updateEditableTextState, deleteFile } = props;
  const { t } = useTranslation();

  const getFileContextMenu = useCallback(
    (file: MyWebnoteFile, folder: MyWebnoteFolder) => [
      {
        key: 'rename',
        label: t('operation.rename'),
        icon: <FileEdit className="w-4" />,
        onClick: ({ domEvent }) => {
          domEvent.stopPropagation();
          console.debug('onRenameFileBefore :: file.id:', file.id, ', folder.id:', folder.id);
          updateEditableTextState(file.id, false, editableTextState);
        }
      },
      {
        key: 'delete',
        label: t('operation.delete'),
        icon: <Trash2 className="w-4"></Trash2>,
        onClick: () => {
          console.debug('onDeletedFileBefore :: file.id:', file.id, ', folder.id:', folder.id);
          deleteFile(file, folder.id);
        }
      },
      {
        key: 'copy_mywebnote_link',
        label: t('operation.copyMyWebnoteLink'),
        icon: <ClipboardCopy className="w-4" />,
        onClick: ({ domEvent }) => {
          domEvent.stopPropagation();
          // for test: When use electron client.
          //navigator.clipboard.writeText(`mywebnote://${file.id}`);
          // TODO: copy link
        }
      }
    ],
    []
  );

  return [getFileContextMenu];
}
