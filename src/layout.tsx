import { createContext, useState } from 'react';
import React from 'react';

export enum Pages {
  Home = 'home',
  GenerateKeystore = 'generate-keystore',
  CreateConfig = 'create-config',
  DeployContract = 'deploy-contract',
  Deployments = 'deployments',
}

export const PageContext = createContext<{
  currentPage: Pages;
  setCurrentPage: (page: Pages) => void;
}>({
  currentPage: Pages.Home,
  setCurrentPage: () => {},
});

export const Layout = ({ children }: { children: React.ReactNode }) => {
  const [currentPage, setCurrentPage] = useState<Pages>(Pages.Home);

  return (
    <PageContext.Provider
      value={{
        currentPage,
        setCurrentPage: (page: Pages) => setCurrentPage(page),
      }}
    >
      <div className='container p-6'>
        <div className='flex gap-2 items-center'>
          <button
            className='btn'
            onClick={() => setCurrentPage(Pages.DeployContract)}
          >
            Deploy Contract
          </button>

          <button
            className='btn'
            onClick={() => setCurrentPage(Pages.GenerateKeystore)}
          >
            Generate Keystore
          </button>

          <button
            className='btn'
            onClick={() => setCurrentPage(Pages.CreateConfig)}
          >
            Config
          </button>

          <button
            className='btn'
            onClick={() => setCurrentPage(Pages.Deployments)}
          >
            Deployments
          </button>
        </div>

        <h1 className='text-2xl font-bold'>Welcome to Vyper Deployooor!</h1>

        {children}
      </div>
    </PageContext.Provider>
  );
};
