#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <errno.h>
#include <string.h>

int main() {
    printf("Testing IPv4 socket creation...\n\n");

    // 测试 IPv4 TCP socket
    printf("1. Creating IPv4 TCP socket...\n");
    int tcp_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (tcp_fd >= 0) {
        printf("   ✓ SUCCESS: TCP socket fd = %d\n", tcp_fd);
        close(tcp_fd);
    } else {
        printf("   ✗ FAILED: errno = %d (%s)\n", errno, strerror(errno));
    }

    // 测试 IPv4 UDP socket
    printf("2. Creating IPv4 UDP socket...\n");
    int udp_fd = socket(AF_INET, SOCK_DGRAM, 0);
    if (udp_fd >= 0) {
        printf("   ✓ SUCCESS: UDP socket fd = %d\n", udp_fd);
        close(udp_fd);
    } else {
        printf("   ✗ FAILED: errno = %d (%s)\n", errno, strerror(errno));
    }

    // 测试无效地址族
    printf("3. Testing invalid domain (AF_UNIX = 1)...\n");
    int invalid_fd = socket(1, SOCK_STREAM, 0);
    if (invalid_fd < 0) {
        printf("   ✓ SUCCESS: Invalid domain rejected, errno = %d (%s)\n", errno, strerror(errno));
    } else {
        printf("   ✗ FAILED: Should reject invalid domain\n");
        close(invalid_fd);
    }

    // 测试无效socket类型
    printf("4. Testing invalid socket type (SOCK_RAW = 3)...\n");
    int invalid_type_fd = socket(AF_INET, 3, 0);
    if (invalid_type_fd < 0) {
        printf("   ✓ SUCCESS: Invalid type rejected, errno = %d (%s)\n", errno, strerror(errno));
    } else {
        printf("   ✗ FAILED: Should reject invalid type\n");
        close(invalid_type_fd);
    }

    printf("\nTest completed.\n");
    return 0;
}