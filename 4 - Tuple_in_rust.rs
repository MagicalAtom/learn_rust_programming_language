fn main(){
    let users = ("mehran","reza","ali");
    let user1 = users.1; // extract reza from tuple
    let (x,y,z) = users; // extract all names from tuple
    print!("{x}");
}
