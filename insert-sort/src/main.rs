//! Insert Sort
//! Usage
//! VEC_SIZE(ランダムで生成する配列数)
//! MIN(ランダムの下限値)
//! MAX(ランダムの上限値)

use rand::Rng;
fn main() {
    println!("挿入ソートの実装");
    const VEC_SIZE:usize = 100;
    const MIN:i32 = -1000;
    const MAX:i32 = 1001; 
    let randvec = create_randvec(VEC_SIZE, MIN, MAX);
    println!("\n ソート前の配列は \n {:?} \n です", &randvec);
    
    let sortedvec = insert_sort(&randvec);
    println!("\n ソート後の配列は \n {:?} \n です", &sortedvec);
}


// ランダム配列を生成する関数
fn create_randvec(size:usize, min:i32, max:i32) -> Vec<i32>{
    let mut randvec = Vec::new();
    for _i in 0..size{
        randvec.push(rand::thread_rng().gen_range(min, max));
    }
    randvec
}
// 配列(ベクタ)を挿入ソートして返す関数
fn insert_sort(randvec:&Vec<i32>) -> Vec<i32>{
    let mut sorted = Vec::new();
    sorted.push(randvec[0]);
    println!("sort start! {:?}",sorted);
    for j in 1..randvec.len(){
        let mut is_inserted= false;
        for i in 0..sorted.len(){
            if randvec[j] <= sorted[i]{
            sorted.insert(i, randvec[j]);
            println!("{} is inserted in [{}] {:?}",&randvec[j],i,sorted);
            is_inserted = true;
            break;
            }
        }
        if !is_inserted {
            sorted.push(randvec[j]);
            println!("{} is inserted in end {:?}",&randvec[j],sorted);
        }
    }
    sorted
}

#[test]
fn test_ins_sort() {
  let vec1:Vec<i32> = create_randvec(10, -100,100);
  let mut vec2:Vec<i32> = vec1.clone();
  vec2.sort();        // 標準ライブラリの昇順ソート
//  vec2.swap(0,1);     // この行を入れて配列を入れ替えるとテストは落ちる
  assert_eq!( vec2, insert_sort(&vec1));
}