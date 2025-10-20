# Lab3

## 实现的功能总结

- 修改了系统调用sys_get_time和sys_mmap、sys_munmap以适应新的进程结构，相比上一章，具体修改了：
  - 获取当前TCB的方式
- 实现了sys_spawn和sys_set_priority系统调用

## 问答题

> stride 算法原理非常简单，但是有一个比较大的问题。例如两个 pass = 10 的进程，使用 8bit 无符号整形储存 stride， p1.stride = 255, p2.stride = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。
>
> - 实际情况是轮到 p1 执行吗？为什么？
>
>   不是，因为p1.stride加上pass = 10后会溢出为10，而p2.stride会溢出为4，之后选择stride最小的进程仍然为p2
>
> 我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明， **在不考虑溢出的情况下** , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2。
>
> - 为什么？尝试简单说明（不要求严格证明）。
>
>   由于P.pass = BigStride / P.priority，而P.priority >=2，则最大步长MAX_PASS <= BigStride / 2，又STRIDE_MAX – STRIDE_MIN <= MAX_PASS，因此STRIDE_MAX – STRIDE_MIN <= BigStride / 2。
>
> - 已知以上结论，**考虑溢出的情况下**，可以为 Stride 设计特别的比较器，让 BinaryHeap<Stride> 的 pop 方法能返回真正最小的 Stride。补全下列代码中的 `partial_cmp` 函数，假设两个 Stride 永远不会相等。
>
> ```rust
> use core::cmp::Ordering;
> 
> struct Stride(u64);
> 
> impl PartialOrd for Stride {
>     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
>         let half = BIG_STRIDE / 2;
>         if self.0.wrapping_sub(other.0) > half {
>             Some(Ordering::Less)
>         } else {
>             Some(Ordering::Greater)
>         }
>     }
> }
> 
> impl PartialEq for Stride {
>     fn eq(&self, other: &Self) -> bool {
>         false
>     }
> }
> ```
>
> TIPS: 使用 8 bits 存储 stride, BigStride = 255, 则: `(125 < 255) == false`, `(129 < 255) == true`.