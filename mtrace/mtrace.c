#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ptrace.h>
#include <sys/reg.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

int do_child(int argc, char **argv);
int do_trace(pid_t child);

int main(int argc, char **argv) {
  printf("argc: %d\n", argc);

  for (int i = 0; i < argc; i++) {
    char *arg = argv[i];

    printf("arg%d: %s\n", i, arg);
  }

  if (argc < 2) {
    fprintf(stderr, "Usage: %s prog args\n", argv[0]);
    exit(1);
  }

  pid_t child = fork();
  if (child == 0) {
    return do_child(argc - 1, argv + 1);
  } else {
    do_trace(child);
  }
}

int do_child(int argc, char **argv) {
  char *args[argc + 1];
  memcpy(args, argv, argc * sizeof(char *));
  args[argc] = NULL;

  ptrace(PTRACE_TRACEME);
  kill(getpid(), SIGSTOP);

  return execvp(args[0], args);
};

int wait_for_syscall(pid_t child);

int do_trace(pid_t child) {
  int status, syscall, retval;
  waitpid(child, &status, 0);

  return 0;
}
