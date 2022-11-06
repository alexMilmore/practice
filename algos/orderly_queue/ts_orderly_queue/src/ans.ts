function orderlyQueue(s: string, k: number): string {
  if (k == 1) {
    let best = s;
    let mem = s;
    let repeat_str = s+s;
    for (let i=0; i < s.length; i++) {
      mem = repeat_str.slice(i, i+s.length);
      if (mem < best) { best = mem; }
    }
    return best;
  }
  return s.split('').sort().join('');
};

export {orderlyQueue};
