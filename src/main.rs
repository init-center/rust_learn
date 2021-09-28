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
    // 如果过早回收了，将会出现无效变量。如果重复回收（二次释放， double free），这也是个 bug。

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
        // 因为 对象的所有者被转移给了str， s 失去了所有权，所以不能再使用这个值（为了避免二次释放问题）
        // 因为如果两个变量的指针都指向同一内存地址，在作用域结束时两个变量都调用drop，会造成二次释放
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
        // 所有的整数，浮点，布尔，char，所有字段都可Copy的Tuple，array
        // 任何需要分配内存的类型都不是拥有copy trait的（可自己实现，但对于自定义类型，只有所有的成员都实现了 Copy trait，这个类型才有资格实现 Copy trait。）

        // 另一种避免转移的方法，就是调用通用的clone方法，这会克隆一份数据而不是转移
        // 但这也意味着更多的资源消耗
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);

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
        println!("{}", i); // 依然有效，因为i是i32类型，实现了copy
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
        let a = &s; // 借用而不是转移
        println!("{}", s); // 可以使用
                           // a = "world!";    // 借用者不能修改借用的值
                           // let b = s;   // 借出时不能转移所有权
                           // s = String::from("world!");  // 借出时也不能修改值
        println!("{}", a);
        console_str(&s); // 借用
        let b = s; // 此时借用已经结束，可以转移
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

        // 可变借用有一个非常严格的限制，那就是特定作用域中（从声明开始到最后一次使用）只能有一个可变借用。
        // 可变借用期间，不允许使用其它的任何借用，包括不可变借用；
        // 可变借用期间，拥有者本身也不能进行任何操作，不能转移，不能修改值。
        // 这可以在编译时就避免数据竞争
        // 数据竞争可由三个行为造成：
        // 多个指针同时访问同一数据
        // 至少有一个指针被用来写数据
        // 没有同步数据访问的机制
    }

    // 避免悬空指针
    {
        // fn dangle() -> &String {
        //     let s = String::from("hello");

        //     &s
        // }
        // 在rust中不允许，因为这个函数试图返回一个指针
        // 但是rust会在该函数结束时清理掉s
        // 所以&s就会变成一个悬空指针，这是rust不允许的
        // 在这个例子中，正确的做法应该是返回s，也就是返回值来让所有权转移
    }

    // 字符串slice
    // 适用于ASCII编码的字符，如果从多个字节的中间位置开始切，会panic出错
    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}", hello); // Hello
    println!("{}", world); // world

    // 下面的会出错
    // let s = String::from("你好");
    // let ni = &s[0..2];
    // let hao = &s[3..4];
    // println!("{}", ni); // byte index 2 is not a char boundary;
    // println!("{}", hao);

    // 简便写法
    let s = String::from("hello world");
    let s1 = &s[..5];
    let s2 = &s[6..s.len()];
    let s3 = &s[6..];
    let s4 = &s[..];
    // s.clear();   //错误，该字符串已被借用为不可变的，不能再借用为可变的，因为 clear 需要清空 String，它尝试获取一个可变引用。
    println!("简便写法");
    println!("{}", s1); // hello
    println!("{}", s2); // world
    println!("{}", s3); // world
    println!("{}", s4); // hello world

    // 学到这里，字符串的两种类型已经很明显了，
    // 为什么每一次使用字符串都要这样写String::from("hello world") ，
    // 而不是直接写字面量
    // 实际上字符串字面量的类型是str，就是字符串切片，常常以引用的形式出现（&str）
    // 也就是借用，不能改变
    // String 类型是 Rust 标准公共库提供的一种数据类型，
    // 它的功能更完善——它支持字符串的追加、清空等实用的操作。
    // String 和 str 除了同样拥有一个字符开始位置属性和一个字符串长度属性以外还有一个容量（capacity）属性。
    // 切片的结果必须是引用类型
    // String类型改成切片也很简单，就是用切片切一下即可，比如 &s[..]

    // 除了字符串意外，其它一些线性数据结构也支持切片操作，比如数组
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }

    {
        // b''可以获取字符的ascii编码
        let b = b'a';
        println!("{}", b); // 97
    }

    {
        fn first_word(s: &String) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }

        let s = String::from("hello world");

        let word = first_word(&s);

        println!("the first word is: {}", word);
    }

    // 结构体
    {
        // 定义结构体
        struct User {
            username: String,
            email: String,
        }

        // 实例化
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("username123"),
        };
        println!("{}", user1.username);

        // user1.email = String::from("anotheremail@example.com");
        // 错误，因为这个实例定义为不可变的

        // 可变的实例
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("username123"),
        };

        user1.email = String::from("anotheremail@example.com");

        fn build_user(email: String, username: String) -> User {
            User {
                email, // 和js一样，这是email: email的简写
                username,
            }
        }

        let user = build_user(String::from("a@b.com"), String::from("DaMing"));
        println!("{}, {}", user.email, user.username);

        // 从其它实例创建实例
        // 类似于JS中ES6的展开
        // 这里只更新了username的值
        let user2 = User {
            username: String::from("XiaoMing"),
            ..user
        };
        println!("update userName:{}, email: {}", user2.username, user2.email);
        let user3 = User { ..user2 };
        println!("user3 userName:{}, email: {}", user3.username, user3.email);
    }

    {
        // 元组结构体
        // 想要打印结构体或者复杂类型，需要通过#[derive]派生属性
        // 来提供某些trait实现
        // 这类似于其它语言的装饰器
        // 实现Debug可以打印
        #[derive(Debug)]
        struct Color(i32, i32, i32);
        #[derive(Debug)]
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
        // 打印结构体时需要:?占位，或者使用:#?打印看起来更美观一点
        println!("{:?}, {:#?}", black, origin);

        // rust中()空括号表示空元组，这是rust中的Unit Type，和kotlin中的Unit类似
        // 当不返回值时默认返回的就是()

        //  这是一个类单元结构体（unit-like structs）
        // 因为它们类似于 ()，即 unit 类型。
        // 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
        #[derive(Debug)]
        struct Empty;
        let empty = Empty {};
        println!("{:?}", empty);
    }

    // 给结构体定义方法
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        // 使用 impl 关键字 后面跟结构体
        impl Rectangle {
            // 定义的方法第一个参数总是self，代表调用的实例
            // 类似于js的this
            // 这里传递的是不可变借用
            // 如果是可变借用（需要修改实例本身），可以使用&mut self
            // 调用的实例不需要手动解引用和引用后再调用
            // rust会自动引用和解引用
            fn area(&self) -> u32 {
                self.width * self.height
            }

            // 编写另一个方法
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );

        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    }

    {
        // 关联函数
        // 类似于其它语言中的静态函数（关联函数是函数，不是方法）
        // 当第一个参数不是self，它就是一个关联函数
        // 关联函数经常被用作返回一个结构体新实例的构造函数。
        // 调用时使用:: 来调用，比如之前的String::from函数
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn square(size: u32) -> Rectangle {
                Rectangle {
                    width: size,
                    height: size,
                }
            }
        }

        let sq = Rectangle::square(20);
        println!("{:#?}", sq);

        // 多个impl块，一个结构体可以定义多个impl块
        // impl Rectangle {
        //     fn can_hold(&self, other: &Rectangle) -> bool {
        //         self.width > other.width && self.height > other.height
        //     }
        // }
    }
}
