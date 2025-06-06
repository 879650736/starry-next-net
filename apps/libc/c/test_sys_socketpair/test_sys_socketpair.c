#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <errno.h>
#include <string.h>

int main() {
    printf("Testing socketpair system call...\n\n");

    // 测试 IPv4 TCP socketpair
    printf("1. Creating IPv4 TCP socketpair...\n");
    int tcp_sv[2];
    int ret = socketpair(AF_INET, SOCK_STREAM, 0, tcp_sv);
    if (ret == 0) {
        printf("   ✓ SUCCESS: TCP socketpair fds = %d, %d\n", tcp_sv[0], tcp_sv[1]);
        close(tcp_sv[0]);
        close(tcp_sv[1]);
    } else {
        printf("   ✗ FAILED: errno = %d (%s)\n", errno, strerror(errno));
    }

    // 测试 IPv4 UDP socketpair
    printf("2. Creating IPv4 UDP socketpair...\n");
    int udp_sv[2];
    ret = socketpair(AF_INET, SOCK_DGRAM, 0, udp_sv);
    if (ret == 0) {
        printf("   ✓ SUCCESS: UDP socketpair fds = %d, %d\n", udp_sv[0], udp_sv[1]);
        close(udp_sv[0]);
        close(udp_sv[1]);
    } else {
        printf("   ✗ FAILED: errno = %d (%s)\n", errno, strerror(errno));
    }

    // 测试无效地址族
    printf("3. Testing invalid domain (AF_UNIX = 1)...\n");
    int invalid_sv[2];
    ret = socketpair(1, SOCK_STREAM, 0, invalid_sv);
    if (ret < 0) {
        printf("   ✓ SUCCESS: Invalid domain rejected, errno = %d (%s)\n", errno, strerror(errno));
    } else {
        printf("   ✗ FAILED: Should reject invalid domain\n");
        close(invalid_sv[0]);
        close(invalid_sv[1]);
    }

    // 测试无效socket类型
    printf("4. Testing invalid socket type (SOCK_RAW = 3)...\n");
    int invalid_type_sv[2];
    ret = socketpair(AF_INET, 3, 0, invalid_type_sv);
    if (ret < 0) {
        printf("   ✓ SUCCESS: Invalid type rejected, errno = %d (%s)\n", errno, strerror(errno));
    } else {
        printf("   ✗ FAILED: Should reject invalid type\n");
        close(invalid_type_sv[0]);
        close(invalid_type_sv[1]);
    }

    // // 测试空指针参数
    // printf("5. Testing NULL pointer for socket array...\n");
    // ret = socketpair(AF_INET, SOCK_STREAM, 0, NULL);
    // if (ret < 0) {
    //     printf("   ✓ SUCCESS: NULL pointer rejected, errno = %d (%s)\n", errno, strerror(errno));
    // } else {
    //     printf("   ✗ FAILED: Should reject NULL pointer\n");
    // }

    // // 测试通信功能（如果创建成功）
    // printf("6. Testing communication between socketpair...\n");
    // int comm_sv[2];
    // ret = socketpair(AF_INET, SOCK_STREAM, 0, comm_sv);
    // if (ret == 0) {
    //     char send_msg[] = "Hello from socket 0";
    //     char recv_msg[64] = {0};
        
    //     // 从 socket 0 发送到 socket 1
    //     ssize_t sent = send(comm_sv[0], send_msg, strlen(send_msg), 0);
    //     if (sent > 0) {
    //         ssize_t received = recv(comm_sv[1], recv_msg, sizeof(recv_msg) - 1, 0);
    //         if (received > 0) {
    //             printf("   ✓ SUCCESS: Communication test passed\n");
    //             printf("     Sent: '%s'\n", send_msg);
    //             printf("     Received: '%s'\n", recv_msg);
    //         } else {
    //             printf("   ✗ FAILED: recv failed, errno = %d (%s)\n", errno, strerror(errno));
    //         }
    //     } else {
    //         printf("   ✗ FAILED: send failed, errno = %d (%s)\n", errno, strerror(errno));
    //     }
        
    //     close(comm_sv[0]);
    //     close(comm_sv[1]);
    // } else {
    //     printf("   ✗ FAILED: Cannot create socketpair for communication test\n");
    // }

    printf("\nTest completed.\n");
    return 0;
}