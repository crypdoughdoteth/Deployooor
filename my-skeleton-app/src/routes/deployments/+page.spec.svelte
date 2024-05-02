import { render } from '@testing-library/svelte';
import Page from './page.svelte';

test('should render table container', () => {
  const { getByTestId } = render(Page);
  const tableContainer = getByTestId('table-container');
  
  expect(tableContainer).toBeInTheDocument();
});

test('should render table', () => {
  const { getByTestId } = render(Page);
  const table = getByTestId('table');
  
  expect(table).toBeInTheDocument();
});

test('should render table headers', () => {
  const { getByText } = render(Page);
  const scNameHeader = getByText('SC Name');
  const deployerAddressHeader = getByText('Deployer Address');
  const deployDateHeader = getByText('Deploy Date');
  const scAddressHeader = getByText('SC Address');
  const networkHeader = getByText('Network');
  const feeHeader = getByText('Fee');
  const verifiedHeader = getByText('Verified');
  
  expect(scNameHeader).toBeInTheDocument();
  expect(deployerAddressHeader).toBeInTheDocument();
  expect(deployDateHeader).toBeInTheDocument();
  expect(scAddressHeader).toBeInTheDocument();
  expect(networkHeader).toBeInTheDocument();
  expect(feeHeader).toBeInTheDocument();
  expect(verifiedHeader).toBeInTheDocument();
});