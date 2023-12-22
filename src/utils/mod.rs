use std::fmt::Debug;

pub fn find_duplicates<T: std::cmp::Eq + std::hash::Hash + Clone + Debug>(vec: Vec<T>) -> Vec<T> {
  let mut set = Vec::new();
  let mut duplicates = Vec::new();
  #[cfg(debug_assertions)]
  println!("Data : {:?}", vec);

  for item in vec {
    #[cfg(debug_assertions)]
    println!("Key : {:?}\nDuplicates : {:?}\n-----------", item, duplicates);
    if set.contains(&item) {
      duplicates.push(item.clone());
    } else {
      set.push(item)
    }
  }

  duplicates
}
