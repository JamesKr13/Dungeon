extern crate rand;
use super::traits::add_dp_place;
use ::rand::Rng;
use macroquad::prelude::*;
use std::format;
use std::time::SystemTime;
use macroquad::ui::{hash, root_ui, widgets, Skin, Style};
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
        if self.answer == self.user_answer {
            return true
        }
        if self.user_answer.len() == 0 {
            return false
        }
        let mut answer: Vec<&str> = self.answer.split(',').collect();
        answer.sort_by(|a, b| b.cmp(a));
        let mut user_input: Vec<&str> = self.user_answer.split(',').collect();
        println!("{:#?} = {:#?}", answer, user_input);
        user_input.sort_by(|a, b| b.cmp(a));
        if user_input.iter().any(|&i| !i.contains('.')) {
            let mut _new_user_answer: Vec<String> = Vec::new();
            for arg_index in 0..user_input.len() {
                if !user_input[arg_index].contains('.') {
                    _new_user_answer.push(user_input[arg_index].to_string().add_dp())
                }
            }
            if _new_user_answer.len() == 2{
                return (answer.iter().any(|ans| ans.eq(&_new_user_answer[0]) || ans.eq(&_new_user_answer[1])))
            } else {
                return (answer.iter().any(|ans| ans.eq(&_new_user_answer[0])))
            }
            
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

        let p: f32 = (rand_value1 * rand_value4 - rand_value2 * rand_value3) as f32;
        self.question = format!(
            "Find a Eigen Value for the Matric \n[{} {}]\n[{} {}]\nInput 2 dp without rounding",
            rand_value1, rand_value2, rand_value3, rand_value4
        );
        self.answer = format!(
            "{:.2},{:.2}",
            m + (m.powf(2.) - p).sqrt(),
            m - (m.powf(2.) - p).sqrt()
        );
    }
    fn timestables(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_value1: i32 = rng.gen_range(-20..20);
        let rand_value2: i32 = rng.gen_range(-20..20);
        println!("{}", self.answer);
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
    let mut each_line: Vec<String> = Vec::new();
    let each_word: Vec<String> = question.question.split("\n").map(str::to_string).collect(); 
    println!("{}",question.question );
    root_ui().window(
        hash!("Question"),
        vec2((screen_width())/2. -200., screen_height()-150.),
        vec2(400., 150.),
        |ui| {
            let mut line_tally= 1;
            for line in &each_word {
                        widgets::Label::new(line.clone()).position(vec2(0., (line_tally) as f32 * 25.))
                        .ui(ui);
                        line_tally += 1;
                        ui.separator();
                        ui.move_window(hash!("Question"),vec2((screen_width())/2.-200., screen_height()-150.));
                    }
                }
    );
    // root_ui().window(
    //     hash!("Answer"),
    //     vec2((screen_width()-200.)/2., screen_height()-175.),
    //     vec2(200., 50.),
    //     |ui| {
    //         widgets::Label::new(question.user_answer.clone()).position(vec2(0., 0.,))
    //         .ui(ui);
    //         ui.separator();
    //         ui.move_window(hash!("Question"),vec2((screen_width()-200.)/2., screen_height()-175.));
    //             }
    // );
    // draw_text(&question.user_answer,(screen_width()-400.)/2., screen_height()-275.,30.,WHITE);
    println!("{}", question.user_answer);
    let entered_char = get_char_pressed();
    if entered_char.is_some() && "1234567890.,-+".contains(entered_char.unwrap()) {
        answer = format!(
            "{}{}",
            String::from_iter(user_answer),
            entered_char.unwrap()
        );
    } else {
        answer = String::from_iter(user_answer);
    }

    if  is_key_pressed(KeyCode::Enter) {
        println!("{} = {} is {}", question.user_answer, question.answer, question.check_answer());
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
