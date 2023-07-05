// defining a enum that describes all possible types of IP
enum IpAddrKind {
    V4,
    V6,
}

// defining a enum that describes message behaviour
enum Message {
    // empty element
    Quit,
    // have named fields as struct
    Move { x: i32, y: i32 },
    // one String param
    Write(String),
    // tuple params
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
// we used enum instead of this structs
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {
    // get instance of enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // call functions with two different values
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // we can describe ip with a struct
    {
        struct IpAddr {
            // using enum as a type
            kind: IpAddrKind,
            address: String,
        }

        //examples of two types
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }

    // however its more concise to use enums
    {
        enum IpAddr {
            // Ip address in params
            V4(String),
            V6(String),
            // also we can use this variant to describe IPv4
            // V4(u8, u8, u8, u8),
        }
        //examples of two types created with enum
        let home = IpAddr::V4(String::from("127.0.0.1"));
        // let home = IpAddr::V4(127, 0, 0, 1); // if enum is described as V4(u8, u8, u8, u8),
        let loopback = IpAddr::V6(String::from("::1"));
    }

    // also we can give a struct as a param
    {
        struct Ipv4Addr {
            // --snip--
        }
        
        struct Ipv6Addr {
            // --snip--
        }
        
        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    }

    // calling a enum method
    let m = Message::Write(String::from("hello"));
    m.call();

    // example of using a enum Option<T>
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // wont work because Option<i8> and i8 are different types
    // let sum = x + y;
}

//function that take a IpAddrKind enum in params
fn route(ip_kind: IpAddrKind) {}