import { localStorageService } from './localstorage';
import { cloudStorageService } from './cloudstorage';

export interface IStorageService {
  searchFileNames(): Promise<string[]>;
  saveOpenFileNames(docNames: string[]): void;
  saveCurrentFile(fileId: string | undefined | null): void;
  loadCurrentFile(): Promise<string | null>;
  loadBoardCustomFont(): Promise<string | null>;
  saveBoardCustomFont(fontName: string | null): void;
  addBoardCustomFont(fontFamilyName: string): void;
  saveLangCode(langCode: string): void;
  loadLangCode(): Promise<string | null>;
  loadBoardCustomFontSwitch(): Promise<string | null>;
  saveBoardCustomFontSwitch(value: boolean): void;
}

export class StorageAdapter implements IStorageService {
  private delegate: IStorageService;

  constructor() {
    this.delegate = cloudStorageService;
  }

  async setupStorageType(storageType: string) {
    if (storageType == 'LOCAL') {
      this.delegate = localStorageService;
    } else {
      this.delegate = cloudStorageService;
    }
  }

  async searchFileNames(): Promise<string[]> {
    return this.delegate.searchFileNames();
  }

  async saveOpenFileNames(docNames: string[]): Promise<void> {
    this.delegate.saveOpenFileNames(docNames);
  }

  async saveCurrentFile(fileId: string | undefined | null): Promise<void> {
    this.delegate.saveCurrentFile(fileId);
  }

  async loadCurrentFile(): Promise<string | null> {
    return this.delegate.loadCurrentFile();
  }

  async loadBoardCustomFont(): Promise<string | null> {
    return this.delegate.loadBoardCustomFont();
  }

  async saveBoardCustomFont(fontName: string | null): Promise<void> {
    this.delegate.saveBoardCustomFont(fontName);
  }

  async addBoardCustomFont(fontFamilyName: string): Promise<void> {
    this.delegate.addBoardCustomFont(fontFamilyName);
  }

  async saveLangCode(langCode: string): Promise<void> {
    this.delegate.saveLangCode(langCode);
  }

  async loadLangCode(): Promise<string | null> {
    return this.delegate.loadLangCode();
  }

  async loadBoardCustomFontSwitch(): Promise<string | null> {
    return this.delegate.loadBoardCustomFontSwitch();
  }

  async saveBoardCustomFontSwitch(boardCustomFont: boolean): Promise<void> {
    this.delegate.saveBoardCustomFontSwitch(boardCustomFont);
  }
}

export const storageAdapter: StorageAdapter = new StorageAdapter();
