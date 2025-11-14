
fn main() {

   let mut user1 = User {
    active:true,
    username:String::from("Ferris"),
    email:String::from("Ferris@gmail.com"),
    sign_in_count:4,
   };

   user1.email = String::from("Ferris1@gmail.com");

   //creating a struct instance from another struct instance
   //it's basically their version of inheritance
   let mut user2 = User{
    username:String::from("Ferris2.0"),
    ..user1
   };

   let black = Color(0,0,0);
   let origin = Point(0,0,0);
   let Point(x,y,z) = origin;
   let subject = AlwaysEqual;

   let x = 5;
   let y= 10;

   let AREA = area_of_rectangle(x,y);
   let AREA1 = area_of_square(x);

   println!("Area of rectangle : {}", AREA);
   println!("Area of sqaure : {}.", AREA1);

    let rect1:(u32,u32) = (30,50);

   println!("The area of the rectangle is {} square pixels.", area(rect1));


   let rect2 = Rectangle{
    width: 30,
    height: 70,
   };

   //println!("The area of the rectangle is {}.", _area(&rect2));

   println!("rect2 is {rect2:#?}");

   dbg!(&rect1);
}




fn build_user(email:String,username: String) -> User{
    User{
        active:true,
        username, // short would be username : email would be username,
        email, // short would be email : email would be email,
        sign_in_count:1,
    }  //This is under the assumption the value and key aer the same name.
}

struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count:u64,
    }

// struct Admin{
//    active:bool,
//    username:&str, //will throw an error. &str is unknown at runtime
//    email:&str,    //this is  immutable and that contributes to the error.
//    sign_in_count:u64,
//}

//Tuple Struct
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
struct AlwaysEqual;

//Calculate the Area of A rectangle
fn area_of_rectangle( x:i32 , y:i32) -> i32{

     x * y

}

//Calculate the Area of A Square

fn area_of_square(x:i32) -> i32 {

   x * x

}

fn area(dimensions:(u32,u32)) -> u32{
    dimensions.0 * dimensions.1
}


#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}

fn _area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}