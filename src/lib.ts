import { invoke } from '@tauri-apps/api/tauri';

export const getConfig = async () => {
  try {
    const config = await invoke('get_config');
    return config;
  } catch (e) {
    console.log(e);
    return null;
  }
};

export const generateKeystore = ({
  path,
  password,
  name,
}: {
  path: string;
  password: string;
  name: string;
}) => {
  return invoke('generate_keystore', { path, password, name });
};

export const createConfig = ({
  providerUrl,
  keystorePath,
}: {
  providerUrl: string;
  keystorePath: string;
}) => {
  return invoke('set_config', { providerUrl, keystorePath });
};
