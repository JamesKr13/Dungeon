extern crate rand;
use super::traits::add_dp_place;
use ::rand::Rng;
use macroquad::prelude::*;
use std::format;
use std::time::SystemTime;

#[derive(Default)]
pub struct Question {
    pub answer: String,
    pub question: String,
    pub show: bool,
    pub user_answer: String,
}
impl Question {
    pub fn create(&mut self, q_type: &str) {
        self.user_answer = String::new();
        let mut rng = rand::thread_rng();
        let chance: usize = rng.gen_range(0..3);
        match chance {
            0 => {
                self.eigen_values_simple();
                Some(())
            }
            1 => {
                self.timestables();
                Some(())
            },
            2 => {self.cofactor_expansion_generation();
            Some(())}
            _ => None,
        };
    }
    fn check_answer(&self) -> bool {
        let mut answer: Vec<&str> = self.answer.split(',').collect();
        answer.sort_by(|a, b| b.cmp(a));
        let mut user_input: Vec<&str> = self.user_answer.split(',').collect();
        user_input.sort_by(|a, b| b.cmp(a));
        if user_input.iter().any(|&i| !i.contains('.')) {
            let mut _new_user_answer: Vec<String> = Vec::new();
            for arg_index in 0..user_input.len() {
                if !user_input[arg_index].contains('.') {
                    _new_user_answer.push(user_input[arg_index].to_string().add_dp())
                }
            }
            println!("{:#?}", user_input);
            println!("{:#?}", answer);
            return answer == _new_user_answer
    } else {
        return answer == user_input
    }
}
    fn eigen_values_simple(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_value1: i32 = rng.gen_range(0..10);
        let rand_value2: i32 = rng.gen_range(0..10);
        let rand_value3: i32 = rng.gen_range(0..10);
        let rand_value4: i32 = rng.gen_range(0..10);
        let m: f32 = (rand_value1 + rand_value4) as f32 / 2.;
        println!("{}", m);

        let p: f32 = (rand_value1 * rand_value4 - rand_value2 * rand_value3) as f32;
        println!("{}", p);
        println!("{}", m.powf(2.));
        self.question = format!(
            "Find the Eigen Value of \n[{} {}]\n[{} {}]\nInput without as dp",
            rand_value1, rand_value2, rand_value3, rand_value4
        );
        self.answer = format!(
            "{:.2},{:.2}",
            m + (m.powf(2.) - p).sqrt(),
            m - (m.powf(2.) - p).sqrt()
        );
        println!("{}", self.answer);
    }
    fn timestables(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_value1: i32 = rng.gen_range(-20..20);
        let rand_value2: i32 = rng.gen_range(-20..20);
        self.answer = format!("{}", (rand_value1 * rand_value2));
        self.question = format!("{} x {}", rand_value1, rand_value2);
    }
    #[must_use]
    pub fn compare_answer(&self, user_answer: &str) -> bool {
        self.answer.eq(user_answer)
    }
    fn cofactor_expansion(&self, matrix: &[[i32; 3]; 3]) -> i32 {
        let a = matrix[0][0];
        let b = matrix[0][1];
        let c = matrix[0][2];
        let d = matrix[1][0];
        let e = matrix[1][1];
        let f = matrix[1][2];
        let g = matrix[2][0];
        let h = matrix[2][1];
        let i = matrix[2][2];
        return a*(e*i-h*f) - b*(d*i-g*f) + c*(d*h-g*e)
    }
    fn cofactor_expansion_generation(&mut self) {
        let mut rng = rand::thread_rng();
        let values: [[i32; 3]; 3] = [[rng.gen_range(-20..20),rng.gen_range(-20..20),rng.gen_range(-20..20)],[rng.gen_range(-20..20),rng.gen_range(-20..20),rng.gen_range(-20..20)],[rng.gen_range(-20..20),rng.gen_range(-20..20),rng.gen_range(-20..20)]];
        self.answer = format!("{}", *&self.cofactor_expansion(&values) as f32);
        self.question = format!(
            "Find the Determinant of this matrix \n[{} {} {}]\n[{} {} {}]\n[{} {} {}]",
            values[0][0],
            values[0][1],
            values[0][2],
            values[1][0],
            values[1][1],
            values[1][2],
            values[2][0],
            values[2][1],
            values[2][2]
        );
    }
}

#[must_use]
pub fn ask_question(question: &Question, old_input: &String) -> String {
    let mut answer = (*old_input.clone()).to_string();
    let user_answer = answer.chars();
    let q: Vec<&str> = question.question.split("\n").collect();
    for line in 0..q.len() {
        draw_text(q[line], 100., 100.+50.*line as f32, 45., BLUE);
    }
    
    let s_answer = &question.user_answer[0..**[&question.user_answer.len(),&question.answer.split(',').collect::<Vec<&str>>()[0].len()].iter().min().unwrap()];
    draw_text(s_answer, 100., screen_height()-150., 45., BLUE);
    if s_answer.len() >= question.user_answer.len() {
    let entered_char = get_char_pressed();
    if entered_char.is_some() {
        answer = format!(
            "{}{}",
            String::from_iter(user_answer),
            entered_char.unwrap()
        );
    } else {
        answer = String::from_iter(user_answer);
    }
}

    if  is_key_pressed(KeyCode::Enter) {
        match question.check_answer() {
            false => return "false".to_string(),
            true => return "true".to_string(),
        };
    }
    answer
}
pub struct CountDown {
    start: SystemTime,
    end: u64,
}
impl CountDown {
    #[must_use]
    pub fn new_countdown(end: u64) -> Self {
        Self {
            start: SystemTime::now(),
            end,
        }
    }
    #[must_use]
    pub fn check(&self) -> bool {
        if self.start.elapsed().unwrap().as_secs() >= self.end {
            return false;
        }
        draw_text(
            &(self.end - self.start.elapsed().unwrap().as_secs()).to_string()[..],
            screen_width() - 75.,
            50.,
            50.,
            RED,
        );
        true
    }
}
