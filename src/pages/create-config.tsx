import { useEffect, useState } from 'react';
import { useStatus } from '../hooks';
import { invoke } from '@tauri-apps/api/tauri'

export const CreateConfigPage = () => {
  const [providerUrl, setProviderUrl] = useState('');
  const [keystoreName, setKeystoreName] = useState('');
  const [status, setStatus] = useStatus('idle');

  useEffect(() => {
    (async () => {
      const config = await invoke('get_config') as {
        provider: string;
        keystore: string;
      };
      setProviderUrl(config.provider);
      setKeystoreName(config.keystore);
    })()
  }, [])

  const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    try {
      e.preventDefault();
      setStatus('loading')
      const keystorePath = `./${keystoreName}`
      const res =await invoke('set_config', { provider: providerUrl, keystore: keystorePath });
      console.log(res);
    } catch (error) {
      console.log(error);
      setStatus('error');
    } finally {
      setStatus('success');
    }
  };

  return (
    <form className='flex flex-col gap-4' onSubmit={onSubmit}>
      <div className='form-control'>
        <label htmlFor='providerUrl' className='label'>
          Provider URL
        </label>
        <input
          className='input input-bordered'
          type='text'
          id='providerUrl'
          value={providerUrl}
          onChange={(e) => setProviderUrl(e.target.value)}
        />
      </div>

      <div className='form-control'>
        <label htmlFor='keystoreName' className='label'>
          Keystore Name
        </label>
        <input
          className='input input-bordered'
          type='text'
          id='keystoreName'
          value={keystoreName}
          onChange={(e) => setKeystoreName(e.target.value)}
        />
      </div>

      <button className='btn btn-primary' type='submit'>
        Create Config
      </button>

      <pre>
        <code>
          {JSON.stringify(status, null, 2)}
        </code>
      </pre>
    </form>
  );
};
