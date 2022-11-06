import {expect, it} from "bun:test";
import {orderlyQueue} from "../src/ans.ts";

// Input: s = "cba", k = 1
// Output: "acb"
it("test_1", () => {
  expect(orderlyQueue("cba", 1)).toBe("acb");
});

// Input: s = "baaca", k = 3
// Output: "aaabc"
it("test_2", () => {
  expect(orderlyQueue("baaca", 3)).toBe("aaabc");
});
