fn main() {
    // 可变变量
    // mut是mutable的意思
    let mut a = 1;
    println!("{}", a);
    a = 2;
    println!("{}", a);

    // 不可变变量
    let b = 1;
    println!("{}", b);
    // b = 2; //error

    // 变量遮蔽
    let b = 2;
    println!("{}", b);

    // 常量
    const DEFAULT_COUNT: i32 = 100;
    println!("{}", DEFAULT_COUNT);

    // 元组
    let tp = (1, 2, "three");
    let (x, y, z) = tp;
    println!("{}, {}, {}", x, y, z);

    // 数组
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("current month is: {}", months[8]);

    let a = [3; 5]; // [3,3,3,3,3]
    println!("{}", a[4]);
    //println!("{}", a[5]);   // 数组越界，实测cargo check不报错，cargo build报错、、、

    another_func();
    another_func2(123);

    // 最后一句不写分号，则当做结果返回（写分号则不行，写分号就是语句，因为语句不会返回任何值）
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    // 函数定义
    fn another_func() {
        println!("another_func!")
    }

    fn another_func2(value: i32) {
        println!("The value is {}!", value)
    }

    //  函数返回值
    fn five() -> i32 {
        5
    }

    fn six() -> char {
        return '6';
    }

    let f = five();
    let s = six();

    println!("The value of f is: {}", f);
    println!("The value of s is: {}", s);

    // 分支
    let x = 1;
    if x == 1 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let x = 6;
    if x == 1 {
        println!("The value is {}", 1);
    } else if x == 2 {
        println!("The value is {}", 2);
    } else if x == 6 {
        println!("The value is {}", 6);
    } else {
        println!("The value is {}", "unknown");
    }

    // if 的返回值可以赋值给变量
    let res = if x == 6 { 6 } else { 5 };

    println!("{}", res);

    // 无限循环，在按下control + c才会停止运行
    // loop {
    //     println!("{}", "hi");
    // }

    // 也可以用break来中断
    let mut i = 0;
    println!("loop start !");
    loop {
        // rust 不支持 自增操作符++ 和 自减操作符--
        i += 1;
        println!("{}", i);
        if i >= 10 {
            break;
        };
    }

    // 当有多个循环嵌套时，可以用label来跳出指定的循环，label名必须以'开头
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // break还可以返回值
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // while循环
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    //使用for...in对数组进行遍历
    let arr = [0, 1, 2, 3, 4, 5];
    for item in arr {
        println!("{}", item);
    }

    // 使用range，range用来生成从一个数字开始到另一个数字之前
    //结束的所有数字的序列。（不包括最后一位）
    for i in 6..11 {
        println!("{}", i);
    }

    // 所有权（系统）是 Rust 最为与众不同的特性
    // 它让 Rust 无需垃圾回收(GC)即可保障内存安全。

    // 栈和堆都是代码在运行时可供使用的内存
    // 栈中的所有数据都必须占用已知且固定的大小。
    //在编译时大小未知或大小可能变化的数据，要改为存储在堆上。

    // Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
    // 值在任一时刻有且只有一个所有者。
    // 当所有者（变量）离开作用域，这个值将被丢弃。

    // 要理解所有权，先学习一下rust的字符串类型以及内存分配

    //let mut s = "123";  // 类型&str，字符串字面量被直接硬编码进最终的可执行文件中
    //s.push_str(", world!"); // 字符串字面量是不能被改变的

    let mut s = String::from("hello"); // 类型是String，它是可变的，需要在运行时动态分配内存，所以需要存在堆中
                                       // 分配内存通过 String::from 来实现
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`

    // 当我们将String使用完成后，需要将内存返回给系统
    // 在拥有GC的语言中，会自动跟踪清理不再使用的内存，进行返还
    // 没有GC，就需要我们手动进行返还，如果不返回，就会造成内存浪费
    // 如果过早回收了，将会出现无效变量。如果重复回收，这也是个 bug。

    // rust使用所有权来在没有GC的情况下避免手动进行垃圾回收

    // 所有权的规则
    // 1. 每个值都有一个变量，这个变量是该值的所有者
    // 2. 每个值同时只能有一个所有者
    // 3. 当所有者超出作用域，该值会被清除
    {
        let s = String::from("hello");
        println!("{}", s);
        // s 是该String值的所有者
        // 当该代码块结束时，s不可用，该值也会被清除，返还内存
        // 每个对象都实现了一个trait, 即Drop（如果你熟悉 Java 可以把trait理解为“接口”），
        // Drop 包含一个方法drop(), 在任何对象离开作用域的时候，它的drop()会被自动调用，从而释放自身内存。
    }

    // 所有权可以转移
    {
        let s = String::from("hello world");
        let str = s;
        //println!("{}", s);
        // 上一句不可行，
        // 提示 move occurs because `s` has type `String`, which does not implement the `Copy` trait
        // 因为 对象的所有者被转移给了str， s 失去了所有权，所以不能再使用这个值
        println!("{}", str);

        // 转移规则不适用于基础类型（实际上是实现了Copy这个trait的类型）
        let s = 123;
        let s1 = s;
        println!("{}, {}", s, s1); //这里的操作是可行的
        // 在基本类型赋值的时候, 拷贝了一份原内存作为新变量，而不是转移所有权。也就是说本例中 s1 是一个独立的变量，拥有独立的内存，
        // 而不是从s处获得。s也得以保留了对应值的内存，因而可以继续使用。
        // 具体来说是实现了Copy这个trait的类型，基本类型 Rust 已经内置实现了，
        // 也就是说，我们完全可以自己为String类型实现Copy ，从而在字符串对象赋值的时候，拷贝而不转移。

        // 一般来说，任何简单标量的类型都是可Copy的
        // 所有的正数，浮点，布尔，char，所有字段都可Copy的Tuple
        // 任何需要分配内存的类型都不是可copy的

        // 一个变量的所有权总是遵循同样的模式
        // 把一个值赋给其它变量时就会发生转移
        // 当一个堆数据离开作用域时，它的值就会被drop函数请阿里
        // 除非数据的所有权移动到另一个变量之上
    }

    {
        let s = String::from("abc");
        fn console_str(a: String) {
            println!("{}", a);
        }
        console_str(s);
        //println!("{}", s);  // 报错，因为传给函数，所有权就转移了，外部的s没有了所有权

        let i = 32;
        fn console_int(i: i32) {
            println!("{}", i);
        }
        console_int(i);
        println!("{}", i);  // 依然有效，因为i是i32类型，实现了copy
    }

    // 返回值会发生所有权的转移
    {
        // 一个简单的函数，将传进去的字符串直接返回
        fn move_ownership(s: String) -> String {
            s
        }
        let i = String::from("a string");
        let i2 = move_ownership(i);
        println!("{}", i2); // i转移给函数的s，s返回再转移给i2
    }

    // 但所有权的转移意味着一些问题的出现
    // 比如我们将某个变量传递给某个函数，这个变量就失去了对应值的所有权
    // 这显然在很多场景下是不能接受的，
    // 如果要保留所有权，就需要传进去，再返回回来重新转移到新变量使用
    // 这未免太过麻烦

    // 所以就有了借用的机制
    // 借用就是把东西借给你，但是你并没有获得这个东西的所有权
    // 在rust中的表现就是把引用传递给某个变量，此时就是借用，不会发生转移
    // 比如将引用传递给一个函数
    {
        fn console_str(a: &String) {
            println!("{}", a);
        }
        // let mut s = String::from("hello");
        let s = String::from("hello");
        let a= &s;  // 借用而不是转移
        println!("{}", s);  // 可以使用
        // a = "world!";    // 借用者不能修改借用的值
        // let b = s;   // 借出时不能转移所有权
        // s = String::from("world!");  // 借出时也不能修改值
        println!("{}", a);
        console_str(&s);    // 借用
        let b = s;  // 此时借用已经结束，可以转移
        println!("{}", b);
    }

    // 在被借用期间，拥有者不允许修改变量，或者转移所有权！
    // 这看似是一个礼节问题，但实则是为内存安全考虑，修改值将导致这些借用的值与本身不一致，引发逻辑错误，
    // 转移所有权必将导致借用失效，因此，这不被允许！

    // 但有时我们确实需要修改值，这就需要可变借用
    // 可变借用能对可变（mut）数据进行修改
    {
        let mut s = String::from("hello_");
        // &mut 表示可变借用
        fn push(s: &mut String) {
            s.push_str("world!");
        }
        push(&mut s);
        println!("{}", s); // hello_world!

        // 可变借用有一个非常严格的限制，那就是只能有一个可变借用。
        // 可变借用期间，不允许使用其它的任何借用，包括不可变借用； 可变借用期间，拥有者本身也不能进行任何操作，不能转移，不能修改值。
    }
}
