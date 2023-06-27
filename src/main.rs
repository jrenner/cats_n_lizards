/*

"""
Given several dictionaries/hash maps, create a single hash map consisting of:
 1. ONLY keys that are present in all hash maps
 2. The value of that key as the sum of the values for that key across all hash maps.
"""

dict1 = { "cats": 3, "dogs": 5, "birds": 2, "lizards": 4 }
dict2 = { "books": 5, "lizards": 11, "papers": 1, "cats": 2 }
dict3 = { "dogs": 8, "cats": 5, "lizards": 5 }

# write solution here:

*/

use std::collections::HashMap;

fn solve_for_dicts(dicts: Vec<HashMap<&str, i32>>) -> HashMap<&str, i32> {
    let mut common_keys: Vec<&str> = vec![];

    let d1 = &dicts[0];
    for k in d1.keys() {

        let mut is_common = true;

        for d in &dicts {
            if !d.contains_key(k) {
                is_common = false;
                break;
            }
        }
        if is_common {
            common_keys.push(k);
        }
    }

    println!("common keys: {common_keys:?}", common_keys=common_keys);

    let mut res: HashMap<&str, i32> = HashMap::new();

    for ckey in &common_keys {
        for d in &dicts {
            if let Some(val) = d.get(ckey) {
                let mut new_val: i32 = *val;
                if let Some(existing_val) = res.get(ckey) {
                    new_val += existing_val;
                }
                res.insert(ckey, new_val);
            }
        }
    }

    return res;
}

fn cats_n_lizards(animals: Vec<HashMap<&str, i32>>) -> Option<HashMap<&str, i32>> {
  match animals.len() {
      0 => None,
      1 => animals.into_iter().next(),
      _ => {
          let result = animals.iter().skip(1).enumerate().fold(HashMap::new(), |mut acc, (i, animal)| {
              if i == 0 {
                  if let Some(prev_animal) = animals.get(i) {
                      for (key, value) in animal {
                          if prev_animal.contains_key(key) {
                              let next_value = prev_animal.get(key).unwrap();
                              acc.insert(*key, value + next_value);
                          }
                      }
                  }
              } else {
                  for (key, value) in animal {
                      if acc.contains_key(key) {
                          let next_value = acc.get(key).unwrap();
                          acc.insert(key, value + next_value);
                      }
                  }
              }
              acc
          });

          if result.is_empty() {
              None
          } else {
              Some(result)
          }
      }
  }
}


fn main() {
    let dict1 = HashMap::from([("cats", 3), ("dogs", 5), ("birds", 2), ("lizards", 4)]);
    let dict2 = HashMap::from([("books", 5), ("lizards", 11), ("papers", 1), ("cats", 2)]);
    let dict3 = HashMap::from([("dogs", 8), ("cats", 5), ("lizards", 5)]);

    let res = solve_for_dicts(vec![dict1, dict2, dict3]);
    println!("res: {res:?}", res=res);
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::solve_for_dicts;
    use crate::cats_n_lizards;

    #[test]
    fn cats_n_lizards_1() {
        let dict1 = HashMap::from([("cats", 3), ("dogs", 5), ("birds", 2), ("lizards", 4)]);
        let dict2 = HashMap::from([("books", 5), ("lizards", 11), ("papers", 1), ("cats", 2)]);
        let dict3 = HashMap::from([("dogs", 8), ("cats", 5), ("lizards", 5)]);
        let all_dicts = vec![dict1, dict2, dict3];

        let res = solve_for_dicts(all_dicts);
        let expected = HashMap::from([("cats", 10), ("lizards", 20)]);
        assert_eq!(res, expected);
    }

    #[test]
    fn cats_n_lizards_2() {
        let dict1 = HashMap::from([("cats", 3), ("dogs", 5), ("birds", 2), ("lizards", 4)]);
        let dict2 = HashMap::from([("books", 5), ("lizards", 11), ("papers", 1), ("cats", 2)]);
        let dict3 = HashMap::from([("dogs", 8), ("cats", 5), ("lizards", 5)]);
        let all_dicts = vec![dict1, dict2, dict3];

        let res = cats_n_lizards(all_dicts).unwrap();
        let expected = HashMap::from([("cats", 10), ("lizards", 20)]);
        assert_eq!(res, expected);
    }
}