import { useEffect, useState } from 'react';
import { useStatus } from '../hooks';
import { invoke } from '@tauri-apps/api/tauri';
import { toast } from 'react-hot-toast';

export const CreateConfigPage = () => {
  const [providerUrl, setProviderUrl] = useState('');
  const [etherscanApiKey, setEtherscanApiKey] = useState('');
  const [status, setStatus] = useStatus('idle');

  useEffect(() => {
    (async () => {
      const config = (await invoke('get_config')) as {
        provider: string;
        etherscan_api: string;
      };
      setProviderUrl(config.provider);
      setEtherscanApiKey(config.etherscan_api);
    })();
  }, []);

  const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    try {
      e.preventDefault();
      setStatus('loading');
      const res = await invoke('set_config', {
        provider: providerUrl,
        etherscanApi: etherscanApiKey,
      });
      console.log(res);
    } catch (error) {
      console.log(error);
      setStatus('error');
      toast.error('Error setting config');
    } finally {
      setStatus('success');
      toast.success('Config set');
    }
  };

  return (
    <form className='flex flex-col gap-4' onSubmit={onSubmit}>
      <div className='form-control'>
        <label htmlFor='providerUrl' className='label'>
          ⛓️ Provider URL
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
        <label htmlFor='etherscanApiKey' className='label'>
          🔑 Etherscan API key
        </label>
        <input
          className='input input-bordered'
          type='text'
          id='etherscanApiKey'
          value={etherscanApiKey}
          onChange={(e) => setEtherscanApiKey(e.target.value)}
        />
      </div>

      <button className='btn btn-primary' type='submit'>
        {status === 'loading' && (
          <span className='loading loading-spinner'></span>
        )}
        Set Config
      </button>
    </form>
  );
};
