// TODO  NEED TO IMPLEMENT THE DATE TIME

use core::{result::Result, todo};
use std::error::Error;
use std::cmp::Ordering;

enum ExerciseType {
    Warmup, 
    Working,
    Drop,
}

enum Kind {
    Workout,
    Plan
}

struct Exercise {
    name: String,
    weight: u8,
    reps: u8,
    exercise_type: ExerciseType,
    goal_reps_low: Option<u8>,
    goal_reps_high: Option<u8>,
    rpe: Option<u8>,
    notes: Option<String>,
    completed: bool,
}

struct Workout {
    date: Option<String>,
    name: Option<String>,
    kind: Kind,
    exercises: Vec<Vec<Exercise>>,
}

impl Workout {
    fn new_workout(name: Option<String>) -> Self {
        Workout { date: Some("today".to_string()), name, kind: Kind::Workout, exercises: vec![] }
    }

    fn new_plan() -> Self {
        todo!()
    }

    fn workout_from_plan(plan: Workout) -> Self {
        todo!()
    }

    fn workout_from_workout(plan: Workout) -> Self {
        todo!()
    }

    fn plan_from_workout(workout: Workout) -> Self {
        todo!()
    }

    fn add_set(&mut self, exercise: Exercise, exercise_index: usize) -> Result<(), Box<dyn Error>> {


        if exercise_index >= self.exercises.len() {
            panic!("Outside of exercise range");
        } 

        if self.exercises[exercise_index].len() > 0{
            if exercise.name != self.exercises[exercise_index].last().expect("should be available").name {
                panic!("non matching exercises");
            }
        }

        self.exercises[exercise_index].push(exercise);
        Ok(())
    }

    fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(vec![exercise]);
    }

    fn remove_set(&mut self,exercise_number: usize, set_number: usize) {
        if exercise_number >= self.exercises.len() || set_number >= self.exercises[exercise_number].len(){
            panic!("Outside of set range");
        } else {
            self.exercises.remove(set_number);
        }
    }

    fn remove_exercise(&mut self, exercise_number: usize ) {
        if exercise_number >= self.exercises.len() {
            panic!("Outside of set range");
        } else {
            self.exercises.remove(exercise_number);
        }
    }

    fn update_set(&mut self, exercise:Exercise, set_number: usize, exercise_number: usize) {
        if exercise_number >= self.exercises.len() || set_number >= self.exercises[exercise_number].len(){
            panic!("Outside of set range");
        } else {
            self.exercises[exercise_number][set_number] = exercise;
        }
    }
}
