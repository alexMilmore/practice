function exist(board: string[][], word: string): boolean {
  for (let i=0; i < board.length; i++) {
    for (let j=0; j < board[0].length; j++) {
      if (dfs(board, i, j, word, 0)) { return true; }
    }
  }
  return false;
};

function dfs(board: string[][], i: number, j: number, word: string, word_index: number): boolean {
  if (board[i][j] != word[word_index]) { return false; };
  word_index += 1;
  if (word_index > word.length-1) { return true; };

  let check = false;
  const temp = board[i][j];
  board[i][j] = "\n";
  if (i > 0) { check ||= dfs(board, i-1, j, word, word_index); };
  if (j > 0) { check ||= dfs(board, i, j-1, word, word_index); };
  if (i < board.length-1)    { check ||= dfs(board, i+1, j, word, word_index); };
  if (j < board[0].length-1) { check ||= dfs(board, i, j+1, word, word_index); };
  board[i][j] = temp;
  return check;
}

export { exist };
