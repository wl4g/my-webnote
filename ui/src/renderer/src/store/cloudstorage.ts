import { MyWebnoteFolder } from '@renderer/types/file';
import { IStorageService } from './storage';

export class CloudStorageService implements IStorageService {
  private apiUrl: string;
  constructor(apiUrl: string) {
    this.apiUrl = apiUrl;
  }

  async searchFolders(folderName: string | null): Promise<MyWebnoteFolder[]> {
    const query = {
      name: folderName
    };
    return this.doRequest('GET', '/modules/folder/query', query).then((res) => {
      return res['data'].map((f) => {
        return {
          key: f['key'],
          name: f['name'],
          gmtCreate: f['createTime'],
          gmtModified: f['updateTime']
        };
      });
    });
  }

  async saveFolder(folderKey: string, folderName: string): Promise<void> {
    this.doRequest('POST', '/modules/folder/save', {
      key: folderKey,
      name: folderName
    });
  }

  async searchFileNames(): Promise<string[]> {
    const data = {};
    return this.doRequest('GET', '/modules/document/query', data).then((res) => {
      return res['data'].map((doc) => doc.folder_key);
    });
  }

  async saveOpenFileNames(docNames: string[]): Promise<void> {
    const data = {
      docNames: docNames
    };
    this.doRequest('POST', '/modules/document/save', data);
  }

  async saveCurrentFile(
    fileKey: string,
    fileType: string,
    fileName: string | undefined | null,
    folderKey: string,
    content: string
  ): Promise<void> {
    const data = {
      key: fileKey,
      type: fileType,
      name: fileName,
      folderKey: folderKey,
      content: content
    };
    this.doRequest('POST', '/modules/document/save', data);
  }

  async loadCurrentFile(): Promise<string | null> {
    const data = {};
    return this.doRequest('GET', '/modules/document/query', data);
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
    this.doRequest('POST', '/sys/user/current', data);
  }

  async loadLangCode(): Promise<string | null> {
    const data = {};
    return this.doRequest('GET', '/sys/user/current', data).then((res) => {
      return res['lang'];
    });
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
export const API_URI = location.origin == 'file://' ? 'http://localhost:5173' : '/serve';
export const cloudStorageService = new CloudStorageService(API_URI);
