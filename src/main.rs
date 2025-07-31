
use std::io;


struct Numbers{
    num1 : f32,
    num2 : f32,
}
impl Numbers{
    fn add (&self)-> f32{
        self.num1+self.num2
    }
    fn sub (&self)-> f32{
        self.num1-self.num2
    }
    fn mul (&self)-> f32{
        self.num1*self.num2
    }
    fn div(&self)-> f32{
        self.num1/self.num2
    }
}

fn main (){
    println!("Enter the first number ");
    let mut num1 = String::new();
    io::stdin()
    .read_line(&mut num1)
    .expect("Cant read the number");
    let num1 :f32= num1.trim().parse().expect("This is not  a valid number");


    println!("Enter the second number ");
    let mut num2 = String::new();
    io::stdin()
    .read_line(&mut num2)
    .expect("Cant read the number");
    let num2 :f32= num2.trim().parse().expect("This is not a valid second number");


    let nums = Numbers{num1,num2};

    println!("The various operation that can be  performed on the numbes  are!!!! ");
    println!("!!!!!!!!!!!");
    println!("(+)--- type this for addition ");
    println!("(-)--- type this for subtraction ");
    println!("(*)--- type this for multiplication ");
    println!("(/)--- type this for Division ");



    let mut choice = String::new();
    io::stdin()
    .read_line(&mut choice)
    .expect("Cant read the symbol you entered ");
   let choice = choice.trim();

   match choice {
    "+"=> println!("The result of addition on {} and {} is {}",num1,num2,nums.add()),
    "-"=> println!("The result of subtraction performed on {} and {} is {} ",num1,num2,nums.sub()),
    "*"=> println!("The result of multiplication performed on {} and {} is {} ",num1,num2,nums.mul()),
    "/"=>{
            if num2 ==0.0{
            println!("The second num is 0 and that is not possible");
            }
            else {
            println!("The division of {} by {} is {}",num1,num2,nums.div());
            }
        }    
    _=>println!("This is not a valid choice sir "),
   }
}


