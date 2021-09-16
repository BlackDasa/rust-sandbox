
pub fn ownership(){
	let s1 = create_base_string();
	let s2 = take_and_give_back(s1);
	println!("{}", s2);
	let len = get_str_len(&s2);
	println!("Len of s2 is {}", len);
	let mut s3 = s2.clone();
	take_ownership(s2);

	borrow_and_modify(&mut s3);

	let mut s3 = create_base_string();
	//let s4 = &s3;
	//let s5 = &s3;
	let s6 = &mut s3;
	s6.push_str("test");
	s3.push_str("test");

}

fn take_ownership(string_to_own : String){
	let string_to_own = take_and_give_back(string_to_own);
	println!("{}", string_to_own);
}

fn create_base_string() -> String{
	let string_to_create = String::from("Hello world!!!");
	string_to_create
}

fn take_and_give_back(mut string_to_change : String) -> String{
	string_to_change.push_str(" This is a changed string.");
	string_to_change
}

fn get_str_len(string_analyze : &String) -> usize{
	string_analyze.len()
}

fn borrow_and_modify(string_to_modify : &mut String){
	string_to_modify.push_str("Any change");
}