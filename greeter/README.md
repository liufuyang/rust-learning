cargo test -- --nocapture


```rust
// 1. "Give" owner ship to another, you loose ownership.
fn main() {
    let mut owner = String::from("Hello, World");

    {   
        let mut another_owner = owner; // passing ownership
        another_owner = String::from("Hello, New World");
        println!("{}", owner);
        println!("{}", another_owner);
    } 

    println!("{}", owner);
}

// 2. When shared(borrowed), mutate not allowed. Or you can share immutalbe stuff.
fn main() {
    let mut owner = String::from("Hello, World");

    {   
        let mut borrower = &owner; // shared borrow
        *borrower = String::from("Hello, New World");
        println!("{}", owner);
        println!("{}", borrower);
    }

    println!("{}", owner);
}

// 3. When mutable shared (mut borrow), first guy loose access!
fn main() {
    let mut owner = String::from("Hello, World");

    {   
        let mut borrower = &mut owner; // mutable borrow
        *borrower = String::from("Hello, New World");;
        println!("{}", owner);
        println!("{}", borrower);
    }

    println!("{}", owner);
}
```

总结一下:
Rust里ownership"给"完就要不回来了, 给完以后既不能看, 也不能写 (看第一题)
ownership除了"给", 还可以"借"
借分两种
一种是 shared 的借, 借过来以后所有人可以同时看/读, 但不能写或者改.借阅者死掉以后ownership回归, owner可以改写了. owner拿到的东西不变 (第二题)
一种是 mut 的借, 借过来以后只有借阅者有ownership, 其他人不可读不可写, 借阅者可读可写. 借阅者死掉以后ownership回归, owner又可以改写了. 但这个时候owner拿到的东西有可能变化掉了 (第三题)

object 可以想成桌上一盘水果, 桌上有很多位子, 但只有一个位子上有刀叉
ownership 可以想成坐在刀叉上的那个位子, 只要坐上去的人, 可以看水果, 也可以吃水果

* 第一种情况: 可以想象成刀叉位置上完全换了一个人, 新人来, 旧人走掉 (ownership change [mut T])

* 第二种情况: 可以是把桌子位置告诉了另一个人, 他坐过来以后坐在owner旁边的位子上,大家都可以可以看水果, 闻水果, 这时候谁都不能吃. 他走掉以后原来坐在有刀叉位置上的人就又可以吃了. (shared borrow or shared reference [&T])

* 第三种情况: 可以是把桌子位置告诉了另一个人, 他坐到原来owner的位置上(原来的人扭头到一边不能看水果), 新人吃完离开后再让原来的人坐回来. (mut borrow [&mut T])


---

Something to use later:
https://stackoverflow.com/questions/45786717/how-to-implement-hashmap-with-two-keys