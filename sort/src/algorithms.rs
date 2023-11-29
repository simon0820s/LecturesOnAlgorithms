pub fn insertion (mut list: Vec<f32>) -> Vec<f32>{

  for i in 2..list.len() {
      let key = list[i];
      let mut j = i-1;
      //Insert list[i] into sorted_list
      while j > 0 && list[j] > key {
        list[j+1] = list[j];
        j = j-1;
      }
      list[j+1] = key;
  }
  list
}