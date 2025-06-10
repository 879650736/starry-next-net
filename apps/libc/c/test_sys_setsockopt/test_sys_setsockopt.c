#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <errno.h>

int main() {
    int sockfd;
    int flag;
    socklen_t len = sizeof(flag);
    
    // 创建TCP套接字
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) {
        perror("创建套接字失败");
        exit(EXIT_FAILURE);
    }
    printf("套接字创建成功，fd = %d\n", sockfd);
    
    // 获取TCP_NODELAY的初始值
    if (getsockopt(sockfd, IPPROTO_TCP, TCP_NODELAY, &flag, &len) < 0) {
        printf("获取TCP_NODELAY失败: %s\n", strerror(errno));
    } else {
        printf("TCP_NODELAY的初始值: %d\n", flag);
    }
    
    // 设置TCP_NODELAY为1（禁用Nagle算法）
    flag = 1;
    if (setsockopt(sockfd, IPPROTO_TCP, TCP_NODELAY, &flag, sizeof(flag)) < 0) {
        printf("设置TCP_NODELAY失败: %s\n", strerror(errno));
    } else {
        printf("TCP_NODELAY设置为1成功\n");
    }
    
    // 再次获取TCP_NODELAY值以验证设置
    flag = 0; // 重置变量
    if (getsockopt(sockfd, IPPROTO_TCP, TCP_NODELAY, &flag, &len) < 0) {
        printf("获取TCP_NODELAY失败: %s\n", strerror(errno));
    } else {
        printf("设置后TCP_NODELAY的值: %d\n", flag);
        if (flag) {
            printf("Nagle算法已被禁用\n");
        } else {
            printf("Nagle算法仍然启用，设置可能未生效\n");
        }
    }
    
    // 关闭套接字
    close(sockfd);
    printf("套接字已关闭\n");
    
    return 0;
}