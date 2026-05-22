import { describe, it, expect } from 'vitest';

describe('vitest smoke test', () => {
  it('should be truthy', () => {
    expect(1 + 1).toBe(2);
  });

  it('jsdom environment available', () => {
    expect(typeof window).toBe('object');
    expect(typeof document).toBe('object');
  });
});
