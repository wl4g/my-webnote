export interface MyWebnoteFolder {
  key: string;
  name: string;
  gmtCreate: string;
  gmtModified: string;
}

export interface MyWebnoteFolderFileMapping {
  folderKey: string;
  fileKey: string;
  gmtCreate: string;
  gmtModified: string;
}

export type MyWebnoteFileType = 'Note' | 'Board';

export interface MyWebnoteFile {
  key: string;
  name: string;
  type: MyWebnoteFileType;
  gmtCreate: string;
  gmtModified: string;
}

export interface MyWebnoteFolder {
  key: string;
  name: string;
  gmtCreate: string;
  gmtModified: string;
}

export type FileTreeItem = MyWebnoteFolder & { children: MyWebnoteFile[] };

export type FileTree = FileTreeItem[];

export interface OnFolderOrFileAddProps {
  fileKey?: string;
  folderKey: string;
  type: 'folder' | 'file';
}

export interface Font {
  name: string;
  nameWithSuffix: string;
  path: string;
}
