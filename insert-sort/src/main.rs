//! Insert Sort
//! Usage
//! VEC_SIZE(ランダムで生成する配列数)
//! MIN(ランダムの下限値)
//! MAX(ランダムの上限値)


use rand::Rng;
fn main() {
    println!("挿入ソートの実装");
    const VEC_SIZE:usize = 10;
    const MIN:i16 = -1000;
    const MAX:i16 = 1001; 
    let random_vec:Vec<i16> = generate_randomvec(MIN,MAX, VEC_SIZE);
    let sorted_vec:Vec<i16> = insert_sort(random_vec.clone());
    println!("\n ソート前の配列は \n {:?} \n です", &random_vec);
    println!("\n ソート後の配列は \n {:?} \n です", &sorted_vec);
}

///      
/// ランダム配列を生成する関数
/// 

fn generate_randomvec(min:i16 ,max:i16, num:usize) -> Vec<i16> {
    let mut random_vec:Vec<i16>= Vec::new();
    for _i in 0..num {
      random_vec.push(rand::thread_rng().gen_range(min, max));
    }
    random_vec
} 

///
/// 挿入ソート関数(WIP)
/// 

fn insert_sort (mut input: Vec<i16>) -> Vec<i16> {
    for _i in 0..input.len()-1{
        for j in 0..input.len()-1 {
            if input[j] > input[j+1] {
                input.swap(j,j+1);
            }
        }
    }
    input.to_vec()
}

#[test]
fn test_insert_sort() {
    let vec1:Vec<i16> = vec![299, 773, 339, -1000, -227, 579, 182, 1000, -660, 313];
    let mut vec2:Vec<i16> = vec1.clone();
    vec2.sort();        // 標準ライブラリの昇順ソート
//  vec2.swap(0,1);     // この行を入れて配列を入れ替えるとテストは落ちる
    assert_eq!( vec2, insert_sort(vec1));
}
#[test]
fn test_insert_sort2(){
    let vec3 = generate_randomvec(-1000, 1000, 10);
    let mut vec4 = vec3.clone();
    vec4.sort();
    assert_eq!( vec4, insert_sort(vec3));
}
