export interface MyWebnoteFolder {
  id: string;
  name: string;
  gmtCreate: string;
  gmtModified: string;
}

export interface MyWebnoteFolderFileMapping {
  folderId: string;
  fileId: string;
  gmtCreate: string;
  gmtModified: string;
}

export type MyWebnoteFileType = 'note' | 'board';

export interface MyWebnoteFile {
  id: string;
  name: string;
  type: MyWebnoteFileType;
  gmtCreate: string;
  gmtModified: string;
}

export interface MyWebnoteFolder {
  id: string;
  name: string;
  gmtCreate: string;
  gmtModified: string;
}

export type FileTreeItem = MyWebnoteFolder & { children: MyWebnoteFile[] };

export type FileTree = FileTreeItem[];

export interface OnFolderOrFileAddProps {
  fileId?: string;
  folderId: string;
  type: 'folder' | 'file';
}

export interface Font {
  name: string;
  nameWithSuffix: string;
  path: string;
}
