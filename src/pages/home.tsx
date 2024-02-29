import { useContext, useEffect, useState } from 'react';
import { getConfig } from '../lib';
import { PageContext, Pages } from '../layout';

export const HomePage = () => {
  const { setCurrentPage } = useContext(PageContext);

  const [config, setConfig] = useState<any>();
  const configDoesntExist = config === null;

  useEffect(() => {
    (async () => {
      const config = await getConfig();
      console.log(config);
      setConfig(config);
    })();
  }, []);

  return (
    <>
      {configDoesntExist && (
        <div className='text-center'>
          <p>No config found.</p>
        </div>
      )}

      {configDoesntExist && (
        <button
          onClick={() => setCurrentPage(Pages.CreateConfig)}
          className='btn'
        >
          Create Config
        </button>
      )}
    </>
  );
};
