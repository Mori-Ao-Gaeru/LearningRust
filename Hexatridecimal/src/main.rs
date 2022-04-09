use rand::Rng;
fn main() {
  println!("36進数の実装");
    let arr = ["0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];
    let num = rand::thread_rng().gen_range(0, 36*36*36-1);
    println!("{}",&num);
    let mut ans = [0;3];
    for i in 0..2{
        ans[i] = num/36/36;
    }
    ans[2] = num%36;
    let mut ans36 = ["0";3];
    for j in 0..ans.len(){
        ans36[j] = arr[ans[j]];
    }
    println!("dec:{}  is Hexatridec:{}{}{}",&num,&ans36[0],&ans36[1],&ans36[2]);
}
