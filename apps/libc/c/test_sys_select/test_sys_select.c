#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/select.h>
#include <sys/time.h>
#include <fcntl.h>

int main() {
    fd_set readfds;
    struct timeval tv;
    int ret;
    char buffer[256];
    
    // 清空 fd_set 结构
    FD_ZERO(&readfds);
    
    // 添加标准输入到 fd_set
    FD_SET(STDIN_FILENO, &readfds);
    
    // 设置超时为 5 秒
    tv.tv_sec = 5;
    tv.tv_usec = 0;
    
    printf("等待标准输入上的数据，或者 5 秒超时...\n");
    
    // 调用 select，监控标准输入
    ret = select(STDIN_FILENO + 1, &readfds, NULL, NULL, &tv);
    
    if (ret == -1) {
        perror("select 错误");
        return EXIT_FAILURE;
    } else if (ret == 0) {
        printf("超时，没有输入！\n");
    } else {
        printf("select 返回: %d\n", ret);
    }
    
    return EXIT_SUCCESS;
}