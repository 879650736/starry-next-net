// 建议保存为 test_single_socket.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/select.h>
#include <arpa/inet.h>
#include <netinet/in.h>

#define SERVER_IP "127.0.0.1"
#define PORT 1234

int main() {
    int server_fd, client_fd, new_socket;
    struct sockaddr_in address;
    int addrlen = sizeof(address);
    char buffer[1024] = {0};
    fd_set read_fds;
    struct timeval tv;
    
    // 创建服务器 socket
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        perror("server socket failed");
        exit(EXIT_FAILURE);
    }
    printf("Server socket created: fd=%d\n", server_fd);
    
    // 设置服务器地址
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = inet_addr(SERVER_IP);
    address.sin_port = htons(PORT);
    
    // 绑定服务器 socket
    if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0) {
        perror("bind failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }
    printf("Server socket bound to port 8080\n");
    
    // 监听连接
    if (listen(server_fd, 1) < 0) {
        perror("listen failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }
    printf("Server listening for connections\n");
    
    // 创建客户端 socket
    if ((client_fd = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        perror("client socket failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }
    printf("Client socket created: fd=%d\n", client_fd);
    
    // 连接到服务器
    if (connect(client_fd, (struct sockaddr *)&address, sizeof(address)) < 0) {
        perror("connect failed");
        close(client_fd);
        close(server_fd);
        exit(EXIT_FAILURE);
    }
    printf("Client connected to server\n");
    
    // 接受客户端连接
    if ((new_socket = accept(server_fd, (struct sockaddr *)&address, (socklen_t*)&addrlen)) < 0) {
        perror("accept failed");
        close(client_fd);
        close(server_fd);
        exit(EXIT_FAILURE);
    }
    printf("Server accepted connection: new_socket=%d\n", new_socket);
    
    // 从客户端发送数据
    const char *message = "Hello from client";
    printf("Client sending message: %s\n", message);
    send(client_fd, message, strlen(message), 0);
    printf("Client message sent\n");
    
    // 设置 select 等待服务器端 socket 可读
    FD_ZERO(&read_fds);
    FD_SET(new_socket, &read_fds);
    
    // 设置超时为 3 秒
    tv.tv_sec = 3;
    tv.tv_usec = 0;
    
    printf("Waiting for data with select...\n");
    int activity = select(new_socket + 1, &read_fds, NULL, NULL, &tv);
    
    if (activity < 0) {
        perror("select error");
    } else if (activity == 0) {
        printf("Select timeout!\n");
    } else {
        if (FD_ISSET(new_socket, &read_fds)) {
            // 读取数据
            int valread = read(new_socket, buffer, 1024);
            printf("Server received %d bytes: %s\n", valread, buffer);
        }
    }
    
    // 清理
    close(new_socket);
    close(client_fd);
    close(server_fd);
    
    return 0;
}