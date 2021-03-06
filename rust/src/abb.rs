use std::{collections::HashMap};

pub trait Abb<T> {
    fn abb(&mut self, c:T) -> String ;
}
pub struct AbbImpl {
    month_word: Vec<String>,
    month_num: Vec<i32>,
}

impl Abb<i32> for AbbImpl {
    fn abb(&mut self, c:i32) -> String {
        let months: HashMap<_, _> = self.month_num.iter().zip(self.month_word.iter()).collect();
        let month = months.get(&c);
        let result = format!("{:?}", month);
        return result
    }
}

impl Abb<String> for AbbImpl {
    fn abb(&mut self, c:String) -> String {
        let months: HashMap<_, _> = self.month_word.iter().zip(self.month_num.iter()).collect();
        let month = months.get(&c);
        let result = format!("{:?}", month);
        return result
    }
}

pub fn abb<T>(data:T) -> String where AbbImpl:Abb<T> {
    let month_word = vec![
        String::from("Jan"), String::from("Feb"), String::from("Mar"),
        String::from("Apr"), String::from("May"), String::from("Jun"),
        String::from("Jul"), String::from("Aug"), String::from("Sep"),
        String::from("Oct"), String::from("Nov"), String::from("Dec")
    ];
    let month_num = (1..13).collect::<Vec<i32>>();
    let mut abb = AbbImpl{month_word, month_num};
    let result = Abb::abb(&mut abb, data)
        .replace("Some(", "")
        .replace(")", "")
        .replace("\"", "")
        .replace("None", "");
    return result
}
