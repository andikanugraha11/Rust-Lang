fn even_or_odd(x:i32){
	if x%2 == 0 {
		println!("{} adalah bilangan genap",x);
	}else if x%2 != 0 {
		println!("{} adalah bilangan ganjil",x);
	}else{
		println!("Inputan anda salah");
	}
}

fn main(){
	even_or_odd(7);
}