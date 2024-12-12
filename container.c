#define _GNU_SOURCE
#include <sched.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>

#define STACK_SIZE (1024 * 1024)

static char child_stack[STACK_SIZE];

int child_main(void *arg) {
    printf("Inside container: PID = %d\n", getpid());
    execlp("/bin/bash", "bash", NULL);  // Replace with desired program
    perror("execlp");
    return 1;
}

int main() {
    printf("Parent: PID = %d\n", getpid());

    pid_t child_pid = clone(child_main, child_stack + STACK_SIZE,
                            SIGCHLD | CLONE_NEWPID, NULL);

    if (child_pid == -1) {
        perror("clone");
        exit(1);
    }

    printf("Child PID: %d\n", child_pid);
    waitpid(child_pid, NULL, 0);
    return 0;
}

