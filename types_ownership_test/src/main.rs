#[derive(Debug)]
struct A;
#[derive(Debug)]
struct B;
#[derive(Debug)]
struct X;
#[derive(Debug)]
struct Y;

impl A {
    fn become_x(self) -> X {
        X
    }

    fn become_y(self) -> Y {
        Y
    }
}

impl X {
    fn become_b(self) -> B {
        B
    }
}

impl Y {
    fn become_b(self) -> B {
        B
    }
}

fn main() {
    let v1 = A;
    println!("{:?}", v1);
    let v1 = v1.become_x();
    println!("{:?}", v1);
    let v1 = v1.become_b();
    println!("{:?}", v1);
    let v2 = A;
    println!("{:?}", v2);
    let v2 = v2.become_y();
    println!("{:?}", v2);
    let v2 = v2.become_b();
    println!("{:?}", v2);
}
