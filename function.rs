fn with_out_param(){
	println!("This function is without parameter");
}
	
fn with_param(x: i32){
	println!("This function has parameter x = {}",x);
}
	
fn return_val(x: i32) -> i32 {
	return x;
}

fn main(){
	with_out_param();
	with_param(33);
	let z = return_val(4);
	println!("{}", z);
}
