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

    // 枚举
    #[derive(Debug)]
    enum Ip {
        V4,
        V6,
    }

    // 创建成员实例
    let v4 = Ip::V4;
    let v6 = Ip::V6;
    println!("{:?}", v4);
    println!("{:?}", v6);

    // 每个枚举成员都可以关联其它值
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let v4 = IpAddr::V4(String::from("127.0.0.1"));
    let v6 = IpAddr::V6(String::from("::1"));
    println!("{:?}", v4);
    println!("{:?}", v6);

    // 每个成员都可以关联不同的类型
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // 枚举上也可以实现方法
    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // rust中没有控制，但标准库的prelude中有一个可以编码存在或不存在概念的枚举
    // 这个枚举就是Option<T>
    // 定义如下
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // 因为处于prelude中，所以不需要引入包
    // 甚至Some和None也不需要指定Option::前缀
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; // 使用Null时必须显式定义类型
    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);

    // match 流程控制（类似switch）

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => {
                println!("{}", 5);
                5
            }
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }

    let c = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", c);

    // 使用match来匹配Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            // match中必须匹配所有情况，否则报错
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // 使用 | 来匹配多个值
        9 | 11 => println!("nine or eleven"),
        // 范围目前只支持 x..的形式，其它形式报错
        13.. => println!("thirteen range fifteen"),
        // 不想处理某些情况时，可以使用下划线_，类似于default
        _ => (),
    }

    // if let
    // 有时候我们只想处理一种情况，如果用match匹配就要写所有情况
    // 或者使用_，但还是太麻烦，这时可使用 if let
    // 这相当于match的语法糖，和match的工作方式相同，只不过不需要我们写那么多代码
    {
        let some_u8_value = Some(0u8);
        if let Some(3) = some_u8_value {
            println!("three");
        }

        // if let 可以加else
        // 这就和match中的_类似
        if let Some(3) = some_u8_value {
            println!("three");
        } else {
            println!("other");
        }
    }

    // 常用的集合（Vector, String, HashMap）
    // Vector在rust中类型是Vec<T>
    // Vector相当于可变长度的数组
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    // 使用初始值创建需要使用vec!();
    // 如果需要改变，声明为mut
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);
    v.push(4);
    println!("{:?}", v);
    // 读取，两种方式，索引或者get
    // 使用索引如果越界会panic
    let third = &v[2];
    println!("{}", third);
    // get 返回的是option
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    // vector和其它struct一样，当离开作用域就会被清除，它的所有元素也会被清除

    //遍历 使用 for...in
    for i in &v {
        println!("{}", i);
    }

    // 遍历
    for i in &mut v {
        // 引用不能直接计算，所以需要解引用
        *i *= 2;
    }
    println!("{:?}", v);

    // 字符串
    // rust中的字符串是Byte的集合
    // 它提供了一些方法，能将byte解析为文本
    // Rust的核心语言层面，只有一个字符串类型，就是字符串切片str(或&str)
    // 字符串切片是对存储在其它地方，UTF-8编码的字符串的引用
    // 字符串字面值是存在二进制文件中，也是字符串切片

    // String类型来自标准库而不是核心语言
    // String可增长、可修改、可拥有所有权

    // rust中所说的字符串一般指String或者&str两种类型
    // rust标准库中还包含了很多其它字符串类型。比如OsString、OsStr、CString、CStr
    // 这些类型中，String后缀的可拥有所有权，Str后缀的可借用
    // 这些字符串类型可存储不同编码的文本或在内存中以不同的形式展现

    // String，因为String本身是Byte的集合，所以很多Vec的操作都可用于String
    // 比如String::new()函数创建一个空串
    // 或者使用实现了Display trait的类型的to_string()方法转化一个String变量
    let s = "this is a string".to_string();
    println!("{}", s);
    // 使用String::from函数，可从字面值创建String
    let s = String::from("abc");
    println!("{}", s);
    // 更新string
    // push_str()，把一个字符串切片附加到String
    // push()方法，把单个字符附加到String
    // + 连接字符串
    let s1 = String::from("a");
    let s2 = String::from("b");
    // 注意，相加是后面的必须使用引用，并且编译器会发生强转(deref coercion)
    // 实际上调用了类似 fn add(self, s:&str) -> String {...}
    // 可以多个相加
    let s3 = s1 + &s2;
    println!("{}", s3);

    // 使用format!这个宏来连接多个字符串
    let s = format!("{}-{}", s2, s3);
    println!("{}", s); // b-ab

    // rust中的字符串是不支持通过索引来访问的
    // 获取字节数
    let len = String::from("Hello").len();
    println!("{}", len); // 5 (utf-8，英文一个一字节)
    let len = String::from("你好").len();
    println!("{}", len); // 6 （中文汉字一般一个3字节）

    let string_length = "我正在学习Rust~";
    // 按字节处理
    println!("\"{}\"的字节长度 : {}", string_length, string_length.len());
    // 按字符处理
    println!(
        "\"{}\"的字符长度 : {}",
        string_length,
        string_length.chars().count()
    );

    // HshMap<K, V>
    // 创建
    use std::collections::HashMap; // 首先需要引入
                                   // 可以直接写类型，帮助编译器推断类型
    let mut hm: HashMap<String, i32> = HashMap::new();
    // 也可以不写类型
    let mut hm = HashMap::new();
    // 使用insert插入，上面不写类型，下面写了插入后，编译器也能推断
    hm.insert(String::from("a"), 1);
    // 调用insert时，如果hm保存的类型不是引用，插入的值的所有权被转移，后续不能访问
    let item = String::from("b");
    hm.insert(item, 2);
    // println!("{}", item);    // 不行，报错

    // 在hashmap生效期间，被引用的值必须有效

    // 遍历hashMap
    for(k, v) in &hm {
        println!("{}: {}", k, v);
    }

    // hashMap所有的key必须是同一种类型，所有的value必须是同一种类型
    // 另一种创建hashMap的方法，使用collect()方法
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];
    // 遍历器调用zip将两个vec合并成存储一一对应的元组的数组，使用collect即可生成map
    // 这里我们需要手动声明结果的类型，因为collect可以返回很多种不同的数据结构。但HashMap中的K和V
    // 使用了两个下划线，这里可以让编译器根据Vec中的类型帮助我们推断K和V的类型
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:#?}", scores);
}

// rust的模块系统
// 由顶至下为
// package(包) -> crate(单元包) -> Module(模块) -> path(路径)

// crate有两种类型，binary和library
// crate root是crate的根Module
// 一个package包含一个cargo.toml，它是包的描述文件
// 只能包含0-1个library crate
// 可以包含任意数量的binary crate
// 必须至少包含一个crate（binary或library）

// package下src/main.rs是binary crate的 crate root
// crate名与package名相同

//package下src/lib.rs是library的 crate root
// 有这个文件说明package包含一个library crate
// crate名与package名相同

// cargo会把crate root文件交给rustc来构建library或binary

// 一个package可以同时包含src/main.rs和src/lib.rs
// 表明该package包含一个binary crate 和 一个 library crate
// 一个 package也可以有多个binary crate，文件放在src/bin中
// 每个文件都是单独的binary crate

// module可以在一个crate中对代码进行分组，以及控制项目的私有性
// 建立module使用mod关键字，mod可以嵌套

// 在rust的模块中找到某个条目，需要使用路径
// 路径有两种使用形式
// 绝对路径: 从crate root开始，使用crate的名称或者使用crate字面值（就是直接写crate这个词）
// 相对路径: 从当前模块开始，使用self，super或当前模块的标识符
// 路径至少由一个标识符组成，标识符之间使用::连接
mod front_of_house {
    // pub表示这是公开的，因为rust中的所有条目默认都是私有的
    // 要想公开，必须加pub
    // 父模块无法访问子模块中的私有条目
    // 子模块里可以使用所有祖先模块中的条目
    pub mod hosting {
        // rust中使用super关键字来访问父级模块路径中的 内容，类似文件系统的中..
        pub fn add_to_wait_list() -> super::Breakfast {
            super::Breakfast {
                toast: String::from("wheat"),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // struct是公共的，但是字段依然不是公共的
    // 每个字段都自己单独加pub
    // 不加的字段依然是私有的
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // 但枚举不同，只要将枚举设置为公共的
    // 那么它的所有成员都是公共的
    pub enum Appetizer {
        // 里面的成员都是公共的
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_wait_list();
    // 同一级可以直接写模块名
    front_of_house::hosting::add_to_wait_list();
}

// 使用 use 关键字可以将路径导入到作用域内
// use crate::front_of_house::hosting;
// use front_of_house::hosting;

// use的习惯用法
// 函数： 将函数的父级模块引入作用域（指定到父级）
// struct,enum,其它：指定完成路径（指定到本身）
// 同名条目，指定到父级或者使用as指定别名，比如 use std::io::Result as IoResult

// 将路径引入到作用域后还可以同时导出，导入时使用pub use即可（即导入又导出）

// use 可以一条use引入同一个路径下的多个条目
// 比如 use std::{cmp::Ordering, io} 引入了std下的cmp::Ordering和io
// std::io::{self, write} self代表std::io自身，同时还引入了下面的write

// 引入路径下的所有条目使用*号，谨慎使用，一般不用
// std::io::*

// 如何使用外部包
// cargo.toml添加依赖的包（cargo会从crates.io拉取包）
// use 将特定条目引入作用域即可

// 可以将模块的内容移动到其它文件中
// 在模块定义时后面跟的是分号而不是代码块，Rust会从src下与模块同名的文件中加载内容
// 比如在lib.rs中写 mod front_of_house;
// 那么就需要在src下创建front_of_house.rs文件
// 文件中写它代码块中的内容（注意是代码块中的内容，本身的定义不需要再写）
// 如果是多级模块拆分，模块在src下的路径需要和模块的路径一致
// 比如front_of_house下再拆分出hosting，front_of_house中写pub mod hosting;
// 需要在src/front_of_house/hosting.rs中写hosting模块的内容
