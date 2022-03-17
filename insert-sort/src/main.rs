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
    let mut randarray = [0;VEC_SIZE];
    for i in 0..VEC_SIZE{
        randarray[i] = rand::thread_rng().gen_range(MIN, MAX);
    }
    println!("\n ソート前の配列は \n {:?} \n です", randarray);
    let mut sorted = Vec::new();
    sorted.push(randarray[0]);
    println!("sort start! {:?}",sorted);
    for j in 1..randarray.len(){
        let mut is_inserted= false;
        for i in 0..sorted.len(){
            if randarray[j] <= sorted[i]{
            sorted.insert(i, randarray[j]);
            println!("{} is inserted in [{}] {:?}",randarray[j],i,sorted);
            is_inserted = true;
            break;
            }
        }
        if !is_inserted {
            sorted.push(randarray[j]);
            println!("{} is inserted in end {:?}",randarray[j],sorted);
        }
    }
    println!("\n ソート後の配列は \n {:?} \n です", sorted);
}
