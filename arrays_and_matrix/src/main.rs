use std::mem;

fn arrays() {
    let arr_u8: [u8; 10] = [3; 10];
    let arr_u32: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //print array with loop
    for i in 0..arr_u32.len() {
        println!("{}. element's value = {}", i, arr_u32[i]);
    }
    println!("{:?}", arr_u8);
    println!(
        "size of u8 array in memory : {} byte",
        mem::size_of_val(&arr_u8)
    );
    println!(
        "size of u32 array in memory : {} byte",
        mem::size_of_val(&arr_u32)
    );
    //slice
    println!("{:?}", &arr_u32[3..10]);
}

fn matrix() {
    let matr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for i in 0..matr.len() {
        for j in 0..matr[i].len() {
            print!("{} ", matr[i][j]);
        }
        println!();
    }
}

fn match_fn() {
    let plate_number: u8 = 82;
    let city: &str = match plate_number {
        1 => "Adana",
        2 => "Adıyaman",
        3 => "Afyonkarahisar",
        4 => "Ağrı",
        5 => "Amasya",
        6 => "Ankara",
        7..=81 => "Other",
        _ => "invalid",
    };
    println!("Plate number : {} = City : {}", plate_number, city);
}

fn tuples() {
    let name: &str = "Marco";
    let age: u8 = 25;
    let success_rate: f32 = 0.5;
    let mut person = (name, age, success_rate);
    println!(
        "name : {}, age : {}, success rate : {}",
        person.0, person.1, person.2
    );
    person.0 = "Polo";
    person = (person.0, 20, 0.9);
    println!("{:?}", person);
}

fn struct_example() {
    struct Person {
        name: &'static str,
        age: u8,
        have_glasses: bool,
    }
    let p1: Person = Person {
        name: "John Doe",
        age: 21,
        have_glasses: true,
    };
    println!(
        "{} is {} years old. (using glasses : {})",
        p1.name, p1.age, p1.have_glasses
    );
    //generics
    struct Point<X, Y, Z> {
        x: X,
        y: Y,
        z: Z,
    }
    struct Triangle<X1, X2, X3, Y1, Y2, Y3, Z1, Z2, Z3> {
        corner1: Point<X1, Y1, Z1>,
        corner2: Point<X2, Y2, Z2>,
        corner3: Point<X3, Y3, Z3>,
    }
    let point1: Point<i32, i32, i32> = Point { x: 0, y: 0, z: 0 };
    let point2: Point<i32, i32, i32> = Point { x: 5, y: 5, z: 5 };
    let point3: Point<f64, i32, &str> = Point {
        x: -3.3,
        y: 5,
        z: "-3.7",
    };
    let mytriangle = Triangle {
        corner1: point1,
        corner2: point2,
        corner3: point3,
    };
    println!(
        "The triangle's corner points are {},{},{} - {},{},{} - {},{},{}",
        mytriangle.corner1.x, mytriangle.corner1.y, mytriangle.corner1.z,
        mytriangle.corner2.x, mytriangle.corner2.y, mytriangle.corner2.z,
        mytriangle.corner3.x, mytriangle.corner3.y, mytriangle.corner3.z
    );
}

fn main() {
    arrays();
    matrix();
    tuples();
    match_fn();
    struct_example();
}
