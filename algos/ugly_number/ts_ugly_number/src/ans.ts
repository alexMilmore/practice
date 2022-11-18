function isUgly(n: number): boolean {
  if (n === 0) { return false; }
  let divisors = [2,3,5];
  divisors.forEach((x)=>{while (n%x==0) {n /= x}});
  return n === 1;
};

export {isUgly};
