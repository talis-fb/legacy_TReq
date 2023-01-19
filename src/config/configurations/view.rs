static MAX_GROW: i32 = 10;

#[derive(Clone)]
pub struct ViewConfig {
    dimension_horizontal_blocks: (i32, i32),
}
impl ViewConfig {
    pub fn init() -> Self {
        Self {
            dimension_horizontal_blocks: (1, 1),
        }
    }

    fn restart_if_reach_max(&mut self) {
        let (left, right) = self.dimension_horizontal_blocks;
        if left % 2 == 0 && right % 2 == 0 {
            self.dimension_horizontal_blocks = (left / 2, right / 2);
        }
    }

    pub fn grow_right_block(&mut self) {
        let (left, right) = &mut self.dimension_horizontal_blocks;
        if *right < MAX_GROW {
            *right += 1;
        }
        self.restart_if_reach_max();
    }

    pub fn grow_left_block(&mut self) {
        let (left, right) = &mut self.dimension_horizontal_blocks;
        if *left < MAX_GROW {
            *left += 1;
        }
        self.restart_if_reach_max();
    }

    pub fn get_dimension_percentage(&self) -> (i32, i32) {
        let left = self.dimension_horizontal_blocks.0 as f32;
        let right = self.dimension_horizontal_blocks.1 as f32;

        let total = right + left;
        let mut percent_left = (left * 100.0) / total;
        let mut percent_right = (right * 100.0) / total;

        if percent_left.fract() < percent_right.fract() {
            percent_left = percent_left.floor();
            percent_right = percent_right.ceil();
        } else {
            percent_left = percent_left.ceil();
            percent_right = percent_right.floor();
        }

        (percent_left as i32, percent_right as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_grow_correct() {
        let mut view = ViewConfig::init();
        assert_eq!(view.dimension_horizontal_blocks.0, 1);
        assert_eq!(view.dimension_horizontal_blocks.1, 1);

        view.grow_right_block();
        assert_eq!(view.dimension_horizontal_blocks.0, 1);
        assert_eq!(view.dimension_horizontal_blocks.1, 2);

        view.grow_right_block();
        assert_eq!(view.dimension_horizontal_blocks.0, 1);
        assert_eq!(view.dimension_horizontal_blocks.1, 3);

        view.grow_left_block();
        assert_eq!(view.dimension_horizontal_blocks.0, 2);
        assert_eq!(view.dimension_horizontal_blocks.1, 3);
    }

    #[test]
    fn should_grow_only_until_10() {
        let mut view = ViewConfig::init();
        view.dimension_horizontal_blocks.0 = 3; // Left
        view.dimension_horizontal_blocks.1 = 3; // Right

        view.grow_left_block(); // to 4
        view.grow_left_block(); // to 5
        view.grow_left_block(); // to 6
        view.grow_left_block(); // to 7
        view.grow_left_block(); // to 8
        view.grow_left_block(); // to 9
        view.grow_left_block(); // to 10
        assert_eq!(view.dimension_horizontal_blocks.0, 10);
        view.grow_left_block(); // to 10 Again because max
        assert_eq!(view.dimension_horizontal_blocks.0, 10);

        view.grow_right_block(); // to 4
        view.grow_right_block(); // to 5
        view.grow_right_block(); // to 6
        view.grow_right_block(); // to 7
        view.grow_right_block(); // to 8
        view.grow_right_block(); // to 9
        view.grow_right_block(); // to 10
        assert_eq!(view.dimension_horizontal_blocks.1, 10);
        view.grow_right_block(); // to 10 Again because max
        assert_eq!(view.dimension_horizontal_blocks.1, 10);
    }

    #[test]
    fn should_get_dimension_by_percentage() {
        let mut view = ViewConfig::init();
        assert_eq!(view.get_dimension_percentage(), (50, 50)); // 1 1

        view.dimension_horizontal_blocks = (3, 2);
        assert_eq!(view.get_dimension_percentage(), (60, 40));

        view.dimension_horizontal_blocks = (4, 6);
        assert_eq!(view.get_dimension_percentage(), (40, 60));

        view.dimension_horizontal_blocks = (2, 5);
        assert_eq!(view.get_dimension_percentage(), (29, 71));

        view.dimension_horizontal_blocks = (7, 5);
        assert_eq!(view.get_dimension_percentage(), (58, 42));
    }
}
