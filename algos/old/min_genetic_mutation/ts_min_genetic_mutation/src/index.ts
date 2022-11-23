function minMutation(start: string, end: string, bank: string[]): number {
  let visited = new Set<string>();
  let queue: [string, number][] = [[start, 0]];
  let active: [string, number] = [start, 0]; 

  while (queue.length > 0) {
    active = queue.pop()!;
    if (active[0] === end) { return active[1]; }
    if (visited.has(active[0])) { continue; }
    visited.add(active[0]);
    for (let candidate in bank) {
      if (hamDist(start, candidate) === 1) {
        queue.splice(0, 0, [candidate, active[1]+1]);
      }
    }
  }
  return -1;
};

function hamDist(a: string, b: string): number {
  let a_chars = a.split('');
  let b_chars = a.split('');
  let count = 0;
  for (let i = 0; i < a_chars.length; i++) {
    if (a_chars[i] != b_chars[i]) { count += 1; }
  }
  return count;
}

export {minMutation}
