export const getOSName = () => {
  let os;

  const userAgent = navigator.userAgent;

  switch (true) {
    case userAgent.includes('Win'):
      os = 'Windows';
      break;
    case userAgent.includes('Mac'):
      os = 'MacOS';
      break;
    case userAgent.includes('Linux'):
      os = 'Linux';
      break;
    default:
      os = 'Unkown';
      break;
  }

  return os;
};

export const getIsInMyWebnoteApp = () => {
  return navigator.userAgent.includes('mywebnote');
};

export const getAppVersion = (): string => {
  const regex = /mywebnote\/(\S+)/;
  return navigator.userAgent.match(regex)?.[1] || 'unkonwn';
};

export const isInMyWebnoteApp = getIsInMyWebnoteApp();
export const osName = getOSName();
export const appVersion = getAppVersion();
