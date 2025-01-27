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

/* Global arrays to store results (for simplicity in this example). */
static char** gResults = NULL;
static int gResultCount = 0;
static int gResultCapacity = 0;

/* Helper function to add a new string to the global results array. */
static void addToResults(const char* str) {
    if (gResultCount >= gResultCapacity) {
        gResultCapacity = (gResultCapacity == 0) ? 16 : gResultCapacity * 2;
        gResults = (char**)realloc(gResults, gResultCapacity * sizeof(char*));
    }
    gResults[gResultCount] = strdup(str);
    gResultCount++;
}

/*
 * Recursive DFS function to build all valid combinations of parentheses.
 * current: The ongoing string of parentheses.
 * left:    The number of '(' used so far.
 * right:   The number of ')' used so far.
 * n:       The target pairs of parentheses.
 */
static void dfs(char* current, int left, int right, int n) {
    if (left == n && right == n) {
        addToResults(current);
        return;
    }

    if (left < n) {
        int len = (int)strlen(current);
        current[len] = '(';
        current[len + 1] = '\0';
        dfs(current, left + 1, right, n);
        current[len] = '\0';  // backtrack
    }

    if (right < left) {
        int len = (int)strlen(current);
        current[len] = ')';
        current[len + 1] = '\0';
        dfs(current, left, right + 1, n);
        current[len] = '\0';  // backtrack
    }
}

/*
 * Generates all valid combinations of parentheses for the given n.
 * Returns a newly allocated array of strings and sets returnSize to the
 * number of generated combinations.
 */
char** generateParenthesis(int n, int* returnSize) {
    /* Reset global results each time the function is called. */
    gResults = NULL;
    gResultCount = 0;
    gResultCapacity = 0;

    /* Allocate a buffer for building parentheses strings. */
    char* buffer = (char*)malloc((2 * n + 1) * sizeof(char));
    buffer[0] = '\0';

    /* Start DFS. */
    dfs(buffer, 0, 0, n);

    /* Free the temporary buffer used by DFS. */
    free(buffer);

    /* Set the return size for the caller. */
    *returnSize = gResultCount;
    return gResults;
}

int main(void) {
    /* Loop from N=10 to N=16 and measure execution time for each. */
    for (int n = 10; n <= 16; n++) {
        clock_t start = clock();

        int returnSize = 0;
        char** parentheses = generateParenthesis(n, &returnSize);

        clock_t end = clock();
        double elapsed = (double)(end - start) / CLOCKS_PER_SEC;

        printf("N = %d  |  Number of combinations: %d  |  Time taken: %f seconds\n",
               n, returnSize, elapsed);

        /* Optionally, you can print the generated parentheses:
         * for (int i = 0; i < returnSize; i++) {
         *     printf("%s\n", parentheses[i]);
         *     free(parentheses[i]); // Free each string after use
         * }
         *
         * free(parentheses); // Finally, free the array of strings
         *
         * However, note that the global pointers will be overwritten
         * in the next iteration unless carefully managed or reset.
         */
    }
    return 0;
}
