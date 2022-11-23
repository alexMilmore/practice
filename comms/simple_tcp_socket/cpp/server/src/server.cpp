#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include <iostream>

int main() {
  std::cout << "starting server" << std::endl;

  int server_fd = socket(AF_INET, SOCK_STREAM, 0);
  if (server_fd == -1) { exit(EXIT_FAILURE); }
  int opt = 1;
  if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR | SO_REUSEPORT, &opt, sizeof(opt)))
  { exit(EXIT_FAILURE); }

  struct sockaddr_in address;
  address.sin_family = AF_INET;
  address.sin_addr.s_addr = INADDR_ANY;
  address.sin_port = htons(2000);
}
