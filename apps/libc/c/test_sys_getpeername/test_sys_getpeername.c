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
    struct sockaddr_in server_addr, local_addr, peer_addr;
    socklen_t addr_len = sizeof(struct sockaddr_in);
    
    // 创建 TCP socket
    printf("创建 socket...\n");
    if ((sockfd = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        perror("socket 创建失败");
        exit(EXIT_FAILURE);
    }
    
    // 初始化本地地址结构
    memset(&local_addr, 0, sizeof(local_addr));
    local_addr.sin_family = AF_INET;
    local_addr.sin_addr.s_addr = htonl(INADDR_ANY); // 任意地址
    local_addr.sin_port = htons(0);  // 让系统自动分配端口
    
    // 绑定 socket 到地址
    printf("绑定 socket...\n");
    if (bind(sockfd, (struct sockaddr*)&local_addr, sizeof(local_addr)) < 0) {
        perror("bind 失败");
        close(sockfd);
        exit(EXIT_FAILURE);
    }
    
    // 设置连接目标地址 (127.0.0.1:1234)
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = inet_addr("127.0.0.1");
    server_addr.sin_port = htons(1234);
    
    // 连接到目标地址
    printf("连接到 127.0.0.1:1234...\n");
    if (connect(sockfd, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        printf("connect 失败，错误码: %d (%s)，但仍继续测试...\n", errno, strerror(errno));
    } else {
        printf("连接成功\n");
        
        // 获取对端地址
        printf("调用 getpeername...\n");
        if (getpeername(sockfd, (struct sockaddr*)&peer_addr, &addr_len) < 0) {
            printf("getpeername 失败，错误码: %d (%s)\n", errno, strerror(errno));
        } else {
            // 打印获取到的对端地址信息
            printf("Socket 对端地址: %s:%d\n", 
                   inet_ntoa(peer_addr.sin_addr), 
                   ntohs(peer_addr.sin_port));
        }
    }
    
    // 测试未连接套接字的情况
    int sockfd2;
    printf("\n测试未连接套接字...\n");
    if ((sockfd2 = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        perror("socket2 创建失败");
    } else {
        if (getpeername(sockfd2, (struct sockaddr*)&peer_addr, &addr_len) < 0) {
            printf("未连接套接字 getpeername 失败，错误码: %d (%s) [期望返回 ENOTCONN]\n", 
                   errno, strerror(errno));
        } else {
            printf("未连接套接字 getpeername 成功（意外结果）\n");
        }
        close(sockfd2);
    }

    // 关闭主套接字
    close(sockfd);
    
    printf("测试完成！\n");
    return 0;
}