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
    this.delegate = localStorageService;
  }

  setupStorageType(storageType: string) {
    if (storageType == 'LOCAL') {
      this.delegate = localStorageService;
    } else {
      this.delegate = cloudStorageService;
    }
  }

  searchFileNames(): Promise<string[]> {
    return this.delegate.searchFileNames();
  }

  saveOpenFileNames(docNames: string[]): void {
    this.delegate.saveOpenFileNames(docNames);
  }

  saveCurrentFile(fileId: string | undefined | null): void {
    this.delegate.saveCurrentFile(fileId);
  }

  loadCurrentFile(): Promise<string | null> {
    return this.delegate.loadCurrentFile();
  }

  loadBoardCustomFont(): Promise<string | null> {
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

  loadLangCode(): Promise<string | null> {
    return this.delegate.loadLangCode();
  }

  loadBoardCustomFontSwitch(): Promise<string | null> {
    return this.delegate.loadBoardCustomFontSwitch();
  }

  saveBoardCustomFontSwitch(boardCustomFont: boolean): void {
    this.delegate.saveBoardCustomFontSwitch(boardCustomFont);
  }
}

export const storageAdapter: StorageAdapter = new StorageAdapter();
