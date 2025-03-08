#include <stdio.h>
#include <netdb.h>
#include <sys/types.h>
#include <sys/socket.h>

int main(){


  char* host = "www.google.com";

  struct addrinfo *result, *rp;;

  int ret = getaddrinfo(host, "80", NULL, &result);

  if (ret < 0) {
    perror("Failed to getaddrinfo");
  }

  struct sockaddr_in *ipv4;

  for (rp = result; rp != NULL; rp = rp->ai_next) {
    if (rp->ai_family == AF_INET) {
      ipv4 = rp->ai_addr;
    }
  }


  int sock = socket(PF_INET, SOCK_STREAM, 0);
  ssize_t add_len = sizeof(ipv4);

  ret = connect(sock, ipv4, add_len);

  if (ret < 0) {
    perror("failed to open socket");
  }

  char buf[1000];

  recv(sock,buf,10,0);

};
