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

    let d1 = dicts[0].clone();
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
}