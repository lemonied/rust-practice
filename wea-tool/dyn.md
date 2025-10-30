在 Rust 中，`dyn` 是 **dynamic dispatch**（动态派发）的缩写，用于表示**trait object（特征对象）**。  

简单来说，当你用 `dyn Trait` 的形式时，这表示一个在运行时才知道具体类型的对象（指向实现了该 trait 的某个具体类型的指针），而编译器会生成虚表（vtable）来支持方法调用。

---

## 语法
```rust
fn example(x: &dyn MyTrait) {
    x.do_something();
}
```
这里 `&dyn MyTrait` 表示“一个引用，指向某个实现了 `MyTrait` 的类型，但具体是什么类型在运行时才确定”。

---

## 为什么需要 `dyn`
Rust 有两种 trait 使用方式：

1. **静态派发**（Static Dispatch）：  
   使用泛型，例如：
   ```rust
   fn foo<T: MyTrait>(x: T) {
       x.do_something();
   }
   ```
   编译器会在编译期生成针对具体类型的代码（单态化，monomorphization），无需虚表调用，性能高，但会增加代码体积。

2. **动态派发**（Dynamic Dispatch）：  
   使用 trait object，例如：
   ```rust
   fn bar(x: &dyn MyTrait) {
       x.do_something();
   }
   ```
   在运行时通过 vtable（虚方法表）查找具体实现，性能稍低，但允许在一个集合中混合不同类型（只要它们实现了同一个 trait），并且可以减少泛型带来的代码膨胀。

---

## 注意事项
- `dyn Trait` 必须通过引用或智能指针来使用，比如 `&dyn Trait` 或 `Box<dyn Trait>`，不能直接声明一个裸值（因为大小不固定）。
- Trait 要作为 trait object 使用，必须是 **object safe**，即其中的方法签名不能包含泛型类型参数，也不能有 `Self` 作为返回值等一些限制。

---

### 示例
```rust
trait Draw {
    fn draw(&self);
}

struct Circle;
impl Draw for Circle {
    fn draw(&self) {
        println!("Circle");
    }
}

struct Square;
impl Draw for Square {
    fn draw(&self) {
        println!("Square");
    }
}

fn main() {
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];

    for shape in shapes.iter() {
        shape.draw();
    }
}
```
这里 `Vec<Box<dyn Draw>>` 里可以存放不同具体类型的对象，但它们必须实现 `Draw` trait，  
调用 `draw()` 时会通过动态派发调用正确的实现。

---

✅ 总结：
- `dyn` 表示动态派发（trait object）。
- 运行时确定类型并通过虚表调用方法。
- 与泛型的静态派发不同，`dyn` 适合在集合、插件系统等需要混合不同类型的场景。

---
