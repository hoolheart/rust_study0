# rust_study0

## 介绍

Rust学习项目0

## 2021-12-14

第一个例子，猜数字：

1. 使用`rand`获取一个100以内的随机自然数；
2. 要求用户输入猜测值；
3. 反馈太小或太大的提示，直至用户猜到之前产生的随机数，然后退出；
4. 若输入不符合要求，打印提示并重试。

## 2021-12-16

第二个例子, 测试变量和常量.

## 2021-12-17

第三个例子, 测试各种基本类型.

## 2021-12-25

第四个例子, 测试函数, 将第一个猜数字的例子封装成为函数, 并增加两个参数:

1. `show_answer`, 布尔类型, 是否显示答案;
2. `max_cnt`, `i32`类型, 最大尝试次数, 正数有效.

并增加布尔类型的返回值, 代表猜数字的结果.

## 2022-02-19

第五个例子, 测试所有权函数, 从所有权上就可以看出, Rust其实就是强制性的要求所有变量都按照现代C++所定义的RAII来使用, 可以说Rust就是语言层面上强制性的C++最佳实践.

其实ownership是个强制性的东西, 并没有什么合适的测试代码, 因为不符合所有权要求的代码根本编译不通过, 函数`test_ownership`也仅仅是测试了一下String和str(相当于Slice)类型.

第六个例子, 测试结构体, 基本上和Golang一样, 比C++多了匿名结构体和空结构体, 参见函数`test_struct`.

结构体高级用法(面向对象):

1. 增加调试信息`#[derive(Debug)]`
2. 增加方法`impl <StructName> { fn <MethodName>(&[mut ]self[, ...]) [-> ...] {} }`
3. 增加静态方法`impl <StructName> { fn <MethodName>(...) [-> ...] {} }`
