在 Rust 中，`&raw` 是 **原始引用（Raw Reference）** 的语法，是在 **Rust 1.70** 引入的一个新特性（[RFC #2582](https://rust-lang.github.io/rfcs/2582-raw-reference.html)），用来更安全、明确地创建原始指针（raw pointer）。

---

## 1. 背景

在传统 Rust 里，如果你要获取一个原始指针（`*const T` 或 `*mut T`），做法通常是：

```rust
let x = 42;
let ptr = &x as *const i32;       // immutable raw pointer
let mut y = 5;
let ptr_mut = &mut y as *mut i32; // mutable raw pointer
```

这虽然可以用，但存在一些问题：
- `&x as *const T` 看起来像普通引用的强制转换，不够直观。
- 可读性和可维护性较差，特别是对于不太熟悉原始指针语法的人。
- 可能触发意外的引用生命周期或借用检查问题。

---

## 2. 新的 `&raw` 语法

`&raw` 是专门用来创建原始指针的语法：
- `&raw const expr` 生成一个 `*const T` 原始指针
- `&raw mut expr` 生成一个 `*mut T` 原始指针

例子：

```rust
fn main() {
    let x = 42;
    let ptr_const = &raw const x; // 原始不可变指针
    let mut y = 5;
    let ptr_mut = &raw mut y;     // 原始可变指针

    // 使用原始指针需要 unsafe
    unsafe {
        println!("ptr_const points to: {}", *ptr_const);
        *ptr_mut = 10;
        println!("y is now {}", y);
    }
}
```

---

## 3. 原始引用特点

- **不会触发借用检查**：原始引用（raw pointer）绕过 Rust 的借用规则，你可以同时拥有多个可变原始指针，甚至可以把它们指向无效/释放的内存（危险！）。
- **不自动保证安全**：解引用原始指针要用 `unsafe`。
- **不绑定生命周期**：与常规引用不同，原始指针没有 Rust 编译器强制管理的生命周期。
- **更显式**：`&raw` 比老的 `&expr as *const T` 更易读，也能提醒开发者这是低级、潜在危险的操作。

---

## 4. 用途

`&raw` 常见于：
- 与 C 或其他语言互操作（FFI）
- 底层内存操作（如实现操作系统内核、驱动）
- 做数据结构内部优化（比如自己管理内存布局）
- 实现不使用 Rust 传统借用检查的高性能代码片段

---

## 5. 总结对比表

| 语法               | 生成类型   | 借用检查 | 是否安全解引用 |
| ------------------ | ---------- | -------- | -------------- |
| `&x`               | `&T`       | ✅        | ✅              |
| `&mut x`           | `&mut T`   | ✅        | ✅              |
| `&x as *const T`   | `*const T` | ❌        | ❌（unsafe）    |
| `&mut x as *mut T` | `*mut T`   | ❌        | ❌（unsafe）    |
| `&raw const x`     | `*const T` | ❌        | ❌（unsafe）    |
| `&raw mut x`       | `*mut T`   | ❌        | ❌（unsafe）    |

---

✅ **一句话总结**：  
`&raw` 是 Rust 1.70 引入的显式创建原始指针的语法，更清晰地表示你在绕过借用检查进入底层内存操作的领域。

---
