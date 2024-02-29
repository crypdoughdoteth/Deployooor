import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './styles.css';
import { Layout } from './layout';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <Layout>
      <App />
    </Layout>
  </React.StrictMode>
);
