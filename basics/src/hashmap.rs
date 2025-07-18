use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    println!("{:?}", scores.get("Red"));
    let x = scores.get("Blue");
    if x.is_none() {
        println!("x is none");
    } else if x.is_some() {
        println!("x is some: {}", x.unwrap());
    }
    //update value of "Yellow"
    let y = scores.get_mut("Yellow");
    if y.is_some() {
        *y.unwrap() += 1;
    }
    println!("{:?}", scores);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_hashmap_creation_and_insertion() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        
        assert_eq!(scores.len(), 2);
        assert_eq!(scores.get("Blue"), Some(&10));
        assert_eq!(scores.get("Yellow"), Some(&50));
    }

    #[test]
    fn test_hashmap_get_existing_key() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        
        let x = scores.get("Blue");
        assert!(x.is_some());
        assert_eq!(x.unwrap(), &10);
    }

    #[test]
    fn test_hashmap_get_non_existing_key() {
        let scores: HashMap<String, i32> = HashMap::new();
        
        let x = scores.get("Red");
        assert!(x.is_none());
        assert_eq!(x, None);
    }

    #[test]
    fn test_hashmap_is_some_is_none() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        
        let existing = scores.get("Blue");
        let non_existing = scores.get("Red");
        
        assert!(existing.is_some());
        assert!(!existing.is_none());
        
        assert!(non_existing.is_none());
        assert!(!non_existing.is_some());
    }

    #[test]
    fn test_hashmap_get_mut_and_update() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Yellow"), 50);
        
        let original_value = *scores.get("Yellow").unwrap();
        
        let y = scores.get_mut("Yellow");
        assert!(y.is_some());
        *y.unwrap() += 1;
        
        assert_eq!(scores.get("Yellow"), Some(&(original_value + 1)));
        assert_eq!(scores.get("Yellow"), Some(&51));
    }

    #[test]
    fn test_hashmap_get_mut_non_existing_key() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        
        let y = scores.get_mut("NonExisting");
        assert!(y.is_none());
    }

    #[test]
    fn test_hashmap_multiple_insertions() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        
        scores.insert(String::from("Team1"), 100);
        scores.insert(String::from("Team2"), 200);
        scores.insert(String::from("Team3"), 300);
        
        assert_eq!(scores.len(), 3);
        assert_eq!(scores.get("Team1"), Some(&100));
        assert_eq!(scores.get("Team2"), Some(&200));
        assert_eq!(scores.get("Team3"), Some(&300));
    }

    #[test]
    fn test_hashmap_overwrite_existing_key() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 20);
        
        assert_eq!(scores.len(), 1);
        assert_eq!(scores.get("Blue"), Some(&20));
    }

    #[test]
    fn test_hashmap_empty_state() {
        let scores: HashMap<String, i32> = HashMap::new();
        
        assert_eq!(scores.len(), 0);
        assert!(scores.is_empty());
        assert_eq!(scores.get("AnyKey"), None);
    }

    #[test]
    fn test_hashmap_contains_key() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        
        assert!(scores.contains_key("Blue"));
        assert!(!scores.contains_key("Red"));
    }

    #[test]
    fn test_hashmap_remove_key() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        
        assert_eq!(scores.len(), 2);
        
        let removed = scores.remove("Blue");
        assert_eq!(removed, Some(10));
        assert_eq!(scores.len(), 1);
        assert!(!scores.contains_key("Blue"));
        
        let not_removed = scores.remove("Red");
        assert_eq!(not_removed, None);
        assert_eq!(scores.len(), 1);
    }

    #[test]
    fn test_hashmap_clear() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        
        assert_eq!(scores.len(), 2);
        
        scores.clear();
        assert_eq!(scores.len(), 0);
        assert!(scores.is_empty());
        assert_eq!(scores.get("Blue"), None);
        assert_eq!(scores.get("Yellow"), None);
    }

    #[test]
    fn test_hashmap_iteration() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        
        let mut total = 0;
        let mut count = 0;
        
        for (key, value) in &scores {
            total += value;
            count += 1;
            assert!(key == "Blue" || key == "Yellow");
            assert!(*value == 10 || *value == 50);
        }
        
        assert_eq!(count, 2);
        assert_eq!(total, 60);
    }

    #[test]
    fn test_hashmap_keys_and_values() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        
        let keys: Vec<&String> = scores.keys().collect();
        let values: Vec<&i32> = scores.values().collect();
        
        assert_eq!(keys.len(), 2);
        assert_eq!(values.len(), 2);
        
        assert!(keys.contains(&&String::from("Blue")));
        assert!(keys.contains(&&String::from("Yellow")));
        assert!(values.contains(&&10));
        assert!(values.contains(&&50));
    }

    #[test]
    fn test_hashmap_entry_api() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        
        // Insert if not present
        scores.entry(String::from("Blue")).or_insert(10);
        assert_eq!(scores.get("Blue"), Some(&10));
        
        // Don't overwrite existing value
        scores.entry(String::from("Blue")).or_insert(20);
        assert_eq!(scores.get("Blue"), Some(&10));
        
        // Update existing value
        let entry = scores.entry(String::from("Blue")).or_insert(0);
        *entry += 5;
        assert_eq!(scores.get("Blue"), Some(&15));
    }

    #[test]
    fn test_hashmap_with_different_types() {
        let mut map: HashMap<i32, String> = HashMap::new();
        map.insert(1, String::from("one"));
        map.insert(2, String::from("two"));
        
        assert_eq!(map.get(&1), Some(&String::from("one")));
        assert_eq!(map.get(&2), Some(&String::from("two")));
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_hashmap_multiple_mutations() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Team1"), 100);
        scores.insert(String::from("Team2"), 200);
        
        // Multiple mutations
        if let Some(team1_score) = scores.get_mut("Team1") {
            *team1_score += 50;
        }
        
        if let Some(team2_score) = scores.get_mut("Team2") {
            *team2_score -= 25;
        }
        
        assert_eq!(scores.get("Team1"), Some(&150));
        assert_eq!(scores.get("Team2"), Some(&175));
    }

    #[test]
    fn test_hashmap_debug_output() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        
        let debug_str = format!("{:?}", scores);
        assert!(debug_str.contains("Blue"));
        assert!(debug_str.contains("10"));
        assert!(debug_str.starts_with("{"));
        assert!(debug_str.ends_with("}"));
    }
}
