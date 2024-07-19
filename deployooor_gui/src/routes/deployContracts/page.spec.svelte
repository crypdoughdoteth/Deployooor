import { render, screen, fireEvent } from '@testing-library/svelte';
import { Stepper, Step } from '@skeletonlabs/skeleton';
import { getToastStore, getModalStore } from '@skeletonlabs/skeleton';

describe('Stepper', () => {
  test('should render Stepper component', () => {
    render(Stepper);
    const stepperComponent = screen.getByTestId('stepper');
    expect(stepperComponent).toBeInTheDocument();
  });

  test('should render Step component', () => {
    render(Step);
    const stepComponent = screen.getByTestId('step');
    expect(stepComponent).toBeInTheDocument();
  });

  test('should trigger handleStep function on next', async () => {
    const handleStep = jest.fn();
    render(Stepper, {
      props: { on: { next: handleStep } }
    });
    const nextButton = screen.getByText('Next');
    await fireEvent.click(nextButton);
    expect(handleStep).toHaveBeenCalledTimes(1);
  });
});

describe('Modal', () => {
  test('should open modal', async () => {
    const modalStore = getModalStore();
    render(Modal, {
      props: { store: modalStore }
    });
    const openButton = screen.getByText('Open Modal');
    await fireEvent.click(openButton);
    const modalComponent = screen.getByTestId('modal');
    expect(modalComponent).toBeVisible();
  });

  test('should close modal', async () => {
    const modalStore = getModalStore();
    render(Modal, {
      props: { store: modalStore }
    });
    const closeButton = screen.getByText('Close');
    await fireEvent.click(closeButton);
    const modalComponent = screen.queryByTestId('modal');
    expect(modalComponent).not.toBeInTheDocument();
  });
});

describe('Form', () => {
  test('should update input value', async () => {
    render(Form);
    const input = screen.getByLabelText('Input Field');
    await fireEvent.change(input, { target: { value: 'New Value' } });
    expect(input.value).toBe('New Value');
  });

  test('should submit form with valid data', async () => {
    const toastStore = getToastStore();
    render(Form, {
      props: { store: toastStore }
    });
    const submitButton = screen.getByText('Submit');
    await fireEvent.click(submitButton);
    const successMessage = screen.getByText('Form submitted successfully');
    expect(successMessage).toBeVisible();
  });
});