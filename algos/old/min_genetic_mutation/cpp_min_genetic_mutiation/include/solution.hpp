#include<string>
#include<vector>
#include<queue>
#include<unordered_set>
#include<iostream>

class Solution {
public:
    int minMutation(std::string start, std::string end, std::vector<std::string>& bank) {
      std::queue<std::tuple<std::string, int>> queue;
      queue.push(std::make_tuple(start, 0));
      std::unordered_set<std::string> visited;

      while (queue.size() != 0) {
        if (std::get<0>(queue.front()) == end) { return std::get<1>(queue.front()); }
        if (visited.count(std::get<0>(queue.front())) != 0) {
          queue.pop();
          continue;
        }
        visited.insert(std::get<0>(queue.front()));
        for (auto gene : bank) {
          if (hamDist(std::get<0>(queue.front()), gene) == 1) {
            queue.push(std::make_tuple(gene, std::get<1>(queue.front())+1));
          }
        }
        queue.pop();
      }
      return -1;
    }

    int hamDist(const std::string& a, const std::string& b) {
      int count{0};
      for (int i{0}; i < a.size(); i++) {
        if (a[i] != b[i]) { count += 1; }
      }
      return count;
    }
};
