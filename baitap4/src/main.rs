use std::collections::HashMap;
#[derive(Debug)]

struct School {
    students: HashMap<String, u32>,
}

impl School {
    //Tạo 1 function new() khởi tạo rỗng ban đầu cho danh sách
    fn new_school() ->  Self {
        School { students: HashMap::new()}
    }

    //Có thể thêm thông tin của sinh viên gồm có tên và điểm
    fn add_student(&mut self,name: String, score: u32) {
            self.students.insert(name, score);
    }

    //Liệt kê các điểm số hiện tại mà trường đã cập nhập (điểm số nên tăng dần và ko có duplicate)
    fn grades(&self) -> Vec<u32> {
        let mut result: Vec<u32> = vec![];
        for t in &self.students {
            result.push(*t.1);
        }
        result.sort();
        result.dedup();
        result
    }

    // Liệt kê danh sách các học sinh có cùng 1 điểm số
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut result1: Vec<String> = vec![];
        for t in &self.students {
            if *t.1 == grade {
                result1.push(t.0.clone())
            }
        }
        result1.sort();
        result1

    }
}

fn main() {
    let mut school1 = School::new_school();

    School::add_student(&mut school1, "Won".to_string(), 9);

    School::add_student(&mut school1, "David".to_string(), 1);

    School::add_student(&mut school1, "Suri".to_string(), 5);

    School::add_student(&mut school1, "TriCao".to_string(), 5);

    School::add_student(&mut school1, "Alice".to_string(), 9);

    School::add_student(&mut school1, "Bob".to_string(), 9);

    School::add_student(&mut school1, "Charlie".to_string(), 9);

    School::add_student(&mut school1, "Zenta".to_string(), 9);

    println!("This is the list of student in School1 : {:#?}", school1.students);

    //Test case 2: 
    println!("{:?}", School::grades(&school1));


    //Test case 3:
    println!("{:?}", School::grade(&school1, 9));

}
