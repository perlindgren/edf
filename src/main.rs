#[derive(Debug)]
struct Task {
    _id: &'static str,
    abs_dl: i8,
}

#[derive(Debug, Default)]
struct Edf {
    now: i8,
    tasks: Vec<Task>,
}

impl Edf {
    fn pend(&mut self, rel_dl: i8, id: &'static str) {
        self.tasks.push(Task {
            _id: id,
            abs_dl: rel_dl.wrapping_add(self.now),
        })
    }

    fn schedule(&mut self, tick: i8) {
        let mut min = None;
        self.now = self.now.wrapping_add(tick);

        let mut best_index = None;
        for (i, task) in self.tasks.iter().enumerate() {
            let rd = task.abs_dl.wrapping_sub(self.now);
            if let Some(m) = min {
                if rd < m {
                    min = Some(rd);
                    best_index = Some(i);
                }
            } else {
                min = Some(rd);
                best_index = Some(i);
            }
        }

        if let Some(i) = best_index {
            println!(
                "now {} min {:?} task {:?},",
                self.now,
                min,
                self.tasks.get(i)
            );
            self.tasks.remove(i);
        } else {
            println!("none");
        }
    }
}

fn main() {
    let mut edf = Edf::default();

    for _ in 0..20 {
        edf.pend(10, "t1");
        edf.pend(127, "t2");
        edf.schedule(20);
        edf.pend(110, "t3");
        edf.schedule(0);
        edf.schedule(50);
        edf.schedule(0);
    }
}
