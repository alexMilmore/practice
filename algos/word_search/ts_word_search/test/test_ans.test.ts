import { expect, it } from "bun:test";
import { exist } from "../src/ans.ts";

// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
// Output: true
it("test_1", () => {
  const ans = exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCCED");
  expect(ans).toBe(true);
});

// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
// Output: true
it("test_2", () => {
  const ans = exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "SEE");
  expect(ans).toBe(true);
});

// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
// Output: false
it("test_3", () => {
  const ans = exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCB");
  expect(ans).toBe(false);
});
