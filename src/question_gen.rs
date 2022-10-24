extern crate rand;
use ::rand::Rng;
use std::format;
use macroquad::prelude::*;
use std::time::SystemTime;
use super::traits::add_dp_place;
// macro_rules! format {
//     ($($arg:tt)*) => { ... };
// }
use super::player::draw_bar;

#[derive(Default)]
pub struct Question {
    pub answer: String,
    pub question: String,
    pub show: bool,
    pub user_answer: String,
}
impl Question {
    pub fn create(&mut self,q_type: &str) {
        match q_type {
            "eigen value" => Some(self.eigen_values_simple()),
            "simple mult" => Some(self.timestables()),
            _ => None,
        };
    //     if !question.is_none(){
    //     self.answer = question.unwrap().answer;
    //     self.question = question.unwrap().question;
    // }
}   
    fn check_answer(&self) -> bool {
        let mut answer: Vec<&str> = self.answer.split(",").collect();
        answer.sort_by(|a, b| b.cmp(&a));
        let mut user_input: Vec<&str> = self.user_answer.split(",").collect();
        user_input.sort_by(|a, b| b.cmp(&a));
        if user_input.iter().any(|&i| !i.contains(".")) {
            let mut _new_user_answer: Vec<String> = Vec::new();
            for arg_index in 0..user_input.len() {
                if !user_input[arg_index].contains('.') {
                    _new_user_answer.push(user_input[arg_index].to_string().add_dp())
                    

                }
            }
            println!("{:#?}", _new_user_answer);
            println!("{:#?}", answer);
            return !matches!(answer,_new_user_answer)
        }
        println!("{:#?}", user_input);
        println!("{:#?}", answer);
        return !matches!(answer,user_input)
    } 
    fn eigen_values_simple(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_value1: i32 = rng.gen_range(0..10);
        let rand_value2: i32 = rng.gen_range(0..10);
        let rand_value3: i32 = rng.gen_range(0..10);
        let rand_value4: i32 = rng.gen_range(0..10);
        let m: f32 = (rand_value1+rand_value4) as f32/2.;
        println!("{}", m);
       
        let p: f32 = (rand_value1*rand_value4- rand_value2*rand_value3) as f32;
        println!("{}", p);
        println!("{}", m.powf(2.));
        self.question = format!("Find the Eigen Value of [{} {}]\n[{} {}] Input without as dp", rand_value1, rand_value2,rand_value3,rand_value4);
        self.answer = format!("{:.2},{:.2}", m+(m.powf(2.) - p).sqrt(), m-(m.powf(2.) - p).sqrt());
        println!("{}", self.answer);
    }
    fn timestables(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_value1: i32 = rng.gen_range(-20..20);
        let rand_value2: i32 = rng.gen_range(-20..20);
        self.answer= format!("{}",(rand_value1*rand_value2));
        self.question= format!("{} x {}", rand_value1, rand_value2);
    }
    pub fn compare_answer(&self,user_answer: &str) -> bool {
        return self.answer.eq(user_answer)
    }
    fn cofactor_expansion(&self,matrix: &[[i32;3];3]) -> i32{
        return matrix[0][0]*(matrix[2][1]*matrix[1][2] - matrix[1][2]*matrix[1][1]) - matrix[0][1]*(matrix[2][0]*matrix[1][2] - matrix[1][2]*matrix[1][0]) + matrix[0][2]*(matrix[2][1]*matrix[1][0] - matrix[1][0]*matrix[1][1])
    }
    fn cofactor_expansion_generation(&mut self) {
        let mut rng = rand::thread_rng();
        let values: [[i32;3];3] = [[rng.gen_range(-20..20);3];3];
        self.answer = format!("{}",&self.cofactor_expansion(&values));
        self.question = format!("[{} {} {}]\n[{} {} {}]\n[{} {} {}]", values[0][0],values[0][1],values[0][2],values[1][0],values[1][1],values[1][2],values[2][0],values[2][1],values[2][2]);
     } 
}

pub fn ask_question(question: &Question, old_input: &String) -> String {
    let mut answer = (*old_input.clone()).to_string();
    let mut user_answer = answer.chars();
    draw_text(&question.question[..],100.,100.,45., BLUE);
    let entered_char = get_char_pressed();
    if !entered_char.is_none() {
        answer = format!("{}{}",String::from_iter(user_answer),entered_char.unwrap().to_string());
    } else {
        answer = String::from_iter(user_answer);
    }
    
    if is_key_pressed(KeyCode::Enter) {
        match question.check_answer() {
           false => return "false".to_string(),
           true => return "true".to_string()
        };
    }
    answer
}
pub struct CountDown {
    start:SystemTime,
    end:u64,
}
impl CountDown{
    pub fn new_countdown(end: u64) -> Self {
        Self {
            start: SystemTime::now(),
            end: end
        }
    }
    pub fn check(&self) -> bool{
        if self.start.elapsed().unwrap().as_secs() >= self.end {
            return false
        }
        draw_text(&(self.end - self.start.elapsed().unwrap().as_secs()).to_string()[..],screen_width()-75.,50.,50.,RED);
        return true
    }
}