use std::io;



fn read(y: bool){
    if y{
        println!("true")}
}

fn quiz(){
    for i in 1..4{
    println!("How old is Miku?");
    let mut input = String::new();

    
    io::stdin()
        .read_line(&mut input)
        .expect("huhhh whaat");

       let input: u8 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
       };
       


    println!("So you think she's {input}? huh?");
    if input == 16{
        println!("CORRECT!! Miku is super proud of you^^");
        break;
    }else if input == 19{
        println!("Corre- WAIT!NO! Not the software's age! HER age!!");
        continue;
    }else if input <=10{
        println!("She'a not a toddler yk..");
        continue;
    }else if input <=20{
        println!("You're not quite sure are you.. Well, you're wrong!!");
        continue;
    }else if input <= 30{
        println!("You aren't rly a vocaloid fan, are you.");
        continue;
    }else if input >=120 {
        println!("Are you joking?");
        continue;
    }else{
        println!("MIKU IS NOT AN OLD LADY!!");
        break;
    }
}
println!("You keep answering wrong. You need to better your Miku knowledge!!!")

}


struct Vocaloid{
    name: String,
    age: u8,
    item: String,
    cryptonloid: bool,
}

/*fn makeVoca(name: String, age: u8,item: String, cryptonloid: bool) -> Vocaloid {
   
    Vocaloid{
        name,
        age,
        item,
        cryptonloid,
    };
    
}
*/


fn main() {
    println!("Hello, world!");
    let mut n: u16 = 0;

    let mut x: i32 = 2;
    println!("x is {}!!",x);
    let mut y: i32 = 4;
    x = y;
    println!("no wait its {}..",x);
    read(false);

    //quiz();

    let mut len = Vocaloid {
        name: String::from("Kagamine Len"),
        age: 14,
        item: String::from("a banana"),
        cryptonloid: true,
        //number: 0,
    };

    let x = &mut len.age;
    *x+=1;
    println!("{} is {} years old and his item is {}",len.name,len.age,len.item );
    if len.cryptonloid {
        println!("{} is a cryptonloid",len.name);
    }else{
        return;
    }

}
