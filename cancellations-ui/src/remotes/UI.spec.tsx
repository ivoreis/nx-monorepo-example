import { render } from '@testing-library/react';

import UI from './UI';

describe('Operation UI', () => {
  it('should render successfully for admin', () => {
    const { baseElement } = render(<UI platform="admin" id="1234" />);
    expect(baseElement).toBeTruthy();
  });

  it('should render successfully for customer', () => {
    const { baseElement } = render(<UI platform="customer" id="1234" />);
    expect(baseElement).toBeTruthy();
  });

  it('should render successfully for business', () => {
    const { baseElement } = render(<UI platform="business" id="1234" />);
    expect(baseElement).toBeTruthy();
  });

  it('should have a title', () => {
    const { getByText } = render(<UI platform="admin" id="1234" />);
    expect(getByText(/Cancellation Page/gi)).toBeTruthy();
  });
});
