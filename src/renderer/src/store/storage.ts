import { localStorageService } from './localstorage';
import { cloudStorageService } from './cloudstorage';

export interface IStorageService {
  loadOpenKeys(): Promise<string[]>;
  saveOpenKeys(openKeys: string[]): void;
  saveCurrentFileId(fileId: string | undefined | null): void;
  loadCurrentFileId(): string | null;
  loadBoardCustomFont(): string | null;
  saveBoardCustomFont(fontName: string | null): void;
  addBoardCustomFont(fontFamilyName: string): void;
  saveLangCode(langCode: string): void;
  loadLangCode(): string | null;
  loadBoardCustomFontSwitch(): string | null;
  saveBoardCustomFontSwitch(value: boolean): void;
}

export class StorageAdapter implements IStorageService {
  private delegate: IStorageService;

  constructor() {
    this.delegate = localStorageService;
  }

  setupStorageType(storageType: string) {
    if (storageType == 'CLOUD') {
      this.delegate = cloudStorageService;
    } else {
      this.delegate = localStorageService;
    }
  }

  loadOpenKeys(): Promise<string[]> {
    return this.delegate.loadOpenKeys();
  }

  saveOpenKeys(openKeys: string[]): void {
    this.delegate.saveOpenKeys(openKeys);
  }

  saveCurrentFileId(fileId: string | undefined | null): void {
    this.delegate.saveCurrentFileId(fileId);
  }

  loadCurrentFileId(): string | null {
    return this.delegate.loadCurrentFileId();
  }

  loadBoardCustomFont(): string | null {
    return this.delegate.loadBoardCustomFont();
  }

  saveBoardCustomFont(fontName: string | null): void {
    this.delegate.saveBoardCustomFont(fontName);
  }

  addBoardCustomFont(fontFamilyName: string): void {
    this.delegate.addBoardCustomFont(fontFamilyName);
  }

  saveLangCode(langCode: string): void {
    this.delegate.saveLangCode(langCode);
  }

  loadLangCode(): string | null {
    return this.delegate.loadLangCode();
  }

  loadBoardCustomFontSwitch(): string | null {
    return this.delegate.loadBoardCustomFontSwitch();
  }

  saveBoardCustomFontSwitch(boardCustomFont: boolean): void {
    this.delegate.saveBoardCustomFontSwitch(boardCustomFont);
  }
}

export const storageAdapter: StorageAdapter = new StorageAdapter();
