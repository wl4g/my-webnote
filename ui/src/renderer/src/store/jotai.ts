import { atom } from 'jotai';
import { MyWebnoteFile, MyWebnoteFolder, FileTree } from '../types/file';

type Theme = 'light' | 'dark';

export const fileTreeAtom = atom<FileTree>([]);

export const currentFileAtom = atom<MyWebnoteFile | undefined | null>(undefined);

export const workspaceLoadedAtom = atom(false);

export const folderListAtom = atom<MyWebnoteFolder[]>([]);

export const currentFolderIdAtom = atom<string | undefined>(undefined);

export const siderbarCollapsedAtom = atom(false);

export const langCodeAtom = atom('en');

export const themeAtom = atom<Theme>('light');
