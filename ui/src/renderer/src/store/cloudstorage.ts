import { IStorageService } from './storage';

export class CloudStorageService implements IStorageService {
  private apiUrl: string;
  constructor(apiUrl: string) {
    this.apiUrl = apiUrl;
  }

  async searchFileNames(): Promise<string[]> {
    const data = {};
    return this.doRequest('GET', '/modules/document/list', data).then((res) => {
      return [res];
    });
  }

  async saveOpenFileNames(docNames: string[]): Promise<void> {
    const data = {
      docNames: docNames
    };
    this.doRequest('POST', '/document/save', data);
  }

  async saveCurrentFile(fileId: string | undefined | null): Promise<void> {
    const data = {
      fileId: fileId
    };
    this.doRequest('POST', '/document/current-save', data);
  }

  async loadCurrentFile(): Promise<string | null> {
    const data = {};
    return this.doRequest('GET', '/document/current-get', data);
  }

  async loadBoardCustomFont(): Promise<string | null> {
    const data = {};
    return this.doRequest('GET', '/board/custom-font/get', data);
  }

  async saveBoardCustomFont(fontName: string | null): Promise<void> {
    const data = {
      fontName: fontName
    };
    this.doRequest('POST', '/board/custom-font/save', data);
  }

  async addBoardCustomFont(fontFamilyName: string): Promise<void> {
    const data = {
      fontFamilyName: fontFamilyName
    };
    this.doRequest('POST', '/board/custom-font/add', data);
  }

  async loadBoardCustomFontSwitch(): Promise<string | null> {
    const data = {};
    return this.doRequest('GET', '/board/custom-font-switch/get', data);
  }

  async saveBoardCustomFontSwitch(boardCustomFont: boolean): Promise<void> {
    const data = {
      boardCustomFont: boardCustomFont
    };
    this.doRequest('POST', '/board/custom-font-switch/save', data);
  }

  async saveLangCode(langCode: string): Promise<void> {
    const data = {
      langCode: langCode
    };
    this.doRequest('POST', '/setting/lang/save', data);
  }

  async loadLangCode(): Promise<string | null> {
    const data = {};
    return this.doRequest('GET', '/settings/lang/get', data);
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
export const API_URI = location.origin != 'file://' ? 'http://localhost:4523' : location.origin;
export const cloudStorageService: CloudStorageService = new CloudStorageService(API_URI + '/m1/4058706-0-default/api/v1');
