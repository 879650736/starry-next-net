#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>

// 定义常量 - 目标服务器地址和端口
#define SERVER_IP "127.0.0.1"
#define SERVER_PORT 1234

int main() {
    printf("测试 sys_connect 连接到 %s:%d\n", SERVER_IP, SERVER_PORT);

    // 1. 创建套接字
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) {
        perror("socket");
        printf("socket() 失败，错误码: %d\n", errno);
        return 1;
    }
    printf("套接字创建成功，文件描述符: %d\n", sockfd);

    // 2. 准备服务器地址结构
    struct sockaddr_in server_addr;
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(SERVER_PORT);
    
    if (inet_pton(AF_INET, SERVER_IP, &server_addr.sin_addr) <= 0) {
        perror("inet_pton");
        printf("inet_pton() 失败，错误码: %d\n", errno);
        close(sockfd);
        return 1;
    }

    
    // 3. 连接到服务器
    printf("正在调用 connect()...\n");
    int result = connect(sockfd, (struct sockaddr*)&server_addr, sizeof(server_addr));
    if (result < 0) {
        perror("connect");
        printf("connect() 失败，错误码: %d\n", errno);
        close(sockfd);
        return 1;
    }

    printf("成功连接到 %s:%d\n", SERVER_IP, SERVER_PORT);
    printf("sys_connect 测试通过！\n");

    // 4. 尝试发送一些数据
    const char *message = "Hello from client!";
    if (write(sockfd, message, strlen(message)) < 0) {
        perror("write");
        printf("数据发送失败\n");
    } else {
        printf("数据发送成功: %s\n", message);
    }

    // 5. 尝试接收响应
    char buffer[1024];
    int bytes_read = read(sockfd, buffer, sizeof(buffer) - 1);
    if (bytes_read > 0) {
        buffer[bytes_read] = '\0';
        printf("收到响应: %s\n", buffer);
    } else if (bytes_read == 0) {
        printf("服务器关闭了连接\n");
    } else {
        perror("read");
    }

    // 6. 关闭套接字
    close(sockfd);
    return 0;
}