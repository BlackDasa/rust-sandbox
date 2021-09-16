pub mod variables;
pub mod guess;
pub mod ownership;
pub mod structs;
pub mod enums;
pub mod strings;

pub enum ELessons{
    Guess,
    Variables,
    OwenerShip,
    Structs,
    Enums,
	Strings,
}

impl ELessons{

    pub fn run(&self){
        match self{
            ELessons::Guess => guess::guess(),
            ELessons::Variables => variables::variables(),
            ELessons::OwenerShip => ownership::ownership(),
            ELessons::Structs => structs::structs(),
            ELessons::Enums => enums::enums(),
			ELessons::Strings => strings::strings(),
        }
    }

	pub fn describe() -> Vec<String>{
		vec![String::from("Guess")
		,String::from("Variables")
		,String::from("OwenerShip")
		,String::from("Structs")
		,String::from("Enums")
		,String::from("Strings")
		]
	}

	pub fn number_to_value(number : u8) -> Result<ELessons, String>{
		match number{
			0 => Ok(ELessons::Guess),
			1 => Ok(ELessons::Variables),
			2 => Ok(ELessons::OwenerShip),
			3 => Ok(ELessons::Structs),
			4 => Ok(ELessons::Enums),
			5 => Ok(ELessons::Strings),
			_ => Err(String::from(format!("{} is not a valid lesson choice.", number))),
		}
	}
}