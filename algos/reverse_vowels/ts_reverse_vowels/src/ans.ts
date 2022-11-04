function reverseVowels(s: string): string {
  let chars = s.split('');
  let i = 0;
  let j = chars.length-1;
  const vowels = new Set(['a','e','i','o','u','A','E','I','O','U']);
  while (i < j) {
    while (i < j && !vowels.has(chars[i])) {i++;}
    while (i < j && !vowels.has(chars[j])) {j--;}
    const temp = s[i];
    chars[i] = chars[j];
    chars[j] = temp;
    i++;
    j--;
  }
  return chars.join('');
};


export { reverseVowels };
