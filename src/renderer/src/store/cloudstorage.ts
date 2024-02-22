import { IStorageService } from './storage';

export class CloudStorageService implements IStorageService {
  private apiUrl: string;
  constructor(apiUrl: string) {
    this.apiUrl = apiUrl;
  }

  loadOpenKeys(): Promise<string[]> {
    const data = {};
    return this.doRequest('GET', '/openKeys', data).then((res) => {
      return [res];
    });
  }

  saveOpenKeys(openKeys: string[]): void {
    const data = {
      openKeys: openKeys
    };
    this.doRequest('POST', '/openKeys', data);
  }

  saveCurrentFileId(fileId: string | undefined | null): void {
    const data = {
      fileId: fileId
    };
    this.doRequest('POST', '/currentFileId', data);
  }

  loadCurrentFileId(): string | null {
    const data = {};
    return this.doRequest('GET', '/currentFileId', data);
  }

  loadBoardCustomFont(): string | null {
    const data = {};
    return this.doRequest('GET', '/boardCustomFont', data);
  }

  saveBoardCustomFont(fontName: string | null): void {
    const data = {
      fontName: fontName
    };
    this.doRequest('POST', '/boardCustomFont', data);
  }

  addBoardCustomFont(fontFamilyName: string): void {
    const data = {
      fontFamilyName: fontFamilyName
    };
    this.doRequest('POST', '/addBoardCustomFont', data);
  }

  saveLangCode(langCode: string): void {
    const data = {
      langCode: langCode
    };
    this.doRequest('POST', '/langCode', data);
  }

  loadLangCode(): string | null {
    const data = {};
    return this.doRequest('GET', '/langCode', data);
  }

  loadBoardCustomFontSwitch(): string | null {
    const data = {};
    return this.doRequest('GET', '/boardCustomFont', data);
  }

  saveBoardCustomFontSwitch(boardCustomFont: boolean): void {
    const data = {
      boardCustomFont: boardCustomFont
    };
    this.doRequest('POST', '/boardCustomFont', data);
  }

  private async doRequest(method: string, path: string, data: object): Promise<string> {
    try {
      let options = {};
      if (method.toUpperCase() == 'GET') {
        options = {
          method: method,
          headers: {}
        };
      } else if (method.toUpperCase() == 'POST') {
        options = {
          method: method,
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(data)
        };
      }
      const response = await fetch(this.apiUrl + path, options);
      if (!response.ok) {
        throw new Error(`HTTP error ${response.status}`);
      }
      return response.json();
    } catch (error) {
      console.error('Error request data to cloud:', error);
      throw error;
    }
  }
}

// If in electron env the location.origin is 'file://'
export const DEFAULT_API_URL =
  location.origin == 'file://' ? 'http://localhost:8888' : location.origin;
export const cloudStorageService: CloudStorageService = new CloudStorageService(DEFAULT_API_URL);
