#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>

int main() {
    int sockfd;
    struct sockaddr_in server_addr, local_addr, connect_addr;
    socklen_t addr_len = sizeof(struct sockaddr_in);
    
    // 创建 TCP socket
    printf("创建 socket...\n");
    if ((sockfd = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        perror("socket 创建失败");
        exit(EXIT_FAILURE);
    }
    
    // 初始化服务器地址结构
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = htonl(INADDR_ANY); // 任意地址
    server_addr.sin_port = htons(0);  // 让系统自动分配端口
    
    // 绑定 socket 到地址
    printf("绑定 socket...\n");
    if (bind(sockfd, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        perror("bind 失败");
        close(sockfd);
        exit(EXIT_FAILURE);
    }
    
    // 设置连接目标地址 (127.0.0.1:1234)
    memset(&connect_addr, 0, sizeof(connect_addr));
    connect_addr.sin_family = AF_INET;
    connect_addr.sin_addr.s_addr = inet_addr("127.0.0.1");
    connect_addr.sin_port = htons(1234);
    
    // 连接到目标地址
    printf("连接到 127.0.0.1:1234...\n");
    if (connect(sockfd, (struct sockaddr*)&connect_addr, sizeof(connect_addr)) < 0) {
        printf("connect 失败，错误码: %d (%s)，但仍继续测试...\n", errno, strerror(errno));
        // 注意：为了测试，即使连接失败我们也继续执行
    } else {
        printf("连接成功\n");
    }
    
    // 获取 socket 的本地地址
    printf("调用 getsockname...\n");
    if (getsockname(sockfd, (struct sockaddr*)&local_addr, &addr_len) < 0) {
        perror("getsockname 失败");
        close(sockfd);
        exit(EXIT_FAILURE);
    }
    
    // 打印获取到的地址信息
    printf("Socket 本地地址: %s:%d\n", 
           inet_ntoa(local_addr.sin_addr), 
           ntohs(local_addr.sin_port));
           
    // // 测试 UDP socket (UDP 无需连接就能获取本地地址)
    // int udp_sockfd;
    // struct sockaddr_in udp_addr;
    // socklen_t udp_len = sizeof(struct sockaddr_in);
    
    // printf("\n创建 UDP socket...\n");
    // if ((udp_sockfd = socket(AF_INET, SOCK_DGRAM, 0)) < 0) {
    //     perror("UDP socket 创建失败");
    //     close(sockfd);
    //     exit(EXIT_FAILURE);
    // }
    
    // // 初始化 UDP 地址结构
    // memset(&udp_addr, 0, sizeof(udp_addr));
    // udp_addr.sin_family = AF_INET;
    // udp_addr.sin_addr.s_addr = htonl(INADDR_ANY);
    // udp_addr.sin_port = htons(0);  // 自动分配端口
    
    // // 绑定 UDP socket
    // printf("绑定 UDP socket...\n");
    // if (bind(udp_sockfd, (struct sockaddr*)&udp_addr, sizeof(udp_addr)) < 0) {
    //     perror("UDP bind 失败");
    //     close(sockfd);
    //     close(udp_sockfd);
    //     exit(EXIT_FAILURE);
    // }
    
    // // 获取 UDP socket 的本地地址
    // printf("对 UDP socket 调用 getsockname...\n");
    // if (getsockname(udp_sockfd, (struct sockaddr*)&udp_addr, &udp_len) < 0) {
    //     perror("UDP getsockname 失败");
    //     close(sockfd);
    //     close(udp_sockfd);
    //     exit(EXIT_FAILURE);
    // }
    
    // // 打印 UDP 地址信息
    // printf("UDP Socket 绑定到地址: %s:%d\n", 
    //        inet_ntoa(udp_addr.sin_addr), 
    //        ntohs(udp_addr.sin_port));
    
    // 关闭 socket
    close(sockfd);
    //close(udp_sockfd);
    
    printf("测试完成！\n");
    return 0;
}