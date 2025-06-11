#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <errno.h>

void test_socket_option(int sock, int level, int optname, const char* opt_name) {
    int value;
    socklen_t optlen = sizeof(value);
    
    if (getsockopt(sock, level, optname, &value, &optlen) < 0) {
        printf("获取 %s 失败: %s\n", opt_name, strerror(errno));
    } else {
        printf("%s: %d 字节\n", opt_name, value);
    }
}

int main() {
    printf("=== 套接字选项测试 ===\n");
    
    // 创建TCP和UDP套接字
    int tcp_sock = socket(AF_INET, SOCK_STREAM, 0);
    if (tcp_sock < 0) {
        perror("TCP socket创建失败");
        return 1;
    }
    
    int udp_sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (udp_sock < 0) {
        perror("UDP socket创建失败");
        close(tcp_sock);
        return 1;
    }
    
    printf("\n=== TCP套接字(fd=%d)测试 ===\n", tcp_sock);
    test_socket_option(tcp_sock, SOL_SOCKET, SO_SNDBUF, "TCP发送缓冲区大小");
    test_socket_option(tcp_sock, SOL_SOCKET, SO_RCVBUF, "TCP接收缓冲区大小");
    test_socket_option(tcp_sock, IPPROTO_TCP, TCP_MAXSEG, "TCP最大分段大小");
    
    printf("\n=== UDP套接字(fd=%d)测试 ===\n", udp_sock);
    test_socket_option(udp_sock, SOL_SOCKET, SO_SNDBUF, "UDP发送缓冲区大小");
    test_socket_option(udp_sock, SOL_SOCKET, SO_RCVBUF, "UDP接收缓冲区大小");
    
    // 测试在UDP套接字上获取TCP特定选项（应该失败）
    printf("\n=== 错误测试 ===\n");
    test_socket_option(udp_sock, IPPROTO_TCP, TCP_MAXSEG, "UDP上获取TCP_MAXSEG");
    
    // 测试不支持的选项
    test_socket_option(tcp_sock, SOL_SOCKET, SO_REUSEADDR, "不支持的SO_REUSEADDR");
    
    // 测试无效的文件描述符
    int invalid_fd = 9999;
    test_socket_option(invalid_fd, SOL_SOCKET, SO_SNDBUF, "无效文件描述符");
    
    // 关闭套接字
    close(tcp_sock);
    close(udp_sock);
    
    return 0;
}