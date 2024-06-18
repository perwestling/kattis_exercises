use std::{io, io::BufRead, collections::{HashMap, HashSet, BTreeMap}};

pub fn main() {
    let mut result : i64 = 0;
    let mut names : HashSet<String> = HashSet::new();
    let mut map : HashMap<String, String> = HashMap::new();
    let mut line = String::new();
    {
        let mut cursor = io::stdin().lock();

        _ = cursor.read_line(&mut line).expect("Failed to read line");
        let line_count = line.trim().parse::<u32>().unwrap();

        // Read strings
        for _ in 0..line_count {
            line.clear();
            _ = cursor.read_line(&mut line).expect("Failed to read line");
            let words = line.split(" ").collect::<Vec<&str>>();
            let from : String = words[0].trim().to_string();
            let to : String = words[1].trim().to_string();

            // Store both from and to in the set, to ensure that all names
            // appearing in relations appear in this collection, and are unique.
            names.insert(from.clone());
            names.insert(to.clone());

            // Create directed relation
            map.insert(from.clone(), to.clone());
            println!("'{}'->'{}'", from, to);
        }
    }

    if names.len() % 2 == 1 {
        // If we create pairs where both names are unique, there will always be someone left out.
        println!("-1");
        return;
    }

    // Remove all self lovers, counting one for each
    // Collect keys that need to be removed
    let keys_to_remove: Vec<String> = map
        .iter()
        .filter(|(k, v)| k == v)
        .map(|(k, _)| k.clone())
        .collect();
    
    // Remove the collected keys, and increase operation count as this will require at least one operation.
    for key in keys_to_remove {
        println!("Remove {}", &key);
        map.remove(&key);
        result += 1;
    }

    // Remove pairs, that is, where a->b and b->a. Remove a and b from names and map, to decrease scope.
    let mutual_mappings = find_mutual_mappings(map);
    for fullfilled = names

    // Go through values and store excesses per name (number of times above one)
    let excesses = count_excesses(map);

    result += excesses.values().sum::<i64>();
    
    // This sum will be number of changes needed
    println!("{}", result);
    ()
}

fn find_mutual_mappings(map: &HashMap<String, String>) -> HashSet<(String, String)> {
    let mut mutual_mappings = HashSet::new();

    for (key, value) in map.iter() {
        if Some(key) == map.get(value) {
            mutual_mappings.insert(key.clone());
            mutual_mappings.insert(value.clone());
        }
    }

    mutual_mappings
}

fn count_excesses(map: HashMap<String, String>) -> BTreeMap<String, i64> {
    let mut excesses : BTreeMap<String, i64> = BTreeMap::new();

    for value in map.values() {
        *excesses.entry(value.clone()).or_insert(-1) += 1;
    }

    excesses
}

// fn radix_sort_descending(arr: &mut [u32]) {
//     let max_num = match arr.iter().max() {
//         Some(&max) => max,
//         None => return,
//     };

//     let mut exp = 1;
//     while max_num / exp > 0 {
//         counting_sort_descending(arr, exp);
//         exp *= 10;
//     }
// }

// fn counting_sort_descending(arr: &mut [u32], exp: u32) {
//     let mut output = vec![0; arr.len()];
//     let mut count = [0; 10];  // Use fixed-size array for counts

//     // Store count of occurrences in count[]
//     for &number in arr.iter() {
//         let index = (number / exp % 10) as usize;
//         count[index] += 1;
//     }

//     // Change count[i] so that count[i] contains the actual
//     // position of this digit in output[]
//     for i in (0..9).rev() {
//         count[i] = count[i + 1] + count[i];
//     }

//     // Build the output array in descending order
//     for &number in arr.iter().rev() {
//         let index = (number / exp % 10) as usize;
//         output[count[index] - 1] = number;
//         count[index] -= 1;
//     }

//     // Copy the output array to arr[], so that arr[] now
//     // contains sorted numbers according to the current digit
//     arr.copy_from_slice(&output);
// }

#[cfg(test)]
mod tests;
