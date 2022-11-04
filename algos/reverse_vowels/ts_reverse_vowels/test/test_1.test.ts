// STATUS: FAIL
import { expect, it } from "bun:test";
import { reverseVowels } from "../src/ans";

// Input: s = "hello"
// Output: "holle"
it("test_1", () => {
  expect(reverseVowels("hello")).toBe("holle");
});

// Input: s = "leetcode"
// Output: "leotcede"
it("test_2", () => {
  expect(reverseVowels("leetcode")).toBe("leotcede");
});
