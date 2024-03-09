use nannou::rand::random_range;

pub struct RandomStepRange {
    end: i32,
    max_step: i32,
    value: i32,
    step: i32,
}

impl RandomStepRange {
    pub fn new(start: i32, end: i32, max_step: i32) -> Self {
        Self {
            end,
            max_step,
            step: 1,
            value: start,
        }
    }
}

impl Iterator for RandomStepRange {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let return_value = self.value;
        self.value = self.value + self.step;
        if self.value > self.end {
            return None;
        }

        let next_step = random_range(1, self.max_step);
        self.step = if return_value + self.step > self.end {
            self.end - return_value
        } else {
            next_step
        };

        Some((return_value, self.step))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_finishes() {
        let results: Vec<_> = RandomStepRange::new(0, 10, 4).collect();
        assert_eq!(results, vec![])
    }
}
