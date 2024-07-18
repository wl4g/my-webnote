import { openDB, DBSchema, IDBPDatabase } from 'idb';
import { v4 as uuidv4 } from 'uuid';
import moment from 'moment-timezone';
import { FileTree } from '../types/file';
import {
  MyWebnoteFile,
  MyWebnoteFolder,
  MyWebnoteFileType,
  MyWebnoteFolderFileMapping
} from '../types/file';

moment.tz.setDefault('Asia/Shanghai');

export interface MyWebnoteDBSchema extends DBSchema {
  folder: {
    key: string;
    value: MyWebnoteFolder;
  };
  file: {
    key: string;
    value: MyWebnoteFile;
  };
  folder_file_mapping: {
    key: number;
    value: MyWebnoteFolderFileMapping;
  };
}

export const INDEXEDDB_FOLDER_KEY = 'folder';
export const INDEXEDDB_FILE_KEY = 'file';
export const INDEXEDDB_FOLD_FILE_MAPPING_KEY = 'folder_file_mapping';
export const LOCALSTORAGE_FIRST_FOLDER_KEY = 'first_forlder_id';
export const LOCALSTORAGE_FIRST_FILE_KEY = 'first_file_id';
export const INDEXEDDB_REVEZONE_MENU = 'mywebnote_menu';

class MenuIndexeddbStorage {
  constructor() {
    if (MenuIndexeddbStorage.instance) {
      return MenuIndexeddbStorage.instance;
    }

    MenuIndexeddbStorage.instance = this;

    (async () => {
      this.db = await this.initDB();
    })();
  }

  static instance: MenuIndexeddbStorage;
  db: IDBPDatabase<MyWebnoteDBSchema> | undefined;

  async initDB(): Promise<IDBPDatabase<MyWebnoteDBSchema>> {
    if (this.db) {
      return this.db;
    }

    const db = await openDB<MyWebnoteDBSchema>(INDEXEDDB_REVEZONE_MENU, 1, {
      upgrade: async (db) => {
        await this.initFolderStore(db);
        await this.initFileStore(db);
        await this.initFolderFileMappingStore(db);
      }
    });

    this.db = db;

    return db;
  }

  async initFolderStore(db): Promise<IDBObjectStore> {
    const folderStore: IDBObjectStore = await db.createObjectStore(INDEXEDDB_FOLDER_KEY, {
      autoIncrement: true
    });

    return folderStore;
  }

  async initFolderFileMappingStore(db): Promise<IDBObjectStore> {
    const folderFileMappingStore: IDBObjectStore = await db.createObjectStore(
      INDEXEDDB_FOLD_FILE_MAPPING_KEY,
      {
        autoIncrement: true
      }
    );

    await folderFileMappingStore.createIndex('folderId', 'folderId', { unique: false });
    await folderFileMappingStore.createIndex('fileId', 'fileId', { unique: true });

    const mapping = {
      folderId: localStorage.getItem(LOCALSTORAGE_FIRST_FOLDER_KEY),
      fileId: localStorage.getItem(LOCALSTORAGE_FIRST_FILE_KEY)
    };

    await folderFileMappingStore.add(mapping);

    return folderFileMappingStore;
  }

  async initFileStore(db): Promise<IDBObjectStore> {
    const fileStore: IDBObjectStore = await db.createObjectStore(INDEXEDDB_FILE_KEY, {
      autoIncrement: true
    });

    await fileStore.createIndex('type', 'type', { unique: false });

    return fileStore;
  }

  async addFolder(name?: string) {
    await this.initDB();
    const id = `folder_${uuidv4()}`;

    const folderInfo = {
      id,
      name: name || '',
      gmtCreate: moment().toLocaleString(),
      gmtModified: moment().toLocaleString()
    };

    await this.db?.add(INDEXEDDB_FOLDER_KEY, folderInfo, id);

    return folderInfo;
  }

  async getFolder(folderId: string): Promise<MyWebnoteFolder | undefined> {
    await this.initDB();
    // @ts-ignore
    const value = await this.db?.get(INDEXEDDB_FOLDER_KEY, folderId);
    return value;
  }

  async getFolders(): Promise<MyWebnoteFolder[]> {
    await this.initDB();
    const folders = await this.db?.getAll('folder');
    const sortFn = (a: MyWebnoteFolder, b: MyWebnoteFolder) =>
      new Date(a.gmtCreate).getTime() < new Date(b.gmtCreate).getTime() ? 1 : -1;
    return folders?.sort(sortFn) || [];
  }

  async addFile(
    folderId: string,
    type: MyWebnoteFileType = 'note',
    name?: string
  ): Promise<MyWebnoteFile> {
    await this.initDB();

    const fileId = `file_${uuidv4()}`;

    const fileInfo = {
      id: fileId,
      name: name || '',
      type,
      gmtCreate: moment().toLocaleString(),
      gmtModified: moment().toLocaleString()
    };

    await this.db?.add(INDEXEDDB_FILE_KEY, fileInfo, fileId);

    await this.db?.add(INDEXEDDB_FOLD_FILE_MAPPING_KEY, {
      folderId,
      fileId,
      gmtCreate: moment().toLocaleString(),
      gmtModified: moment().toLocaleString()
    });

    return fileInfo;
  }

  // TODO: NOT FINISHED, DO NOT USE
  async _copyFile(copyFileId: string, folderId: string) {
    await this.initDB();

    if (!(copyFileId && folderId)) return;

    const copyFile = await this.db?.get(INDEXEDDB_FILE_KEY, copyFileId);

    await this.addFile(folderId, copyFile?.type);

    // await blocksuiteStorage.copyPage();
  }

  async getFile(fileId: string): Promise<MyWebnoteFile | undefined> {
    await this.initDB();
    const value = await this.db?.get(INDEXEDDB_FILE_KEY, fileId);
    return value;
  }

  async deleteFile(file: MyWebnoteFile) {
    await this.initDB();

    file && (await this.db?.delete(INDEXEDDB_FILE_KEY, file.id));

    const folderFileMappingKeys = await this.db?.getAllKeysFromIndex(
      INDEXEDDB_FOLD_FILE_MAPPING_KEY,
      // @ts-ignore
      'fileId',
      file.id
    );

    const deleteFolderFileMappingPromises = folderFileMappingKeys?.map(async (key) =>
      this.db?.delete(INDEXEDDB_FOLD_FILE_MAPPING_KEY, key)
    );

    deleteFolderFileMappingPromises && (await Promise.all(deleteFolderFileMappingPromises));
  }

  async getFiles(): Promise<MyWebnoteFile[]> {
    await this.initDB();
    const files = await this.db?.getAll(INDEXEDDB_FILE_KEY);
    const sortFn = (a: MyWebnoteFile, b: MyWebnoteFile) =>
      new Date(a.gmtCreate).getTime() < new Date(b.gmtCreate).getTime() ? 1 : -1;
    return files?.sort(sortFn) || [];
  }

  async getAllFileFolderMappings(): Promise<MyWebnoteFolderFileMapping[]> {
    await this.initDB();
    const mappings = await this.db?.getAll(INDEXEDDB_FOLD_FILE_MAPPING_KEY);
    return mappings || [];
  }

  async getFileTree(): Promise<FileTree> {
    await this.initDB();
    const folders = await this.getFolders();
    const files = await this.getFiles();
    const mappings = await this.getAllFileFolderMappings();

    const tree = folders.map((folder) => {
      const children: MyWebnoteFile[] = [];

      const mappingsCertainFolder = mappings.filter((map) => map.folderId === folder.id);

      files.forEach((file) => {
        const _file = mappingsCertainFolder.find((map) => map.fileId === file.id);
        if (_file) {
          children.push(file);
        }
      });

      return { ...folder, children };
    });

    return tree;
  }

  async getFilesInFolder(folderId: string): Promise<MyWebnoteFile[] | undefined> {
    await this.initDB();

    const mappings = await this.db?.getAllFromIndex(
      INDEXEDDB_FOLD_FILE_MAPPING_KEY,
      // @ts-ignore
      'folderId',
      folderId
    );

    const promises = mappings
      ?.map(async (item) => this.getFile(item.fileId))
      .filter((item) => !!item);

    const files = mappings && promises && (await Promise.all(promises)).filter((item) => !!item);

    // @ts-ignore
    return files;
  }

  async updateFileName(file: MyWebnoteFile, name: string) {
    await this.initDB();

    if (name === file?.name) return;

    file &&
      (await this.db?.put(
        INDEXEDDB_FILE_KEY,
        { ...file, name, gmtModified: moment().toLocaleString() },
        file.id
      ));
  }

  async updateFileGmtModified(file: MyWebnoteFile) {
    await this.initDB();

    file &&
      (await this.db?.put(
        INDEXEDDB_FILE_KEY,
        { ...file, gmtModified: moment().toLocaleString() },
        file.id
      ));
  }

  async updateFolderName(folder: MyWebnoteFolder, name: string) {
    await this.initDB();
    if (name === folder?.name) return;
    folder && this.db?.put(INDEXEDDB_FOLDER_KEY, { ...folder, name }, folder.id);
  }

  async deleteFolder(folderId: string) {
    await this.initDB();

    if (!folderId) return;

    await this.db?.delete(INDEXEDDB_FOLDER_KEY, folderId);

    const filesInFolder = await this.getFilesInFolder(folderId);

    const deleteFilesPromise = filesInFolder?.map(async (file) => this.deleteFile(file));

    deleteFilesPromise && (await Promise.all(deleteFilesPromise));
  }
}

export const menuIndexeddbStorage = new MenuIndexeddbStorage();
