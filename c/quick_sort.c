#include <stdio.h>

void swap(int* a, int* b) {
    int tmp = *a;
    *a = *b;
    *b = tmp;
}

int partition(int* array, int n, int pivot) {
    int left = 0, right = n - 1;

    while (left <= right) {
        while (left < n && array[left] < pivot) {
            left++;
        }
        while (right >= 0 && array[right] > pivot) {
            right--;
        }
        if (left <= right) {
            swap(&array[left], &array[right]);
            left++;
            right--;
        }
    }
    return left; // Return the partition index
}

void quick_sort(int* array, int n) {
    if (n <= 1) {
        return; // Base case
    }

    int pivot = array[n / 2];
    int k = partition(array, n, pivot);

    // Avoid sorting invalid ranges
    if (k > 0) {
        quick_sort(array, k); // Sort left partition
    }
    if (k < n) {
        quick_sort(array + k, n - k); // Sort right partition
    }
}

int main() {
    int a[10] = {4, 9, 0, 5, 6, 7, 2, 3, 1, 8};
    quick_sort(a, 10);

    for (int i = 0; i < 10; i++) {
        printf("%d ", a[i]);
    }
    printf("\n");

    return 0;
}

