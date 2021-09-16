
enum EIPAddrVersion{	
	V4(u8, u8, u8, u8),
	V6(String),
}

impl EIPAddrVersion{

	fn print(&self){
		match self{
			EIPAddrVersion::V4(d1, d2, d3, d4) => println!("IP V4 = {}.{}.{}.{}", d1, d2, d3, d4),
			EIPAddrVersion::V6(value) => println!("IP V6 = {}", value),
		};
	}

}

pub fn enums(){
	let ip_4 = EIPAddrVersion::V4(127,0,0,1);
	let ip_6 = EIPAddrVersion::V6(String::from("2001:b07:645e:fc4c:f1ab:bfd6:92aa:8f18"));

	print_ip_ver(&ip_4);
	print_ip_ver(&ip_6);
}

fn print_ip_ver(ip_addr : &EIPAddrVersion){
	ip_addr.print();
	if let EIPAddrVersion::V4(_d1, _d2, _d3, _d4) = ip_addr {
		println!("This is an old IP!!!\n");
	}
}