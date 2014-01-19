fn main(){
	let x = (41, false);
	match x{
	      (y,true) if y>20 && y<26 => {println("Statement a");},
	      (y,true) if y<20 || y>26 => {println("Statement b");},
	      (y,_) if y>40 && y<49 => {println("Statement c");},
	      (_,_) => {println("Default");}


	}
}
