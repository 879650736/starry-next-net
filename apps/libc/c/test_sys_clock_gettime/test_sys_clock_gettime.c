#include <stdio.h>
#include <time.h>   // For clock_gettime, CLOCK_PROCESS_CPUTIME_ID, CLOCK_MONOTONIC
#include <unistd.h> // For sleep

int main() {
    struct timespec cpu_start, cpu_end;
    struct timespec wall_start, wall_end;
    long long cpu_time_ns, wall_time_ns;

    printf("--- CLOCK_PROCESS_CPUTIME_ID Test ---\n\n");

    // 1. 获取开始时的 CPU 时间和墙上时间
    if (clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &cpu_start) == -1) {
        perror("Error getting CLOCK_PROCESS_CPUTIME_ID start");
        return 1;
    }
    if (clock_gettime(CLOCK_MONOTONIC, &wall_start) == -1) {
        perror("Error getting CLOCK_MONOTONIC start");
        return 1;
    }

    printf("Starting CPU-bound work...\n");

    // 2. 执行一个 CPU 密集型循环
    // 这个循环会消耗大量的CPU时间
    volatile long long i; // 使用 volatile 防止编译器优化掉循环
    for (i = 0; i < 2000000000LL; ++i) { // 20亿次迭代，根据你的CPU速度调整
        // 空操作，只是为了消耗CPU
    }

    printf("CPU-bound work finished.\n");

    // 3. 模拟一个睡眠/等待操作
    printf("Sleeping for 1 second (this should NOT count towards CPU time)...\n");
    sleep(1); // 进程进入睡眠状态，不消耗CPU

    printf("Sleep finished.\n\n");

    // 4. 获取结束时的 CPU 时间和墙上时间
    if (clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &cpu_end) == -1) {
        perror("Error getting CLOCK_PROCESS_CPUTIME_ID end");
        return 1;
    }
    if (clock_gettime(CLOCK_MONOTONIC, &wall_end) == -1) {
        perror("Error getting CLOCK_MONOTONIC end");
        return 1;
    }

    // 5. 计算并打印结果
    cpu_time_ns = (cpu_end.tv_sec - cpu_start.tv_sec) * 1000000000LL +
                  (cpu_end.tv_nsec - cpu_start.tv_nsec);

    wall_time_ns = (wall_end.tv_sec - wall_start.tv_sec) * 1000000000LL +
                   (wall_end.tv_nsec - wall_start.tv_nsec);

    printf("Results:\n");
    printf("  Total CPU time consumed by process: %.6f seconds\n", (double)cpu_time_ns / 1000000000.0);
    printf("  Total Wall (real) time elapsed:     %.6f seconds\n", (double)wall_time_ns / 1000000000.0);

    printf("\n--- End of Test ---\n");

    return 0;
}
