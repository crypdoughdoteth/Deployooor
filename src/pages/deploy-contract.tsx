import { useState } from 'react';
import { useStatus } from '../hooks';
import { invoke } from '@tauri-apps/api/tauri';

export const DeployContractPage = () => {
  const [pathToContract, setPathToContract] = useState('');
  const [evmVersion, setEvmVersion] = useState('');

  const [status, setStatus] = useStatus('idle');

  const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    try {
      e.preventDefault();
      setStatus('loading');
      console.log(pathToContract);
      const res = await invoke('compile_version', {
        path: pathToContract,
        version: evmVersion,
      });
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
        <label htmlFor='pathToContract' className='label'>
          Path To Contract
        </label>
        <input
          className='input input-bordered'
          type='text'
          id='pathToContract'
          value={pathToContract}
          onChange={(e) => setPathToContract(e.target.value)}
        />
      </div>

      <div className='form-control'>
        <label htmlFor='evmVersion' className='label'>
          EVM Version
        </label>
        <input
          className='input input-bordered'
          type='text'
          id='evmVersion'
          value={evmVersion}
          onChange={(e) => setEvmVersion(e.target.value)}
        />
      </div>

      <button className='btn btn-primary' type='submit'>
        Compile Contract
      </button>

      <pre>
        <code>{JSON.stringify(status, null, 2)}</code>
      </pre>
    </form>
  );
};
