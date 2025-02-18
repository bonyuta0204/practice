#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>


int main() {
  char* path = "/dev/tty";
  FILE* file = fopen(path,"w");

  if (!file) {
    perror("Cannot open file");
    exit(1);
  };

  fprintf(file, "Hello World!\n");

}
