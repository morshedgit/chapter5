// struct User {
//     active:bool,
//     username:String,
//     email:String,
//     sign_in_count:u64
// }

#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // // Structs
    // // Defining

    // let mut user1 = User {
    //     email:String::from("some@email.com"),
    //     username:String::from("someuser"),
    //     active:true,
    //     sign_in_count:1,
    // };

    // user1.email = String::from("another@email.com");

    // let newUser = build_user(String::from("Sam"),String::from("Harris"));

    // println!("username: {}",newUser.username);

    // Methods
    let rect1 = Rectangle {
        width:30,
        height:50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

}

// // Field Init Shorthand
// fn build_user(email:String, username:String)->User{
//     User {
//         email,
//         username,
//         active:true,
//         sign_in_count:1,
//     }
// }
