fn main() {

let mut total_cohort_students = Counter::new();

total_cohort_students.increase_count();
total_cohort_students.reset_count(); 
total_cohort_students.increase_count_by(200);
total_cohort_students.decrease_count(); 
total_cohort_students.decrease_count_by(20); 

println!("The total registered students are : {} ", total_cohort_students.value())


}


    pub struct Counter {
            value : i32
        }

        impl Counter {

            fn new() -> Self {
                Counter {
                    value : 0 
                }
            }

            fn increase_count(&mut self) {
                self.value += 1;
            }

            fn decrease_count(&mut self) {
                if self.value <= 0 {return};
                self.value -=1; 
            }

            fn reset_count(&mut self) {
                self.value = 0;
            }

            fn value(&self) -> i32 {
                self.value
            }

            fn increase_count_by (&mut self, increase_by: i32) {
                self.value = self.value + increase_by;
            }

            fn decrease_count_by (&mut self, decrease_by: i32) {
                if self.value - decrease_by < 0 {
                    println!("The Inputed value is more than the available count"); 
                } else {
                    self.value = self.value - decrease_by;
                }
            }

     }
