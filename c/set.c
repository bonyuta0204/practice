#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>
#include <string.h>
#include <stdio.h>

#define INITIAL_SIZE 8
#define N 1000000 // Number of elements for the benchmark


typedef struct
{
    char **elements;
    size_t capacity;
    size_t size;
} Set;

Set *create_set()
{
    Set *set = malloc(sizeof(Set));
    if (!set)
    {
        perror("Failed to allocate memory for set");
        exit(EXIT_FAILURE);
    }
    set->capacity = INITIAL_SIZE;
    set->size = 0;
    set->elements = calloc(set->capacity, sizeof(char *));

    if (!set->elements)
    {
        perror("Failed to allocate memory for elements");
        exit(EXIT_FAILURE);
    }
    return set;
}


// Hash function for simple strings
size_t hash_string(const char *str, size_t capacity) {
    size_t hash = 0;
    while (*str) {
        hash = (hash * 31 + *str++) % capacity;
    }
    return hash;
}

void resize_set(Set *set) {
    size_t new_capacity = set->capacity * 2;
    char **new_elements = calloc(new_capacity, sizeof(char *));
   if (!new_elements) {
        perror("Failed to allocate memory for resizing");
        exit(EXIT_FAILURE);
    }

    for(size_t i = 0; i < set->capacity; ++i) {
        if(set->elements[i]) {
            size_t new_index = hash_string(set->elements[i], new_capacity);
            while (new_elements[new_index]) {
                new_index = (new_index + 1) % new_capacity;
            }
            new_elements[new_index] = set->elements[i];
        }

    }

    free(set->elements);
    set->elements = new_elements;
    set->capacity = new_capacity;

}

bool add_to_set(Set *set, const char *element) {
  if ((float)set->size / set->capacity > 0.7) {
        resize_set(set);
    }

    size_t index = hash_string(element,set->capacity);

    while(set->elements[index]) {
        if (strcmp(set->elements[index],element) == 0) {
            return false;
        }
        index = (index + 1) % set->capacity;
    }

    set->elements[index] = strdup(element);
    if (!set->elements[index]) {
        perror("Failed to allocate memory for element");
        exit(EXIT_FAILURE);
    }
    set->size++;
    return true;
}


bool remove_from_set(Set *set, const char *element) {
    size_t index = hash_string(element,set->capacity);

    while(set->elements[index]) {
        if (strcmp(set->elements[index],element) == 0) {
            free(set->elements[index]);
            set->elements[index] = NULL;
            set->size--;
            return true;
        }
        index = (index + 1) % set->capacity;
    }
    return false;
}

bool is_in_set(Set *set, const char *element) {
    size_t index = hash_string(element,set->capacity);

    while(set->elements[index]) {
        if (strcmp(set->elements[index],element) == 0) {
            return true;
        }
        index = (index + 1) % set->capacity;
    }
    return false;
}
// Free the set
void free_set(Set *set) {
    for (size_t i = 0; i < set->capacity; ++i) {
        free(set->elements[i]);
    }
    free(set->elements);
    free(set);
}

#define MAX_LEN 12

int main() {
    clock_t start, end;
    double cpu_time_used;

    // Create your set
    Set *custom_set = create_set();

    if (!custom_set) {
        fprintf(stderr, "Failed to create set\n");
        return EXIT_FAILURE;
    }

    // Generate random integers
    char **data = malloc(N * sizeof(char *));

    if (!data) {
        fprintf(stderr, "Failed to allocate memory for data\n");
        return EXIT_FAILURE;
    }
    srand(time(NULL));

    for (int i = 0; i < N; i++) {
     // Allocate memory for each string
        data[i] = malloc(MAX_LEN * sizeof(char));
        if (!data[i]) {
            fprintf(stderr, "Failed to allocate memory for string at index %d\n", i);
            return EXIT_FAILURE;
        }
        int random_number = rand();
        snprintf(data[i], MAX_LEN, "%d", random_number);
    }

    // Benchmark insertion
    start = clock();
    for (int i = 0; i < N; i++) {
        add_to_set(custom_set, data[i]);
    }
    end = clock();
    cpu_time_used = ((double)(end - start)) / CLOCKS_PER_SEC;
    printf("Insertion Time: %.2f seconds\n", cpu_time_used);

    // Benchmark lookup
    start = clock();
    for (int i = 0; i < N; i++) {
        is_in_set(custom_set, data[i]);
    }
    end = clock();
    cpu_time_used = ((double)(end - start)) / CLOCKS_PER_SEC;
    printf("Lookup Time: %.2f seconds\n", cpu_time_used);

    // Benchmark deletion
    start = clock();
    for (int i = 0; i < N; i++) {
        remove_from_set(custom_set, data[i]);
    }
    end = clock();
    cpu_time_used = ((double)(end - start)) / CLOCKS_PER_SEC;
    printf("Deletion Time: %.2f seconds\n", cpu_time_used);

    // Clean up
    free_set(custom_set);
    free(data);

    return 0;
}