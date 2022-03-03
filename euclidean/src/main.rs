use std::io;
use std::process;
use rand::Rng;
fn main() {
  println!("ユークリッドの互除法プログラム");
  loop {
    println!("0以外の好きな数字を入力してください");
    let mut input_number:String = String::new();
    io::stdin().read_line(&mut input_number)
                .expect("読み込みに失敗しました");

    let input_number:i32  = match input_number.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    if input_number == 0{
      continue;
    }
    let random_number:i32 = rand::thread_rng().gen_range(1, 1000);
    println!("入力されたのは {} です", input_number);
    println!("ランダム数値は {} です", random_number);

    println!(" {} と {} の最大公約数は {} です",
              input_number, random_number, get_greatest_com_div(input_number, random_number)
    );
    process::exit(0);
  }
}
fn get_greatest_com_div(mut a: i32, mut b:i32) -> i32 {
  loop{
    let c:i32 = a % b;
    a = b;
    b = c;
    if b == 0 {
      break a.abs()     //解答を絶対値に変換
    }
  }
}

#[test]
fn test_euclid_argo() {
  assert_eq!( 25, get_greatest_com_div( 150,  125) );  //正大小
  assert_eq!( 25, get_greatest_com_div( 125,  150) );  //正小大
  assert_eq!( 25, get_greatest_com_div(-125,  150) );  //負小正大
  assert_eq!( 25, get_greatest_com_div(-150, -125) );  //負大小
  assert_eq!( 25, get_greatest_com_div(-125, -150) );  //負小大
}