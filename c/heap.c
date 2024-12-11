#include <stdio.h>

void swap(int* a, int* b) {
    int tmp = *a;
    *a = *b;
    *b = tmp;
}

int heappush(int elem, int array[], int n) {
    if (n >= 100) {
        printf("Heap overflow\n");
        return n;
    }

    array[n] = elem;
    int current_idx = n;

    while (current_idx > 0) {
        int parent_idx = (current_idx - 1) / 2;
        if (array[current_idx] < array[parent_idx]) {
            swap(&array[current_idx], &array[parent_idx]);
            current_idx = parent_idx;
        } else {
            break;
        }
    }
    return n + 1;
}

int heappop(int array[], int n) {
    if (n == 0) {
        printf("Heap underflow\n");
        return -1; // エラー値
    }

    int elem = array[0];
    array[0] = array[n - 1];
    array[n - 1] = 0;
    int current_idx = 0;
    int new_size = n - 1;

    while (current_idx * 2 + 1 < new_size) {
        int left_child_idx = current_idx * 2 + 1;
        int right_child_idx = left_child_idx + 1;

        int smaller_idx = left_child_idx;
        if (right_child_idx < new_size && array[right_child_idx] < array[left_child_idx]) {
            smaller_idx = right_child_idx;
        }

        if (array[smaller_idx] < array[current_idx]) {
            swap(&array[smaller_idx], &array[current_idx]);
            current_idx = smaller_idx;
        } else {
            break;
        }
    }
    return elem;
}

void heapify(int array[], int n) {
    for (int i = (n / 2) - 1; i >= 0; i--) {
        int current_idx = i;

        while (current_idx * 2 + 1 < n) {
            int left_child_idx = current_idx * 2 + 1;
            int right_child_idx = left_child_idx + 1;

            int smaller_idx = left_child_idx;
            if (right_child_idx < n && array[right_child_idx] < array[left_child_idx]) {
                smaller_idx = right_child_idx;
            }

            if (array[smaller_idx] < array[current_idx]) {
                swap(&array[smaller_idx], &array[current_idx]);
                current_idx = smaller_idx;
            } else {
                break;
            }
        }
    }
}

int main() {
    int heap[100];  // Array to store heap
    int size = 0;   // Current number of elements in the heap

    printf("Adding elements to the heap:\n");
    size = heappush(10, heap, size);
    size = heappush(3, heap, size);
    size = heappush(20, heap, size);
    size = heappush(7, heap, size);
    size = heappush(15, heap, size);

    printf("Heap after insertions:\n");
    for (int i = 0; i < size; i++) {
        printf("%d ", heap[i]);
    }
    printf("\n");

    printf("Popping the top element:\n");
    int popped = heappop(heap, size);
    size--;
    printf("Popped element: %d\n", popped);

    printf("Heap after popping:\n");
    for (int i = 0; i < size; i++) {
        printf("%d ", heap[i]);
    }
    printf("\n");

    printf("Heapifying the array:\n");
    heapify(heap, size);

    printf("Heap after heapify operation:\n");
    for (int i = 0; i < size; i++) {
        printf("%d ", heap[i]);
    }
    printf("\n");

    return 0;
}
