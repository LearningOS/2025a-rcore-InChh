# lab1

## 实现的功能总结

- 实现了系统调用sys_trace的具体逻辑，具体为：
  - 在TaskManagerInner中添加了新的字段，用于记录系统调用次数信息
  - 在trace_sys_call函数中实现了对系统调用次数的统计
  - 在get_sys_call_count函数中实现了获取系统调用次数的逻辑

## 问答题

1. 问题：正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访
问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ），
描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

答：

使用的SBI及其版本: RustSBI version 0.3.0-alpha.2

出错行为：

- ch2b_bad_address.rs: 访问非法内存地址，报PageFault异常
- ch2b_bad_instruction.rs: 执行特权指令，报IllegalInstruction异常
- ch2b_bad_register.rs: 访问特权寄存器，报IllegalInstruction异常

2. 问题：深入理解 trap.S 中两个函数 __alltraps 和__restore 的作用，并回答如下问题:

    1. L40：刚进入 __restore 时，sp 代表了什么值。请指出__restore 的两种使用情景。

    2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

    ```
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    ```

    3. L50-L56：为何跳过了 x2 和 x4？

    ```
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
       LOAD_GP %n
       .set n, n+1
    .endr
    ```

    4. L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？

    ```
    csrrw sp, sscratch, sp
    ```

    5. __restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

    6. L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？

    ```
    csrrw sp, sscratch, sp
    ```

    7. 从 U 态进入 S 态是哪一条指令发生的？

答：

  1. 刚进入__restore时，sp指向内核栈栈顶，__restore的两种使用情景：1. 运行第一个用户程序时，从S mode切换回U mode；2. 陷入处理完成后，恢复用户程序的上下文并切换到U mode继续运行用户程序
  2. 特殊处理了CSR寄存器sstatus、sepc和sscratch。这些寄存器的值对于进入用户态有重要意义：sstatus用于保存恢复后的特权级，sepc保存了用户程序的返回地址，sscratch保存了用户栈指针
  3. x2是栈指针sp，x4是线程指针tp，这两个寄存器在切换上下文时不需要恢复，因为sp已经通过sscratch恢复，tp目前还没用到
  4. 执行该指令后，sp被更新为用户栈指针，sscratch保存了内核栈指针
  5. 状态切换发生在最后的`sret`指令，执行该指令后，处理器根据sstatus寄存器的值切换到对应的特权级，由于在前面恢复了sstatus寄存器的值（U mode），因此会进入用户态
  6. 在刚进入__alltraps时，sp指向内核栈栈顶，sscratch保存了内核栈指针，执行该指令后，sp被更新为内核栈指针，sscratch保存了用户栈指针
  7. `sret` 指令

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与以下各位就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

  > copilot辅助完成代码编写

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

  > rCore-Tutorial-Book 第三版
  > rCore-Tutorial-Guide 2025S

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
