#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <sys/resource.h>
#include <unistd.h>

// 执行一些睡眠的函数
void do_some_sleep(int sec) {
    sleep(sec); // 模拟一些延迟
}

// 执行一些CPU密集型计算的函数
void do_some_work(int iterations) {
    double result = 0.0;
    for (int i = 0; i < iterations; i++) {
        result += i * i / (i + 1.0);
    }
    // 防止编译器优化掉整个循环
    printf("计算结果: %f\n", result);
}

// 打印rusage结构体的内容
void print_rusage(const struct rusage *usage) {
    printf("用户CPU时间: %ld.%06ld 秒\n", 
           usage->ru_utime.tv_sec, usage->ru_utime.tv_usec);
    printf("系统CPU时间: %ld.%06ld 秒\n", 
           usage->ru_stime.tv_sec, usage->ru_stime.tv_usec);
    // printf("最大常驻集大小: %ld\n", usage->ru_maxrss);
    // printf("页面错误数: %ld\n", usage->ru_minflt);
    // printf("主要页面错误数: %ld\n", usage->ru_majflt);
    // printf("自愿上下文切换: %ld\n", usage->ru_nvcsw);
    // printf("非自愿上下文切换: %ld\n", usage->ru_nivcsw);
}

// 计算两个timeval的差值，返回微秒
long timeval_diff_us(struct timeval *start, struct timeval *end) {
    return (end->tv_sec - start->tv_sec) * 1000000 + 
           (end->tv_usec - start->tv_usec);
}

int main() {
    struct rusage usage_start, usage_end;
    
    // 获取初始资源使用情况
    if (getrusage(RUSAGE_SELF, &usage_start) == -1) {
        perror("getrusage");
        return 1;
    }
    
    printf("初始资源使用情况:\n");
    print_rusage(&usage_start);
    
    // 执行一些工作
    //do_some_sleep(3);
    //do_some_work(50000000);
    //clock();// 将clock映射为sleep(2),测试内核内部sleep
    
    
    // 获取结束时的资源使用情况
    if (getrusage(RUSAGE_SELF, &usage_end) == -1) {
        perror("getrusage");
        return 1;
    }
    
    printf("\n最终资源使用情况:\n");
    print_rusage(&usage_end);

    return 0;
}