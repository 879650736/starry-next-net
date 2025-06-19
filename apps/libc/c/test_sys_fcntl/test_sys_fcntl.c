#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>
#include <assert.h>
#include <errno.h>
#include <sys/socket.h>

void test_f_getfl() {
    printf("Testing F_GETFL...\n");
    
    // Test with stdin
    int flags = fcntl(STDIN_FILENO, F_GETFL);
    assert(flags >= 0);
    printf("stdin flags: %d\n", flags);
    
    // Test with created file (read-write)
    int fd = open("test_file.txt", O_CREAT | O_RDWR, 0644);
    assert(fd >= 0);
    flags = fcntl(fd, F_GETFL);
    assert(flags >= 0);
    printf("file flags (O_RDWR): %d\n", flags);
    close(fd);

    // Test with read-only file
    int fd1 = open("test_file.txt", O_RDONLY, 0644);
    assert(fd1 >= 0);
    flags = fcntl(fd1, F_GETFL);
    assert(flags >= 0);
    printf("file flags (O_RDONLY): %d\n", flags);
    close(fd1);

    // Test with write-only file
    int fd2 = open("test_file.txt", O_WRONLY, 0644);
    assert(fd2 >= 0);
    flags = fcntl(fd2, F_GETFL);
    assert(flags >= 0);
    printf("file flags (O_WRONLY): %d\n", flags);
    close(fd2);
    
    printf("F_GETFL test passed!\n");
}


void test_socket_fcntl() {
    printf("Testing socket fcntl...\n");
    
    // Create socket
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    assert(sockfd >= 0);
    
    // Get socket flags
    int flags = fcntl(sockfd, F_GETFL);
    assert(flags >= 0);
    printf("socket flags: %d\n", flags);
    
    // Set socket to non-blocking
    int result = fcntl(sockfd, F_SETFL, flags | O_NONBLOCK);
    assert(result == 0);
    printf("Set socket to non-blocking: %d\n", result);
    
    // Verify flags changed
    flags = fcntl(sockfd, F_GETFL);
    assert(flags & O_NONBLOCK);
    printf("socket flags after setting non-blocking: %d\n", flags);
    
    close(sockfd);
    printf("Socket fcntl test passed!\n");
}

void test_invalid_fd() {
    printf("Testing invalid fd...\n");
    
    errno = 0; // Clear errno before test
    int result = fcntl(-1, F_GETFL);
    assert(result == -1);
    assert(errno == EBADF);
    printf("Invalid fd test passed (errno: %d)!\n", errno);
}

int main() {
    printf("Starting sys_fcntl tests...\n");
    
    test_f_getfl();
    test_socket_fcntl();
    test_invalid_fd();
    
    printf("All tests passed!\n");
    return 0;
}