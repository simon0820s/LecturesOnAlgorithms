pub fn insertion (mut list: Vec<f32>) -> Vec<f32>{

  for i in 1..list.len() {
      let mut j = i;
      //Insert list[i] into sorted_list
      while j > 0 && list[j] < list[j-1] {
        list.swap(j, j-1);
        j -= 1;
      }
    println!("{}", i)
  }
  list
}