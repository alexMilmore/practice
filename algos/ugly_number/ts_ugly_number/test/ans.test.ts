import {expect, it} from 'bun:test';
import {isUgly} from '../src/ans.ts';

// Input: n = 6
// Output: true
it("test_1", () => {
  const ans = isUgly(6);
  expect(ans).toBe(true);
});

// Input: n = 1
// Output: true
it("test_2", () => {
  const ans = isUgly(1);
  expect(ans).toBe(true);
});

// Input: n = 14
// Output: false
it("test_3", () => {
  const ans = isUgly(14);
  expect(ans).toBe(false);
});

// Input: n = 0
// Output: false
it("test_4", () => {
  const ans = isUgly(0);
  expect(ans).toBe(false);
});

// Input: n = 8
// Output: true
it("test_5", () => {
  const ans = isUgly(8);
  expect(ans).toBe(true);
});
