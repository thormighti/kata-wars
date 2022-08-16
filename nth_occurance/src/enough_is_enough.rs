use std::collections::HashMap;

//little program to delete the nth occurances
pub fn enough(arr:&[u8], mut n:u8) -> Vec<u8>{
    let mut map = HashMap::new();

    let mut vec:Vec<u8> = Vec::new();

    for item in arr {
        let count = map.entry(item).or_insert(0);
        if count >= &mut n{
          continue; //skip if it passes the occurance

        }
        vec.push(*item);
        
        *count+=1;
    }
    vec

}
