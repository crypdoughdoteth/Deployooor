import { render, fireEvent, getByLabelText } from '@testing-library/svelte';
import Page from './page.svelte';

test('should render the page correctly', () => {
  const { getByText, getByLabelText } = render(Page);

  const passwordInput = getByLabelText('Password');
  const nameInput = getByLabelText('Name');
  const submitButton = getByText('Submit');

  expect(passwordInput).toBeInTheDocument();
  expect(nameInput).toBeInTheDocument();
  expect(submitButton).toBeInTheDocument();
});

test('should update password value correctly', () => {
  const { getByLabelText } = render(Page);

  const passwordInput = getByLabelText('Password');
  
  fireEvent.input(passwordInput, { target: { value: 'password123' } });

  expect(passwordInput.value).toBe('password123');
});

test('should update name value correctly', () => {
  const { getByLabelText } = render(Page);

  const nameInput = getByLabelText('Name');
  
  fireEvent.input(nameInput, { target: { value: 'John Doe' } });

  expect(nameInput.value).toBe('John Doe');
});

test('should execute submit button click correctly', () => {
  const { getByText } = render(Page);

  const submitButton = getByText('Submit');
  fireEvent.click(submitButton);

  // Add assertions for the desired behavior after clicking the submit button
});