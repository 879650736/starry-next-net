#include <stdio.h>
#include <unistd.h>
#include <errno.h>
#include <string.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <sys/time.h>
#include <fcntl.h>

int main(void)
{
    struct sockaddr_in sa = { .sin_family = AF_INET };
    int s, c, t;
    char buf[100];

    // UDP 测试
    s = socket(PF_INET, SOCK_DGRAM, IPPROTO_UDP);
    if (s < 0) {
        perror("UDP socket creation failed");
        return 1;
    }
    
    if (bind(s, (void *)&sa, sizeof sa) != 0) {
        perror("UDP bind failed");
        close(s);
        return 1;
    }
    
    getsockname(s, (void *)&sa, (socklen_t[]){sizeof sa});
    
    struct timeval tv = {.tv_usec=1};
    setsockopt(s, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(struct timeval));

    c = socket(PF_INET, SOCK_DGRAM, IPPROTO_UDP);
    if (c < 0) {
        perror("UDP client socket creation failed");
        close(s);
        return 1;
    }
    
    sa.sin_addr.s_addr = htonl(0x7f000001);
    sendto(c, "x", 1, 0, (void *)&sa, sizeof sa);
    recvfrom(s, buf, sizeof buf, 0, (void *)&sa, (socklen_t[]){sizeof sa});
    
    if (buf[0] != 'x') {
        printf("Data mismatch, expected 'x', got '%c'\n", buf[0]);
    }

    close(c);
    close(s);

    // TCP 测试
    memset(&sa, 0, sizeof sa);
    sa.sin_family = AF_INET;
    
    s = socket(PF_INET, SOCK_STREAM|SOCK_CLOEXEC, IPPROTO_TCP);
    if (s < 0) {
        perror("TCP socket creation failed");
        return 1;
    }
    
    if (!(fcntl(s, F_GETFD) & FD_CLOEXEC)) {
        printf("SOCK_CLOEXEC did not work\n");
    }
    
    bind(s, (void *)&sa, sizeof sa);
    getsockname(s, (void *)&sa, (socklen_t[]){sizeof sa});
    sa.sin_addr.s_addr = htonl(0x7f000001);

    listen(s, 1);

    c = socket(PF_INET, SOCK_STREAM|SOCK_NONBLOCK, IPPROTO_TCP);
    if (c < 0) {
        perror("TCP client socket creation failed");
        close(s);
        return 1;
    }
    
    if (!(fcntl(c, F_GETFL) & O_NONBLOCK)) {
        printf("SOCK_NONBLOCK did not work\n");
    }

    int ret = connect(c, (void *)&sa, sizeof sa);
    if (ret != 0 && errno != EINPROGRESS) {
        perror("Connect failed unexpectedly");
        close(c);
        close(s);
        return 1;
    }

    t = accept(s, (void *)&sa, &(socklen_t){sizeof sa});
    if (t < 0) {
        perror("Accept failed");
        close(c);
        close(s);
        return 1;
    }

    close(t);
    close(c);
    close(s);
    
    printf("Socket test completed successfully\n");
    return 0;
}