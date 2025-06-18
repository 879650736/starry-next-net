#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <sys/select.h>

#define PORT 1234
#define SERVER_IP "127.0.0.1"

int main() {
    int sock_fd;
    struct sockaddr_in server_addr;
    fd_set read_fds, write_fds;
    struct timeval tv;
    char buffer[1024];
    
    // 创建套接字
    sock_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (sock_fd < 0) {
        perror("套接字创建失败");
        exit(EXIT_FAILURE);
    }
    printf("套接字创建成功\n");
    
    // 配置服务器地址
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = inet_addr(SERVER_IP);
    server_addr.sin_port = htons(PORT);
    
    // 连接到服务器
    printf("尝试连接到 %s:%d...\n", SERVER_IP, PORT);
    if (connect(sock_fd, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        perror("连接失败");
        close(sock_fd);
        exit(EXIT_FAILURE);
    }
    printf("已连接到服务器\n");
    
    // 初始化文件描述符集合
    FD_ZERO(&read_fds);
    FD_SET(sock_fd, &read_fds);
    
    FD_ZERO(&write_fds);
    FD_SET(sock_fd, &write_fds);
    
    // 设置超时
    tv.tv_sec = 0;
    tv.tv_usec = 0;
    
    printf("等待select()...\n");
    int activity = select(sock_fd + 1, &read_fds, &write_fds, NULL, &tv);
    
    if (activity < 0) {
        perror("select错误");
    }
    
    if (activity == 0) {
        printf("超时！0秒内没有活动\n");
    }
    
    // 检查是否可读
    if (FD_ISSET(sock_fd, &read_fds)) {
        memset(buffer, 0, sizeof(buffer));
        int bytes_read = read(sock_fd, buffer, sizeof(buffer) - 1);
        
        if (bytes_read <= 0) {
            if (bytes_read == 0) {
                printf("服务器关闭了连接\n");
            } else {
                perror("读取错误");
            }
        }
        
        printf("从服务器收到: %s\n", buffer);
    }
    
    // 检查是否可写
    if (FD_ISSET(sock_fd, &write_fds)) {
        printf("套接字可写\n");
        // 此处可以发送更多数据，但为了简单起见，我们只发送一次
        sleep(1);  // 避免过于频繁地打印消息
    }
    
    close(sock_fd);
    return 0;
}