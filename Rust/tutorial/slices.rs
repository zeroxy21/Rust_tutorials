


fn main(){

let tab = &[1,2,3];
println!("{:?}",tab);

let s = &tab[1..];
println!("{:?}",s);

let mut v = Vec::new();

v.push(0);
v.push(1);
v.push(2);

println!("{:?}",v);

let s = &v[1..];
println!("{:?}",s);

}



