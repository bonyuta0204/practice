#include <unistd.h>
#include <stdio.h>
#include <sys/time.h>
#include <signal.h>

#define MAX_RECORDS 10

// Struct to store timer results
typedef struct {
  long real_elapsed_sec;
  long real_elapsed_usec;
  long virtual_elapsed_sec;
  long virtual_elapsed_usec;
} TimerRecord;

void calculate_elapsed_time(struct itimerval *initial, struct itimerval *current, long *elapsed_sec, long *elapsed_usec) {
  *elapsed_sec = initial->it_value.tv_sec - current->it_value.tv_sec;
  *elapsed_usec = initial->it_value.tv_usec - current->it_value.tv_usec;

  // Adjust for negative microseconds
  if (*elapsed_usec < 0) {
    (*elapsed_sec)--;
    *elapsed_usec += 1000000;
  }
}

int main() {
  struct itimerval real_timer, real_initial, virtual_timer, virtual_initial;
  TimerRecord records[MAX_RECORDS];
  int record_index = 0;

  // Set ITIMER_REAL (real-time timer)
  real_timer.it_interval.tv_sec = 0;
  real_timer.it_interval.tv_usec = 500000; // 500 ms
  real_timer.it_value.tv_sec = 1;          // 2 seconds
  real_timer.it_value.tv_usec = 0;

  if (setitimer(ITIMER_REAL, &real_timer, NULL) < 0) {
    perror("setitimer (ITIMER_REAL)");
    return 1;
  }

  // Save the initial value of ITIMER_REAL
  getitimer(ITIMER_REAL, &real_initial);

  // Set ITIMER_VIRTUAL (CPU time consumed by the process)
  virtual_timer.it_interval.tv_sec = 0;
  virtual_timer.it_interval.tv_usec = 500000; // 500 ms
  virtual_timer.it_value.tv_sec = 2;          // 2 seconds
  virtual_timer.it_value.tv_usec = 0;

  if (setitimer(ITIMER_VIRTUAL, &virtual_timer, NULL) < 0) {
    perror("setitimer (ITIMER_VIRTUAL)");
    return 1;
  }

  // Save the initial value of ITIMER_VIRTUAL
  getitimer(ITIMER_VIRTUAL, &virtual_initial);

  // Step 1: Record initial timers
  records[record_index++] = (TimerRecord){0, 0, 0, 0};

  // Step 2: Sleep for 1 second
  usleep(1000000); // Sleep for 1 second

  getitimer(ITIMER_REAL, &real_timer);
  getitimer(ITIMER_VIRTUAL, &virtual_timer);

  calculate_elapsed_time(&real_initial, &real_timer, &records[record_index].real_elapsed_sec, &records[record_index].real_elapsed_usec);
  calculate_elapsed_time(&virtual_initial, &virtual_timer, &records[record_index].virtual_elapsed_sec, &records[record_index].virtual_elapsed_usec);
  record_index++;

  // Step 3: Perform CPU-bound task
  for (volatile long i = 0; i < 100000000; i++) {} // Simulated workload

  getitimer(ITIMER_REAL, &real_timer);
  getitimer(ITIMER_VIRTUAL, &virtual_timer);

  calculate_elapsed_time(&real_initial, &real_timer, &records[record_index].real_elapsed_sec, &records[record_index].real_elapsed_usec);
  calculate_elapsed_time(&virtual_initial, &virtual_timer, &records[record_index].virtual_elapsed_sec, &records[record_index].virtual_elapsed_usec);
  record_index++;

  // Print all records at the end
  printf("Elapsed Time Records:\n");
  for (int i = 0; i < record_index; i++) {
    printf("Record %d:\n", i);
    printf("  ITIMER_REAL - sec: %ld  usec: %ld\n", records[i].real_elapsed_sec, records[i].real_elapsed_usec);
    printf("  ITIMER_VIRTUAL - sec: %ld  usec: %ld\n", records[i].virtual_elapsed_sec, records[i].virtual_elapsed_usec);
  }

  return 0;
}
