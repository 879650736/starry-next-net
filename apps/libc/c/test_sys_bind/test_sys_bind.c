#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>

void test_tcp_bind() {
    printf("=== Testing TCP bind ===\n");
    
    // 创建 TCP socket
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) {
        perror("socket creation failed");
        return;
    }
    printf("TCP socket created successfully, fd: %d\n", sockfd);
    
    // 设置地址结构
    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = inet_addr("127.0.0.1");  
    addr.sin_port = htons(1234);        // 端口 1234

    printf("Binding to address: %s:%d\n", 
           inet_ntoa(addr.sin_addr), 
           ntohs(addr.sin_port));
    // 绑定地址
    if (bind(sockfd, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
        perror("bind failed");
        close(sockfd);
        return;
    }
    printf("TCP socket bound to 127.0.0.1:8080 successfully\n");

    // 获取绑定后的地址信息
    struct sockaddr_in bound_addr;
    socklen_t addr_len = sizeof(bound_addr);
    // if (getsockname(sockfd, (struct sockaddr*)&bound_addr, &addr_len) == 0) {
    //     printf("Bound address: %s:%d\n", 
    //            inet_ntoa(bound_addr.sin_addr), 
    //            ntohs(bound_addr.sin_port));
    // }
    
    close(sockfd);
    printf("TCP test completed\n\n");
}

void test_udp_bind() {
    printf("=== Testing UDP bind ===\n");
    
    // 创建 UDP socket
    int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (sockfd < 0) {
        perror("socket creation failed");
        return;
    }
    printf("UDP socket created successfully, fd: %d\n", sockfd);
    
    // 设置地址结构
    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = inet_addr("127.0.0.1");  // localhost
    addr.sin_port = htons(9090);                    // 端口 9090
    
    // 绑定地址
    if (bind(sockfd, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
        perror("bind failed");
        close(sockfd);
        return;
    }
    printf("UDP socket bound to 127.0.0.1:9090 successfully\n");
    
    // 获取绑定后的地址信息
    struct sockaddr_in bound_addr;
    socklen_t addr_len = sizeof(bound_addr);
    // if (getsockname(sockfd, (struct sockaddr*)&bound_addr, &addr_len) == 0) {
    //     printf("Bound address: %s:%d\n", 
    //            inet_ntoa(bound_addr.sin_addr), 
    //            ntohs(bound_addr.sin_port));
    // }
    
    close(sockfd);
    printf("UDP test completed\n\n");
}

void test_bind_errors() {
    printf("=== Testing bind error cases ===\n");
    
    // 测试无效文件描述符
    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = INADDR_ANY;
    addr.sin_port = htons(8081);
    
    if (bind(-1, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
        printf("Expected error for invalid fd: %s\n", strerror(errno));
    }
    
    // 测试重复绑定
    int sockfd1 = socket(AF_INET, SOCK_STREAM, 0);
    int sockfd2 = socket(AF_INET, SOCK_STREAM, 0);
    
    if (sockfd1 >= 0 && sockfd2 >= 0) {
        addr.sin_port = htons(8082);
        
        // 第一个 socket 绑定成功
        if (bind(sockfd1, (struct sockaddr*)&addr, sizeof(addr)) == 0) {
            printf("First socket bound to port 8082\n");
            
            // 第二个 socket 绑定同一端口应该失败
            if (bind(sockfd2, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
                printf("Expected error for duplicate bind: %s\n", strerror(errno));
            }
        }
        
        close(sockfd1);
        close(sockfd2);
    }
    
    printf("Error test completed\n\n");
}

void test_multiple_binds() {
    printf("=== Testing multiple binds ===\n");
    
    const int num_sockets = 3;
    int sockfds[num_sockets];
    int ports[] = {8083, 8084, 8085};
    
    // 创建多个 socket 并绑定不同端口
    for (int i = 0; i < num_sockets; i++) {
        sockfds[i] = socket(AF_INET, SOCK_STREAM, 0);
        if (sockfds[i] < 0) {
            perror("socket creation failed");
            continue;
        }
        
        struct sockaddr_in addr;
        memset(&addr, 0, sizeof(addr));
        addr.sin_family = AF_INET;
        addr.sin_addr.s_addr = INADDR_ANY;
        addr.sin_port = htons(ports[i]);
        
        if (bind(sockfds[i], (struct sockaddr*)&addr, sizeof(addr)) == 0) {
            printf("Socket %d bound to port %d\n", sockfds[i], ports[i]);
        } else {
            perror("bind failed");
        }
    }
    
    // 清理
    for (int i = 0; i < num_sockets; i++) {
        if (sockfds[i] >= 0) {
            close(sockfds[i]);
        }
    }
    
    printf("Multiple bind test completed\n\n");
}

int main() {
    printf("Starting bind system call tests...\n\n");
    
    test_tcp_bind();
    //test_udp_bind();
    //test_bind_errors();
    // test_multiple_binds();
    
    printf("All tests completed!\n");
    return 0;
}