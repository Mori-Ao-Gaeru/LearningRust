use rand::Rng;
fn main() {
  println!("バブルソートアルゴリズムの実装");
  const VEC_SIZE:usize = 100;
  let mut random_vec:Vec<i16>= Vec::new();
  for _i in 0..VEC_SIZE {
    random_vec.push(rand::thread_rng().gen_range(-1000, 1001));
  }
  print!("\n ランダム配列を作成しました \n {:?} \n",random_vec);

  let sorted_vec :Vec<i16> = bubble_sort(random_vec, VEC_SIZE);
  println!("\n ソート後の配列は \n {:?} \n です", &sorted_vec);
}

fn bubble_sort (mut input: Vec<i16> , _size:usize) -> Vec<i16> {
  for _i in 0.._size-1{
    for j in 0.._size-1 {
      if input[j] > input[j+1] {
         input.swap(j,j+1);
      }
    }
  }
  input
}

#[test]
fn test_bs() {
  let vec1:Vec<i16> = vec![299, 773, 339, -1000, -227, 579, 182, 1000, -660, 313];
  let mut vec2:Vec<i16> = vec1.clone();
  vec2.sort();        // 標準ライブラリの昇順ソート
//  vec2.swap(0,1);     // この行を入れて配列を入れ替えるとテストは落ちる
  assert_eq!( vec2, bubble_sort(vec1, 10));
}