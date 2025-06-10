#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>

int main() {
    // 创建 TCP 和 UDP 套接字
    int tcp_sock = socket(AF_INET, SOCK_STREAM, 0);
    if (tcp_sock < 0) {
        perror("TCP socket 创建失败");
        return 1;
    }
    
    int udp_sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (udp_sock < 0) {
        perror("UDP socket 创建失败");
        close(tcp_sock);
        return 1;
    }
    
    printf("TCP socket fd: %d\n", tcp_sock);
    printf("UDP socket fd: %d\n", udp_sock);
    
    // 测试 SO_SNDBUF 选项
    int send_buf_size;
    socklen_t optlen = sizeof(send_buf_size);
    
    // 获取 TCP 套接字的发送缓冲区大小
    if (getsockopt(tcp_sock, SOL_SOCKET, SO_SNDBUF, &send_buf_size, &optlen) < 0) {
        perror("获取 TCP SO_SNDBUF 失败");
    } else {
        printf("TCP socket 发送缓冲区大小: %d 字节\n", send_buf_size);
    }
    
    // 获取 UDP 套接字的发送缓冲区大小
    if (getsockopt(udp_sock, SOL_SOCKET, SO_SNDBUF, &send_buf_size, &optlen) < 0) {
        perror("获取 UDP SO_SNDBUF 失败");
    } else {
        printf("UDP socket 发送缓冲区大小: %d 字节\n", send_buf_size);
    }
    
    // 测试 SO_RCVBUF 选项
    int recv_buf_size;
    optlen = sizeof(recv_buf_size);
    
    // 获取 TCP 套接字的接收缓冲区大小
    if (getsockopt(tcp_sock, SOL_SOCKET, SO_RCVBUF, &recv_buf_size, &optlen) < 0) {
        perror("获取 TCP SO_RCVBUF 失败");
    } else {
        printf("TCP socket 接收缓冲区大小: %d 字节\n", recv_buf_size);
    }
    
    // 获取 UDP 套接字的接收缓冲区大小
    if (getsockopt(udp_sock, SOL_SOCKET, SO_RCVBUF, &recv_buf_size, &optlen) < 0) {
        perror("获取 UDP SO_RCVBUF 失败");
    } else {
        printf("UDP socket 接收缓冲区大小: %d 字节\n", recv_buf_size);
    }
    
    // 测试不支持的选项（应该返回错误）
    int unsupported_opt;
    optlen = sizeof(unsupported_opt);
    
    if (getsockopt(tcp_sock, SOL_SOCKET, SO_REUSEADDR, &unsupported_opt, &optlen) < 0) {
        perror("获取 SO_REUSEADDR 失败（预期会失败）");
    } else {
        printf("SO_REUSEADDR 值: %d（意外成功）\n", unsupported_opt);
    }
    
    // 关闭套接字
    close(tcp_sock);
    close(udp_sock);
    
    return 0;
}