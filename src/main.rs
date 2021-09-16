use std::io;

mod lessons;

fn main(){

    let lessons = lessons::ELessons::describe();
    let quit_index = lessons.len() as u8;
    loop{
        print_menu(&lessons, quit_index);

        let user_choice = read_user_input();
        if user_choice == quit_index{
            println!("The application will now exit.");
            break;
        }

        let user_choice =lessons::ELessons::number_to_value(user_choice);
        match user_choice {
            Ok(value) => value.run(),
            Err(error) => println!("{}", error)
        }
    }
}

fn read_user_input() -> u8{
    let mut user_input = String::new();
    io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to read line");

    user_input.trim().parse::<u8>().unwrap()
}

fn print_menu(lessons_values : &Vec<String>, quit_index : u8){
    println!("Please select a lesson:");
    for (position, lesson) in lessons_values.iter().enumerate() {
        println!("{}-{}", position, lesson);
    };
    println!("{}-Quit", quit_index);
}