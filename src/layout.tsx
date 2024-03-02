import { createContext, useState } from 'react';
import React from 'react';
import { Toaster } from 'react-hot-toast';

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
      <Toaster />
      <div className='container p-6'>
        <div className='flex gap-2 items-center'>
          <h1 className='text-2xl font-bold'>🚀 Deployooor</h1>

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

        <main className='p-12'>{children}</main>
      </div>
    </PageContext.Provider>
  );
};
