extern crate rand;
use ::rand::Rng;
use std::format;
use macroquad::ui::{hash, root_ui, widgets, Skin,Style};
use macroquad::prelude::*;

// macro_rules! format {
//     ($($arg:tt)*) => { ... };
// }
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
    fn eigen_values_simple(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_value1: i32 = rng.gen_range(-20..20);
        let rand_value2: i32 = rng.gen_range(-20..20);
        let rand_value3: i32 = rng.gen_range(-20..20);
        let rand_value4: i32 = rng.gen_range(-20..20);
        let m: i32 = (rand_value1+rand_value3)/2;
        let p: i32 = rand_value1*rand_value3 - rand_value2*rand_value3;
        self.question = format!("Find the Eigen Value of [{} {}]\n[{} {}]", rand_value1, rand_value2,rand_value3,rand_value4);
        self.answer = format!("{},{}", ((m+(m.pow(2)-p)) as f32).sqrt() as i32,((m-(m.pow(2)-p)) as f32).sqrt() as i32);
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
     widgets::Window::new(hash!("Question"), vec2(screen_width()*(1./8.),screen_height()-75.), vec2(300., 75.)).ui(&mut *root_ui(), |ui| {
        // ui.tree_node(hash!(), &question.question[..], |ui| {
            ui.label(None, &question.question[..]);
            ui.editbox(hash!("Question"), vec2(285., 165.), &mut answer);
        // });
    });
    // root_ui().move_window(hash!("Question"),vec2(screen_width()*(1./8.),screen_height()-75.));
return answer
}