import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';
import { useStatus } from '../hooks'

export const GenerateKeystorePage = () => {
  const [password, setPassword] = useState('');
  const [name, setName] = useState('');

  const [status, setStatus] = useStatus('idle');

  const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    setStatus('loading')
    try {
      e.preventDefault();
  
      const res = await invoke('generate_keystore', {
        path: './',
        password,
        name,
      });

      console.log(res);
    } catch (e) {
      console.log(e)
      setStatus('error')
    } finally {
      setStatus('success')
    }
  };

  return (
    <form className='flex flex-col gap-4' onSubmit={onSubmit}>
      <div className='form-control'>
        <label htmlFor='password' className='label'>
          Password
        </label>
        <input
          className='input input-bordered'
          type='password'
          id='password'
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
      </div>

      <div className='form-control'>
        <label htmlFor='name' className='label'>
          Name
        </label>
        <input
          className='input input-bordered'
          type='text'
          id='name'
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
      </div>

      <button className='btn btn-primary' type='submit'>
        Generate Keystore
      </button>

      <pre>
        <code>
          {JSON.stringify(status, null, 2)}
        </code>
      </pre>
    </form>
  );
};
