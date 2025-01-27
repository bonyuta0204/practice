#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

/*
 * This C program rewrites the "generateParenthesis" algorithm from Python
 * and includes a simple benchmark to measure how long it takes to generate
 * all valid parentheses combinations for each N in a loop.
 *
 * Compilation (e.g., using gcc):
 *    gcc -o generate_parentheses generate_parentheses.c
 * Usage:
 *    ./generate_parentheses
 */

/* Structure to manage dynamic array of strings */
typedef struct {
    char** strings;
    int count;
    int capacity;
} ParenthesesArray;

/* Initialize a new ParenthesesArray */
static void initParenthesesArray(ParenthesesArray* array) {
    array->strings = NULL;
    array->count = 0;
    array->capacity = 0;
}

/* Add a new string to the ParenthesesArray */
static void addToArray(ParenthesesArray* array, const char* str) {
    if (array->count >= array->capacity) {
        array->capacity = (array->capacity == 0) ? 16 : array->capacity * 2;
        array->strings = (char**)realloc(array->strings, array->capacity * sizeof(char*));
    }
    array->strings[array->count] = strdup(str);
    array->count++;
}

/*
 * Recursive DFS function to build all valid combinations of parentheses.
 * array:   The ParenthesesArray to store results
 * current: The ongoing string of parentheses
 * left:    The number of '(' used so far
 * right:   The number of ')' used so far
 * n:       The target pairs of parentheses
 */
static void dfs(ParenthesesArray* array, char* current, int left, int right, int n) {
    if (left == n && right == n) {
        addToArray(array, current);
        return;
    }

    if (left < n) {
        int len = (int)strlen(current);
        current[len] = '(';
        current[len + 1] = '\0';
        dfs(array, current, left + 1, right, n);
        current[len] = '\0';  // backtrack
    }

    if (right < left) {
        int len = (int)strlen(current);
        current[len] = ')';
        current[len + 1] = '\0';
        dfs(array, current, left, right + 1, n);
        current[len] = '\0';  // backtrack
    }
}

/*
 * Generates all valid combinations of parentheses for the given n.
 * Returns a newly allocated array of strings and sets returnSize to the
 * number of generated combinations.
 */
char** generateParenthesis(int n, int* returnSize) {
    ParenthesesArray array;
    initParenthesesArray(&array);

    /* Allocate a buffer for building parentheses strings */
    char* buffer = (char*)malloc((2 * n + 1) * sizeof(char));
    buffer[0] = '\0';

    /* Start DFS */
    dfs(&array, buffer, 0, 0, n);

    /* Free the temporary buffer used by DFS */
    free(buffer);

    /* Set the return size for the caller */
    *returnSize = array.count;
    return array.strings;
}

int main(void) {
    /* Loop from N=10 to N=16 and measure execution time for each */
    for (int n = 10; n <= 16; n++) {
        clock_t start = clock();

        int returnSize = 0;
        char** parentheses = generateParenthesis(n, &returnSize);

        clock_t end = clock();
        double elapsed = (double)(end - start) / CLOCKS_PER_SEC;

        printf("N = %d  |  Number of combinations: %d  |  Time taken: %f seconds\n",
               n, returnSize, elapsed);

        /* Free allocated memory */
        for (int i = 0; i < returnSize; i++) {
            free(parentheses[i]);
        }
        free(parentheses);
    }

    return 0;
}
