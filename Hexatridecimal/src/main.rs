use rand::Rng;
fn main() {
    println!("10進数→36進数の変換");
    let arr = ["0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];
    let num = rand::thread_rng().gen_range(0, 36*36*36-1);
    let mut ans = [0;3];
    ans[0] = num/36/36;
    ans[1] = (num-ans[0]*36*36)/36;
    ans[2] = num-ans[0]*36*36-ans[1]*36;
    let mut ans36 = ["0";3];
    for i in 0..3{
        ans36[i] = arr[ans[i]];
    }
    println!("dec:{}  is Hexatridec:{}{}{}",&num,&ans36[0],&ans36[1],&ans36[2]);
}
