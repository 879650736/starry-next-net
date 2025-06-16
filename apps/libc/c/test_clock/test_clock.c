#include <stdio.h> // C 语言的输入输出库
#include <time.h>  // 包含 clock() 和 CLOCKS_PER_SEC

int main() {
    clock_t clast; // 声明一个变量来存储开始时间

    clast = clock(); // 记录开始时间
    printf("clast: %ld\n", clast); // 输出开始时间的值

    printf("开始测量代码执行时间...\n");
    // 这里是你要测量执行时间的代码块
    for (long long i = 0; i < 1000000000; ++i) {
        // 做一些耗时操作，例如一个空循环
        // 实际上，一个空循环可能会被编译器优化掉，
        // 如果需要确保耗时，可以做一些简单的计算，例如：
        volatile int temp = 0;
        temp++;
    }

    clock_t cend = clock(); // 记录结束时间
    printf("cend: %ld\n", cend); // 输出结束时间的值

    // 计算时间差
    // 注意：在 C 语言中，类型转换使用 (type) value
    double time_taken = (double)(cend - clast) / CLOCKS_PER_SEC;

    // 使用 printf 进行输出
    printf("代码执行时间: %f 秒\n", time_taken);

    return 0; // C 语言中 main 函数返回 0 表示成功
}
