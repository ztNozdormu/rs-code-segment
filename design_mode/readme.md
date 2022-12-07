设计模式参考文章 https://vector.blog.csdn.net/article/details/119341262?spm=1001.2101.3001.6650.1&utm_medium=distribute.pc_relevant.none-task-blog-2%7Edefault%7ECTRLIST%7Edefault-1-119341262-blog-124391830.pc_relevant_aa2&depth_1-utm_source=distribute.pc_relevant.none-task-blog-2%7Edefault%7ECTRLIST%7Edefault-1-119341262-blog-124391830.pc_relevant_aa2

github 地址: https://github.com/lpxxn/rust-design-pattern

目前实现的有，会持续更新：

序号	模式 & 描述	已经实现的模式
1	**创建型模式 **
这些设计模式提供了一种在创建对象的同时隐藏创建逻辑的方式，
而不是使用 new 运算符直接实例化对象。
这使得程序在判断针对某个给定实例需要创建哪些对象时更加灵活。	工厂模式（Factory Pattern）
[抽象工厂模式（Abstract Factory Pattern）](Abstract Factory)
建造者模式（Builder Pattern）
单例模式（SingletonPattern）
2	行为型模式
这些设计模式特别关注对象之间的通信。	策略模式（Strategy Pattern）
状态模式（State Pattern）
命令模式（Command Pattern）
迭代器模式（Itera tor Pattern）
观察者模式（Observer Pattern）
责任链模式（Chain of Responsibility Pattern）
3	结构型模式
这些设计模式关注类和对象的组合。继承的概念被用来组合接口和定义组合对象获得新功能的方式。	适配器模式（Adapter Pattern）
装饰器模式（Decorator Pattern）
代理模式（Proxy Pattern）

### 设计模式分类
目录
1.概述
1.1.创建型模式[Creational Pattern]
1.2.结构型模式[Structural Pattern]
1.3.行为型模式[Behavioral Pattern]
2.分类依据
3.模式简述
4.六大原则
5.源码分享
1.概述
经典设计模式大方向分为三大类，创建型模式、结构型模式、行为型模式。创建型模式里面包含5个，结构型模式包含7个，行为型模式包含11个。

1.1.创建型模式[Creational Pattern]
单例模式
抽象工厂模式
工厂方法模式
建造者模式
原型模式
1.2.结构型模式[Structural Pattern]
适配器模式
装饰器模式
代理模式
外观模式
桥接模式
组合模式
享元模式
1.3.行为型模式[Behavioral Pattern]
策略模式
模板方法模式
观察者模式
迭代子模式
责任链模式
命令模式
备忘录模式
状态模式
访问者模式
中介者模式
解释器模式
2.分类依据
创建型模式：主要用于处理对象的创建，实例化对象。但是，这可能会限制在系统内创建对象的类型或数目。
结构型模式：处理类或对象间的组合。它将以不同的方式影响着程序，允许在补充写代码或自定义代码的情况下创建系统，而且具有重复使用性和应用性能。
行为型模式：描述类或对象怎样进行交互和职责分配。影响系统的状态、行为流，简化、优化并且提高应用程序的可维护性。
3.模式简述
单例模式(Singleton Pattern)： 保证一个类仅有一个实例，并提供一个访问它的全局访问点。

工厂方法模式(Factory Method Pattern)： 定义一个用于创建对象的接口，让子类决定将哪一个类实例化。Factory Method使一个类的实例化延伸到其子类。

抽象工厂模式(Abstract Factory Pattern)： 提供一个创建一系列相关或相互依赖对象的接口，而无需指定它们具体的类。

建造者模式(Builder Pattern)： 将一个复杂对象的构建与它的表示分离，使得同样的构建过程可以创建不同的表示。

原型模式(Prototype Pattern)： 用原型实例指定创建对象的种类，并且通过拷贝这个原型来创建新的对象。

适配器模式(Adapter Pattern)： 将一个类的接口转换成客户希望的另外一个接口。Adapter模式使得原本由于接口不兼容而不能一起工作的那些类可以一起工作。

桥接模式(Bridge Pattern)： 将抽象部分与它的实现部分分离，使它们都可以独立地变化。

组合模式(Composite Pattern)： 将对象组合成树形结构以表示“部分-整体”的层次结构。它使得客户对单个对象和复合对象的使用具有一致性。

装饰模式(Decorator Pattern)： 动态地给一个对象添加一些额外的职责。就扩展功能而言，它比生成子类方式更为灵活。

外观模式(Facade Pattern)： 为子系统中的一组接口提供一个一致的界面，Facade模式定义了一个高层接口，这个接口使得这一子系统更加容易使用。

享元模式(Flyweight Pattern)： 运用共享技术有效地支持大量细粒度的对象。

**代理模式(Proxy Pattern)：**为其他对象提供一个代理以控制对这个对象的访问。

**责任链模式(Chain of Responsibility Pattern)：**为解除请求的发送者和接收者之间耦合，而使多个对象都有机会处理这个请求。将这些对象连成一条链，并沿着这条链传递该请求，直到有一个对象处理它。

命令模式(Command Pattern)： 将一个请求封装为一个对象，从而使你可用不同的请求对客户进行参数化；对请求排队或记录请求日志，以及支持可取消的操作。

**解释器模式(Interpreter Pattern)：**给定一个语言，定义它的文法的一种表示，并定义一个解释器, 该解释器使用该表示来解释语言中的句子。

迭代器模式(Iterator Pattern): 提供一种方法顺序访问一个聚合对象中各个元素, 而又不需暴露该对象的内部表示。

中介者模式(Mediator Pattern): 用一个中介对象来封装一系列的对象交互。中介者使各对象不需要显式地相互引用，从而使其耦合松散，而且可以独立地改变它们之间的交互。

备忘录模式(Memento Pattern): 在不破坏封装性的前提下，捕获一个对象的内部状态，并在该对象之外保存这个状态。这样以后就可将该对象恢复到保存的状态。

观察者模式(Observer Pattern): 定义对象间的一种一对多的依赖关系,以便当一个对象的状态发生改变时,所有依赖于它的对象都得到通知并自动刷新。

状态模式(State Pattern): 允许一个对象在其内部状态改变时改变它的行为。对象看起来似乎修改了它所属的类。

策略模式(Strategy Pattern): 定义一系列的算法，把它们一个个封装起来，并且使它们可相互替换。本模式使得算法的变化可独立于使用它的客户。

模板方法模式(Template Method Pattern): 定义一个操作中的算法的骨架，而将一些步骤延迟到子类中。Template Method使得子类可以不改变一个算法的结构即可重定义该算法的某些特定步骤。

访问者模式(Visitor Pattern): 表示一个作用于某对象结构中的各元素的操作。它使你可以在不改变各元素的类的前提下定义作用于这些元素的新操作。

4.六大原则
原则	解释
单一原则	一个类只做一件事
开放-封闭原则（OCP）	软件实体（类、模块、函数）可以拓展，但是不可修改
依赖倒转原则（DIP）	A.高层模块不应该依赖底层，两个都应该依赖抽。B.抽象不应该依赖细节，细节依赖抽象
里氏代换原则（LSP）	子类型必须能够替换掉它们的父类型
迪米特法则（LoD）	如果两个类不必直接通信，那么这两个类不应当发生直接的相互作用。如果其中一个类需要调用另一个类的某一个方法的话，可通过第三者发起这个调用
合成/聚合复用原则（CARP）	尽量使用合成/聚合，尽量不要使用类继承


## RUST测试方法编写说明:

运行测试

在函数上加#[test],可以把函数变为测试函数

使用cargo test命令来执行测试函数

mod tests {
    #[test]
    fn it_works1() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works2() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
//running 2 tests
//test tests::it_works1 ... ok
//test tests::it_works2 ... ok
断言(Assert)
assert!:用来确定某个状态是否为true

assert_eq!和assert_ne!测试相等性

#[should_panic]加在方法前面，方法出现panic测试通过，不出现panic测试失败

#[should_panic(expected = "***")],加入expected参数可以进行对选定panic测试通过

为cargo test添加命令行参数来改变cargo test的行为

cargo test --help:显示cargo test ***的所有参数

cargo test -- --help:显示cargo test --***的所有参数

cargo test -- --show-output

cargo test后加想要测试的名称，就可以选择性的进行测试

#[cfg(test)]
#[test]
fn add1(){
    println!("add1");
}
#[test]
fn add2(){
    println!("add1");
}
cargo test: add1和add2都会测试
cargo test add1: 只会测试add1
cargo test add: 包含add字段的方法都会被测试
#[ignore]加在方法前可以使该方法执行cargo test时被忽略测试
