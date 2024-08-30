import { FileTree, MyWebnoteFile } from '@renderer/types/file';

const REVEZONE_LINK_PROTOCOL = 'mywebnote://';
import { DOUBLE_LINK_REGEX } from '@renderer/utils/constant';

export const getFileById = (fileKey: string, fileTree: FileTree) => {
  const files = fileTree.reduce(
    (prev: MyWebnoteFile[], current) => [...prev, ...current.children],
    []
  );

  const file = fileKey ? files?.find((_file) => _file.key === fileKey) : null;

  return file;
};

export const getFolderIdByFileId = (fileKey: string, fileTree: FileTree): string => {
  let currentFolderId;
  for (const folder of fileTree) {
    const file = folder.children.find((_file) => _file.key === fileKey);
    if (file) {
      currentFolderId = folder.key;
      break;
    }
  }

  return currentFolderId;
};

export const getFileIdOrNameFromLink = (link: string) => {
  if (link.startsWith(REVEZONE_LINK_PROTOCOL)) {
    // file id
    return link.split(REVEZONE_LINK_PROTOCOL)?.[1];
  } else if (DOUBLE_LINK_REGEX.test(link)) {
    // file name
    return link?.match(DOUBLE_LINK_REGEX)?.[1];
  }
  return null;
};
