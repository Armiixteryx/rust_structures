#![allow(unused_variables)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@you.com"),
        sign_in_count: 1,
        active: false,
        username: String::from("Someone"),
    };

    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
    println!("{}", user1.username);
    println!("{}", user1.active);

    println!("\n");

    //Una instancia puede ser mutable.
    //Toda la estructura será mutable sin excepción.
    //No es válido tratar de instanciar atributos como mutables.
    let mut user2 = User {
        username: String::from("anotherguy123"),
        email: String::from("anotherguy@you.com"),
        sign_in_count: 1,
        active: true,
    };

    //Puedes cambiar el contenido de una estructura instanciada como mutable.
    user2.username = String::from("anotherGuy321");

    println!("{}", user2.username);
    println!("{}", user2.email);
    println!("{}", user2.sign_in_count);
    println!("{}", user2.active);

    println!("\n");

    //Actualización de estructura. Creando un usuario en base a otro ya creado.
    let user3 = User {
        username: String::from("newuser"),
        email: String::from("newuser@you.com"),
        ..user1
    };
    print_user(user3);
    
    //Usando funciones.
    let new_username = String::from("itsme");
    let new_email = String::from("me@you.com");
    let user4 = build_user(new_username, new_email);
    print_user(user4);

    //Tuplas como estructuras.
    let color1 = Color(74, 190, 152);
    print_color(color1);

    let point1 = Point(50, 60, 3);
    print_point(point1);

    let red: u8 = 22;
    let green: u8 = 50;
    let blue: u8 = 75;
    let color2 = build_color(red, green, blue);
    print_color(color2);

    let x_axis = 554;
    let y_axis = 32;
    let z_axis = 43;
    let point2 = build_point(x_axis, y_axis, z_axis);
    print_point(point2);
}

fn build_user(username: String, email: String) -> User {
    User {
        email, //igual que email: email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

fn print_user(x: User) {
    //Hay mejores maneras de imprimir una estructura.
    //En este punto no las conozco aún.
    println!("{}", x.email);
    println!("{}", x.username);
    println!("{}", x.sign_in_count);
    println!("{}", x.active);
    println!("\n");
}

fn build_color(red: u8, green: u8, blue: u8) -> Color {
    let new_color = Color(red, green, blue);
    new_color
}

fn print_color(x: Color) {
    print!("Color: ");
    println!("RED: {}, GREEN: {}, BLUE: {}", x.0, x.1, x.2);
}

fn build_point(x_axis: i32, y_axis: i32, z_axis: i32) -> Point {
    let new_point = Point(x_axis, y_axis, z_axis);
    new_point
}

fn print_point(x: Point) {
    print!("Point: ");
    println!("X: {}, Y: {}, Z: {}", x.0, x.1, x.2);
}