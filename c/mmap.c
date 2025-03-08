#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/mman.h>
#include <fcntl.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>

// Test much larger file sizes
#define SMALL_SIZE 10000000      // 10 million integers (40MB)
#define MEDIUM_SIZE 50000000     // 50 million integers (200MB)
#define LARGE_SIZE 100000000     // 100 million integers (400MB)

// Function to test read performance with standard I/O
double test_standard_io(const char* filename, int count) {
    FILE* file = fopen(filename, "r");
    if (file == NULL) {
        perror("Failed to open file for standard I/O");
        return -1.0;
    }
    
    int* buf = malloc(count * sizeof(int));
    if (buf == NULL) {
        perror("Memory allocation failed");
        fclose(file);
        return -1.0;
    }
    
    clock_t start = clock();
    
    size_t read_count = fread(buf, sizeof(int), count, file);
    if (read_count != count) {
        fprintf(stderr, "Warning: Only read %zu of %d items\n", read_count, count);
    }
    
    // Verify data (optional - just to ensure equivalent work to mmap version)
    volatile int sum = 0;  // Use volatile to prevent optimization
    for (int i = 0; i < read_count; i++) {
        sum += buf[i];
    }
    
    clock_t end = clock();
    double elapsed = (double)(end - start) / CLOCKS_PER_SEC * 1000.0; // Convert to milliseconds
    
    free(buf);
    fclose(file);
    
    return elapsed;
}

// Function to test read performance with mmap
double test_mmap(const char* filename, int count) {
    int fd = open(filename, O_RDONLY);
    if (fd == -1) {
        perror("Failed to open file for mmap");
        return -1.0;
    }
    
    // Get file size
    struct stat sb;
    if (fstat(fd, &sb) == -1) {
        perror("Failed to get file size");
        close(fd);
        return -1.0;
    }
    
    // Map the file
    clock_t start = clock();
    
    int* mm = mmap(NULL, sb.st_size, PROT_READ, MAP_PRIVATE, fd, 0);
    if (mm == MAP_FAILED) {
        perror("mmap failed");
        close(fd);
        return -1.0;
    }
    
    // Read the data (accessing each element to make a fair comparison)
    volatile int sum = 0;  // Use volatile to prevent optimization
    for (int i = 0; i < count; i++) {
        sum += mm[i];
    }
    
    clock_t end = clock();
    double elapsed = (double)(end - start) / CLOCKS_PER_SEC * 1000.0; // Convert to milliseconds
    
    // Unmap the file
    if (munmap(mm, sb.st_size) == -1) {
        perror("munmap failed");
    }
    close(fd);
    
    return elapsed;
}

// Create a test file with specified number of integers
void create_test_file(const char* filename, int count) {
    printf("Creating test file with %d integers (%d MB)...\n", 
           count, (int)(count * sizeof(int) / (1024 * 1024)));
    
    FILE* file = fopen(filename, "w");
    if (file == NULL) {
        perror("Failed to create test file");
        exit(1);
    }
    
    // Use a smaller buffer to avoid excessive memory usage
    const int BUFFER_SIZE = 1000000; // 1 million integers at a time
    int* buf = malloc(BUFFER_SIZE * sizeof(int));
    if (buf == NULL) {
        perror("Memory allocation failed");
        fclose(file);
        exit(1);
    }
    
    // Write in chunks to handle very large files
    int remaining = count;
    int value = 0;
    
    while (remaining > 0) {
        int chunk_size = (remaining < BUFFER_SIZE) ? remaining : BUFFER_SIZE;
        
        for (int i = 0; i < chunk_size; i++) {
            buf[i] = value++;
        }
        
        size_t written = fwrite(buf, sizeof(int), chunk_size, file);
        if (written != chunk_size) {
            fprintf(stderr, "Warning: Only wrote %zu of %d items\n", written, chunk_size);
            break;
        }
        
        remaining -= chunk_size;
    }
    
    free(buf);
    fclose(file);
    printf("File creation complete.\n");
}

int main() {
    const char* filename = "mmap_test.dat";
    int sizes[] = {SMALL_SIZE, MEDIUM_SIZE, LARGE_SIZE};
    
    printf("File Size      | Standard I/O (ms) | mmap (ms) | Speedup\n");
    printf("---------------|-------------------|-----------|--------\n");
    
    for (int i = 0; i < sizeof(sizes)/sizeof(sizes[0]); i++) {
        int count = sizes[i];
        
        // Create test file
        create_test_file(filename, count);
        
        // Test standard I/O
        double standard_time = test_standard_io(filename, count);
        
        // Test mmap
        double mmap_time = test_mmap(filename, count);
        
        // Calculate speedup
        double speedup = standard_time / mmap_time;
        
        // Print results
        printf("%7d MB    | %17.2f | %9.2f | %.2fx\n", 
               (int)(count * sizeof(int) / (1024 * 1024)), 
               standard_time, mmap_time, speedup);
    }
    
    // Clean up
    unlink(filename);
    
    return 0;
}