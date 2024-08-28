import { useCallback, useEffect, useState } from 'react';
import { FileTreeItem, MyWebnoteFile } from '@renderer/types/file';
import { Revedraw } from 'revemate';
import { ExcalidrawImperativeAPI, NonDeletedExcalidrawElement } from 'revemate/es/Revedraw/types';
import { boardIndexeddbStorage } from '@renderer/store/boardIndexeddb';
import { useDebounceFn } from 'ahooks';
import { currentFileAtom, fileTreeAtom, langCodeAtom } from '@renderer/store/jotai';
import { useAtom } from 'jotai';
import { getOSName } from '@renderer/utils/navigator';
import { getFileIdOrNameFromLink } from '@renderer/utils/file';
import { storageAdapter } from '../../store/storage';

import './index.css';

interface Props {
  file: MyWebnoteFile;
}

const OS_NAME = getOSName();

let firsRender = true;

export default function RevedrawApp({ file }: Props) {
  const [dataSource, setDataSource] = useState<string>();
  const [, setRef] = useState<ExcalidrawImperativeAPI>();
  const [fileTree] = useAtom(fileTreeAtom);
  const [, setCurrentFile] = useAtom(currentFileAtom);
  const [systemLangCode] = useAtom(langCodeAtom);
  const [didRender, setDidRender] = useState(true);

  const getDataSource = useCallback(async (id) => {
    // reset data source for a new canvas file
    setDataSource(undefined);

    const data = await boardIndexeddbStorage.getBoard(id);

    setDataSource(data);
  }, []);

  // HACK: fix the custom font not working completely when first render
  const rerender = useCallback(() => {
    const WAIT_TIME_WINDWOS = 500;
    const WAIT_TIME_MACOS = 200;

    const waitTime = OS_NAME === 'MacOS' ? WAIT_TIME_MACOS : WAIT_TIME_WINDWOS;

    setTimeout(() => {
      setDidRender(false);
      setTimeout(() => {
        setDidRender(true);
      }, 100);
    }, waitTime);
  }, []);

  useEffect(() => {
    if (firsRender) {
      firsRender = false;
      rerender();
    }
  }, []);

  const onChangeFn = useCallback(
    async (data) => {
      const str = JSON.stringify(data);
      await boardIndexeddbStorage.addOrUpdateBoard(file.id, str);

      // TODO:
      console.log('onChangeFn() -> addOrUpdateBoard() :: file: ', file, ' data:', data);
      await storageAdapter.saveCurrentFile(
        file.id,
        file.type,
        file.name,
        'xxxxxx-see-onFolderOrFileAdd',
        str
      );
    },
    [file.id]
  );

  const { run: onChangeDebounceFn, cancel: cancelDebounceFn } = useDebounceFn(onChangeFn, {
    wait: 200
  });

  const onLinkOpen = useCallback(
    (element: NonDeletedExcalidrawElement) => {
      const { link } = element;
      console.log('link', link);

      const fileIdOrNameInMyWebnote = link && getFileIdOrNameFromLink(link);

      if (fileIdOrNameInMyWebnote) {
        const files = fileTree?.reduce((prev: MyWebnoteFile[], item: FileTreeItem) => {
          return [...prev, ...item.children];
        }, []);

        const file = files.find(
          (_file) => _file.id === fileIdOrNameInMyWebnote || _file.name === fileIdOrNameInMyWebnote
        );

        if (file) {
          setCurrentFile(file);
        }
      } else {
        link && window.open(link);
      }
    },
    [fileTree]
  );

  useEffect(() => {
    getDataSource(file.id);
    return () => {
      cancelDebounceFn();
    };
  }, [file.id]);

  return dataSource ? (
    <>
      {didRender ? (
        <Revedraw
          dataSource={dataSource}
          canvasName={file.name}
          getRef={(ref) => setRef(ref)}
          systemLangCode={systemLangCode}
          onChange={onChangeDebounceFn}
          onLinkOpen={onLinkOpen}
        />
      ) : null}
    </>
  ) : null;
}
