#include <tuple>
#include <algorithm>

#include "ans.hpp"

int Solution::jobScheduling(std::vector<int>& startTime, std::vector<int>& endTime, std::vector<int>& profit) {

  // collect events
  std::vector<std::tuple<int, bool, int>> events;
  for (int i{0}; i < startTime.size(); i++) {
    events.push_back(std::make_tuple(startTime[i], true, i));
    events.push_back(std::make_tuple(endTime[i], false, i));
  }
  std::sort(events.begin(), events.end(), [](auto a, auto b) {
      if (std::get<0>(a) == std::get<0>(b)) { return std::get<1>(a) < std::get<1>(b); }
      return std::get<0>(a) < std::get<0>(b);
  });

  // get best
  int current_max = 0;
  std::vector<int> cache(startTime.size(), 0);
  for (auto event : events) {
    size_t i = std::get<2>(event);
    if (std::get<1>(event)) { // start of job
      cache[i] = current_max + profit[i];
    }
    else { // end of job
      current_max = std::max(current_max, cache[i]);
    }
  }

  return current_max;
}
