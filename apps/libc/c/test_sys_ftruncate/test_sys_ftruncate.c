#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <string.h>
#include <assert.h>
#include <sys/types.h>
#include <sys/stat.h>

#define TEST_FILE "test_ftruncate.txt"
#define INITIAL_CONTENT "This is a test file for ftruncate system call testing."
#define INITIAL_SIZE 51  // Length of INITIAL_CONTENT
#define NEW_SIZE 20      // Size to truncate to

int main() {
    int fd;
    struct stat st;
    
    // Create a test file
    fd = open(TEST_FILE, O_RDWR | O_CREAT | O_TRUNC, 0644);
    if (fd == -1) {
        perror("Failed to create test file");
        return 1;
    }
    
    // Write initial content
    if (write(fd, INITIAL_CONTENT, INITIAL_SIZE) != INITIAL_SIZE) {
        perror("Failed to write to test file");
        close(fd);
        return 1;
    }
    
    // Check the initial file size
    if (fstat(fd, &st) == -1) {
        perror("Failed to get file stats");
        close(fd);
        return 1;
    }
    
    printf("Initial file size: %ld bytes\n", (long)st.st_size);
    assert(st.st_size == INITIAL_SIZE);
    
    // Truncate the file to NEW_SIZE bytes
    if (ftruncate(fd, NEW_SIZE) == -1) {
        perror("ftruncate failed");
        close(fd);
        return 1;
    }
    
    // Check the new file size
    if (fstat(fd, &st) == -1) {
        perror("Failed to get file stats after truncate");
        close(fd);
        return 1;
    }
    
    printf("File size after truncate to %d bytes: %ld bytes\n", NEW_SIZE, (long)st.st_size);
    assert(st.st_size == NEW_SIZE);
    
    // Read the truncated content to verify
    char buffer[INITIAL_SIZE] = {0};
    lseek(fd, 0, SEEK_SET);  // Rewind to the beginning of the file
    
    ssize_t bytes_read = read(fd, buffer, sizeof(buffer));
    if (bytes_read == -1) {
        perror("Failed to read from file");
        close(fd);
        return 1;
    }
    
    printf("Read %zd bytes from truncated file\n", bytes_read);
    printf("Content after truncate: \"%s\"\n", buffer);
    assert(bytes_read == NEW_SIZE);
    assert(strncmp(buffer, INITIAL_CONTENT, NEW_SIZE) == 0);
    
    // Test expanding the file size
    long expand_size = INITIAL_SIZE + 10;
    
    if (ftruncate(fd, expand_size) == -1) {
        perror("ftruncate (expand) failed");
        close(fd);
        return 1;
    }
    
    // Check the expanded file size
    if (fstat(fd, &st) == -1) {
        perror("Failed to get file stats after expand");
        close(fd);
        return 1;
    }
    
    printf("File size after expand to %ld bytes: %ld bytes\n", expand_size, (long)st.st_size);
    assert(st.st_size == expand_size);
    
    // Close and clean up
    close(fd);
    unlink(TEST_FILE);
    
    printf("All tests passed!\n");
    return 0;
}