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
import { storageAdapter } from '../store/storage';

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

    await folderFileMappingStore.createIndex('folderKey', 'folderKey', { unique: false });
    await folderFileMappingStore.createIndex('fileKey', 'fileKey', { unique: true });

    const mapping = {
      folderKey: localStorage.getItem(LOCALSTORAGE_FIRST_FOLDER_KEY),
      fileKey: localStorage.getItem(LOCALSTORAGE_FIRST_FILE_KEY)
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
    const folderKey = `folder_${uuidv4()}`;

    const folderInfo = {
      key: folderKey,
      name: name || '',
      gmtCreate: moment().toLocaleString(),
      gmtModified: moment().toLocaleString()
    };

    await this.db?.add(INDEXEDDB_FOLDER_KEY, folderInfo, folderKey);

    return folderInfo;
  }

  async getFolder(folderKey: string): Promise<MyWebnoteFolder | undefined> {
    await this.initDB();
    // @ts-ignore
    const value = await this.db?.get(INDEXEDDB_FOLDER_KEY, folderKey);
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
    folderKey: string,
    type: MyWebnoteFileType = 'Note',
    name?: string
  ): Promise<MyWebnoteFile> {
    await this.initDB();

    const fileKey = `file_${uuidv4()}`;

    const fileInfo = {
      key: fileKey,
      name: name || '',
      type,
      gmtCreate: moment().toLocaleString(),
      gmtModified: moment().toLocaleString()
    };

    await this.db?.add(INDEXEDDB_FILE_KEY, fileInfo, fileKey);

    await this.db?.add(INDEXEDDB_FOLD_FILE_MAPPING_KEY, {
      folderKey,
      fileKey,
      gmtCreate: moment().toLocaleString(),
      gmtModified: moment().toLocaleString()
    });

    return fileInfo;
  }

  // TODO: NOT FINISHED, DO NOT USE
  async _copyFile(copyFileId: string, folderKey: string) {
    await this.initDB();

    if (!(copyFileId && folderKey)) return;

    const copyFile = await this.db?.get(INDEXEDDB_FILE_KEY, copyFileId);

    await this.addFile(folderKey, copyFile?.type);

    // await blocksuiteStorage.copyPage();
  }

  async getFile(fileKey: string): Promise<MyWebnoteFile | undefined> {
    await this.initDB();
    const value = await this.db?.get(INDEXEDDB_FILE_KEY, fileKey);
    return value;
  }

  async deleteFile(file: MyWebnoteFile) {
    await this.initDB();

    file && (await this.db?.delete(INDEXEDDB_FILE_KEY, file.key));

    const folderFileMappingKeys = await this.db?.getAllKeysFromIndex(
      INDEXEDDB_FOLD_FILE_MAPPING_KEY,
      // @ts-ignore
      'fileKey',
      file.key
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

    // MODIFIED:
    //const folders = await this.getFolders();
    //const files = await this.getFiles();
    const folders = await storageAdapter.searchFolders(null);
    const files = await storageAdapter.searchFileNames();
    const mappings = await this.getAllFileFolderMappings();

    const tree = folders.map((folder) => {
      const children: MyWebnoteFile[] = [];

      const mappingsCertainFolder = mappings.filter((map) => map.folderKey === folder.key);

      files.forEach((file) => {
        const _file = mappingsCertainFolder.find((map) => map.fileKey === file.key);
        if (_file) {
          children.push(file);
        }
      });

      return { ...folder, children };
    });

    return tree;
  }

  async getFilesInFolder(folderKey: string): Promise<MyWebnoteFile[] | undefined> {
    await this.initDB();

    const mappings = await this.db?.getAllFromIndex(
      INDEXEDDB_FOLD_FILE_MAPPING_KEY,
      // @ts-ignore
      'folderKey',
      folderKey
    );

    const promises = mappings
      ?.map(async (item) => this.getFile(item.fileKey))
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
        file.key
      ));
  }

  async updateFileGmtModified(file: MyWebnoteFile) {
    await this.initDB();

    file &&
      (await this.db?.put(
        INDEXEDDB_FILE_KEY,
        { ...file, gmtModified: moment().toLocaleString() },
        file.key
      ));
  }

  async updateFolderName(folder: MyWebnoteFolder, name: string) {
    await this.initDB();
    if (name === folder?.name) return;
    folder && this.db?.put(INDEXEDDB_FOLDER_KEY, { ...folder, name }, folder.key);
  }

  async deleteFolder(folderKey: string) {
    await this.initDB();

    if (!folderKey) return;

    await this.db?.delete(INDEXEDDB_FOLDER_KEY, folderKey);

    const filesInFolder = await this.getFilesInFolder(folderKey);

    const deleteFilesPromise = filesInFolder?.map(async (file) => this.deleteFile(file));

    deleteFilesPromise && (await Promise.all(deleteFilesPromise));
  }
}

export const menuIndexeddbStorage = new MenuIndexeddbStorage();
