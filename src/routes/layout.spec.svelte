import { render } from '@testing-library/svelte';
import Layout from './layout.svelte';

test('renders correctly', () => {
  const { getByText } = render(Layout);

  const deploooyerLink = getByText('Deploooyer | D_D');
  expect(deploooyerLink).toBeInTheDocument();

  const deployContractsLink = getByText('Deploy Contracts');
  expect(deployContractsLink).toBeInTheDocument();

  const configLink = getByText('Config');
  expect(configLink).toBeInTheDocument();

  const deploymentsLink = getByText('Deployments');
  expect(deploymentsLink).toBeInTheDocument();

  const keyStoreLink = getByText('Key Store');
  expect(keyStoreLink).toBeInTheDocument();
});