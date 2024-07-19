import { render, fireEvent } from '@testing-library/svelte';
import Page from './page.svelte';

test('should render provider URL input field', () => {
  const { getByLabelText } = render(Page);
  const providerInput = getByLabelText('Provider URL');
  expect(providerInput).toBeInTheDocument();
});

test('should render etherscan API Key input field', () => {
  const { getByLabelText } = render(Page);
  const apiKeyInput = getByLabelText('Etherscan API Key');
  expect(apiKeyInput).toBeInTheDocument();
});

test('should render Set Config button', () => {
  const { getByText } = render(Page);
  const setConfigButton = getByText('Set Config');
  expect(setConfigButton).toBeInTheDocument();
});

test('should trigger click event on Set Config button', async () => {
  const { getByText } = render(Page);
  const setConfigButton = getByText('Set Config');
  fireEvent.click(setConfigButton);

  // Add assertions here for the expected behavior after clicking the button
});