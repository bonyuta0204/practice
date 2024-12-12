#define _GNU_SOURCE
#include <sched.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/mount.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#define STACK_SIZE (1024 * 1024)

static char child_stack[STACK_SIZE];

int child_main(void *arg) {
  printf("Inside container: PID = %d\n", getpid());
  char s[100];
  getcwd(s, sizeof(s));
  printf("working directory: %s\n", s);

  if (mount(NULL, "/", NULL, MS_SLAVE | MS_REC, NULL) == -1) {
    perror("mount propagation setup");
    exit(1);
  }

  if (mount("proc", "/proc", "proc", MS_NOSUID | MS_NOEXEC | MS_NODEV, NULL) ==
      -1) {
    perror("mount /proc");
    exit(1);
  }

  // mount current directory to /app
  if (mount(s, "/app", NULL, MS_BIND, NULL) == -1) {

    perror("mount directory");
    exit(1);
  }

  execlp("/bin/bash", "bash", NULL); // Replace with desired program
  perror("execlp");
  return 1;
}

int main() {
  printf("Parent: PID = %d\n", getpid());

  pid_t child_pid = clone(child_main, child_stack + STACK_SIZE,
                          SIGCHLD | CLONE_NEWPID | CLONE_NEWNS, NULL);

  if (child_pid == -1) {
    perror("clone");
    exit(1);
  }

  printf("Child PID: %d\n", child_pid);
  waitpid(child_pid, NULL, 0);
  return 0;
}
