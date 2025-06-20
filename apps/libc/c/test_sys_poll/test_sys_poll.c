#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <poll.h>
#include <errno.h>

#define DEFAULT_PORT 1234
#define DEFAULT_IP "127.0.0.1"
#define BUFFER_SIZE 1024

// 客户端使用poll测试
void run_client() {
    int sockfd;
    struct sockaddr_in server_addr;
    char buffer[BUFFER_SIZE] = {0};
    struct pollfd fds[1];
    
    // 创建socket
    if ((sockfd = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        perror("socket创建失败");
        exit(EXIT_FAILURE);
    }
    
    // 设置服务器地址
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(DEFAULT_PORT);
    
    // 转换IP地址
    if (inet_pton(AF_INET, DEFAULT_IP, &server_addr.sin_addr) <= 0) {
        perror("地址无效");
        exit(EXIT_FAILURE);
    }
    
    // 连接服务器
    printf("正在连接服务器 %s:%d...\n", DEFAULT_IP, DEFAULT_PORT);
    if (connect(sockfd, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        perror("连接失败");
        exit(EXIT_FAILURE);
    }
    printf("已连接到服务器\n");
    
    // 设置poll
    fds[0].fd = sockfd;
    fds[0].events = POLLOUT;  // 监控可写事件
    
    // 使用poll等待socket可写
    printf("测试poll，超时=500ms\n");
    int ret = poll(fds, 1, 500);
    
    if (ret < 0) {
        perror("poll失败");
        exit(EXIT_FAILURE);
    } else if (ret == 0) {
        printf("poll超时\n");
    } else {
        printf("poll返回 %d\n", ret);
        
        if (fds[0].revents & POLLOUT) {
            printf("Socket可写\n");
            
            // 发送数据
            const char *message = "你好，服务器!";
            send(sockfd, message, strlen(message), 0);
            printf("已发送: %s\n", message);
            
            int bytes_read = recv(sockfd, buffer, sizeof(buffer) - 1, 0);

            if (bytes_read > 0) {
                buffer[bytes_read] = '\0';
                printf("收到 %d 字节: %s\n", bytes_read, buffer);
            } else {
                printf("接收失败或连接关闭\n");
            }


            fds[0].events = POLLIN;  // 改为监控可读事件
            printf("等待响应，poll超时=500ms\n");
            ret = poll(fds, 1, 50000);
            
            if (ret > 0 && (fds[0].revents & POLLIN)) {
                printf("数据可读\n");
            } else if (ret == 0) {
                printf("在超时时间内未收到响应\n");
            } else {
                perror("读取poll失败");
            }
        }
    }
    
    close(sockfd);
}

int main(int argc, char *argv[]) {
    run_client();
    return 0;
}