#include <sys/time.h>
#include <stdio.h>
#include <unistd.h>
#include <signal.h>
#include <time.h>

struct timespec start_real_time;
struct timespec current_real_time;

struct timespec start_cpu_time;
struct timespec current_cpu_time;

long i = 0;

long elapsed_nsec(struct timespec *t1, struct timespec *t2) {
    // Convert seconds to nanoseconds and add the nanoseconds part
    long nsec1 = t1->tv_sec * 1000000000L + t1->tv_nsec;
    long nsec2 = t2->tv_sec * 1000000000L + t2->tv_nsec;

    // Calculate the difference
    return nsec2 - nsec1;
}

void signal_handler(int signum) {
    clock_gettime(CLOCK_REALTIME,&current_real_time);
    clock_gettime(CLOCK_PROCESS_CPUTIME_ID,&current_cpu_time);

    long real_elapsed = elapsed_nsec(&start_real_time, &current_real_time) / 1000;
    long cpu_elapsed = elapsed_nsec(&start_cpu_time, &current_cpu_time) / 1000;

    printf("Timer called. Value: %10ld  %10ld real usec!  %10ld CPU usec \n", i ,real_elapsed, cpu_elapsed);
    fflush(stdout);
}




int main() {
    /* Register signal handler */
    struct sigaction sa;
    sigemptyset(&sa.sa_mask);
    sa.sa_handler = signal_handler;
    sa.sa_flags = 0;

    clock_gettime(CLOCK_REALTIME,&start_real_time);
    clock_gettime(CLOCK_PROCESS_CPUTIME_ID,&start_cpu_time);

    sigaction(SIGALRM, &sa, NULL);

    /* Configure timer */
    struct itimerval timer;
    timer.it_interval.tv_sec = 0;
    timer.it_interval.tv_usec = 100000; // 100ms interval
    timer.it_value.tv_sec = timer.it_interval.tv_sec;
    timer.it_value.tv_usec = timer.it_interval.tv_usec;

    setitimer(ITIMER_REAL, &timer, NULL);

    /* Wait for signals */
    while (1) {
        i++;
    }
}
