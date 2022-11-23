import {minMutation} from '../src/index'

// Input: start = "AACCGGTT", end = "AACCGGTA", bank = ["AACCGGTA"]
// Output: 1
test('denji', () => {
  const ans = minMutation("AACCGGTT", "AACCGGTA", ["AACCGGTA"]);
  expect(ans).toBe(1);
});

// Input: start = "AACCGGTT", end = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
// Output: 2
test('aki', () => {
  const ans = minMutation("AACCGGTT", "AAACGGTA",
                          ["AACCGGTA","AACCGCTA","AAACGGTA"]);
  expect(ans).toBe(2);
});

// Input: start = "AAAAACCC", end = "AACCCCCC", bank = ["AAAACCCC","AAACCCCC","AACCCCCC"]
// Output: 3
test('power', () => {
  const ans = minMutation("AAAAACCC", "AACCCCCC",
                          ["AAAACCCC","AAACCCCC","AACCCCCC"]);
  expect(ans).toBe(3);
});
