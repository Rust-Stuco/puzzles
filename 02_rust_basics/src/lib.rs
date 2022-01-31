// Structs
struct Color; // TODO: Task 1

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        todo!("Task 2");
    }

    fn add(&self, other: &Color) -> Color {
        // Implement this based off of the following additive color algorithm:
        // sqrt((x^2 + y^2) / 2)
        // applied for each channel.
        todo!("Task 3");
    }
}

// Slices
fn sum(xs: &[i32]) -> i32 {
    todo!("Task 4");
}

fn prefix_sums(xs: &mut [i32]) {
    todo!("Task 5");
}

// Enums
#[derive(Debug)]
enum Tree {
    Node(Box<Tree>, i32, Box<Tree>),
    Leaf(i32),
}

impl Tree {
    fn leftmost_leaf_iter(&self) -> &Tree {
        todo!("Task 6");
    }

    fn leftmost_leaf_recur(&self) -> &Tree {
        todo!("Task 7");
    }

    fn max_and_min(&self) -> (i32, i32) {
        todo!("Task 8");
    }

    fn sum(&self) -> i32 {
        todo!("Task 9");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1() {
        let forest_green = Color {
            red: 1,
            green: 81,
            blue: 34,
        };
        let orange = Color {
            red: 244,
            green: 151,
            blue: 0,
        };
    }

    #[test]
    fn task_2() {
        let red = Color::new(255, 0, 0);
        assert_eq!(red.red, 255);
        assert_eq!(red.green, 0);
        assert_eq!(red.blue, 0);

        let c = Color::new(1, 2, 3);
        assert_eq!(c.red, 1);
        assert_eq!(c.green, 2);
        assert_eq!(c.blue, 3);
    }

    #[test]
    fn task_3() {
        let a = Color::new(200, 50, 150);
        let b = Color::new(41, 20, 210);

        let c = a.add(&b);
        let d = b.add(&a);

        assert_eq!(c.red, d.red);
        assert_eq!(c.green, d.green);
        assert_eq!(c.blue, d.blue);

        assert_eq!(c.red, 144);
        assert_eq!(c.green, 38);
        assert_eq!(c.blue, 182);
    }

    #[test]
    fn task_4() {
        let square = [1, 4, 9, 16, 25, 36];
        assert_eq!(sum(&square), square.iter().sum());
        assert_eq!(sum(&square[3..]), square.iter().skip(3).sum());
        assert_eq!(sum(&square[1..4]), square.iter().skip(1).take(3).sum());

        let nums = [1, 2, 3, 4, 5, 6];
        assert_eq!(sum(&nums), nums.iter().sum());
        assert_eq!(sum(&nums[4..]), nums.iter().skip(4).sum());

        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn task_5() {
        let mut nums = [1, 2, 3, 4, 5];
        let expect = [1, 3, 6, 10, 15];
        prefix_sums(&mut nums);
        assert_eq!(nums, expect);

        let mut negs = [1, -1, 1, -1, 1, -1];
        let expect = [1, 0, 1, 0, 1, 0];
        prefix_sums(&mut negs);
        assert_eq!(negs, expect);
    }

    #[test]
    fn task_6() {
        let leftmost = Box::new(Tree::Leaf(3));
        let t = Tree::Node(
            Box::new(Tree::Node(leftmost, 2, Box::new(Tree::Leaf(5)))),
            1,
            Box::new(Tree::Node(
                Box::new(Tree::Leaf(-4)),
                0,
                Box::new(Tree::Leaf(3)),
            )),
        );

        match t.leftmost_leaf_iter() {
            Tree::Leaf(3) => (),
            n => panic!("Wrong tree node: {:?}", n),
        }

        assert_eq!(
            t.leftmost_leaf_iter() as *const _,
            t.leftmost_leaf_iter() as *const _
        );

        let t = Tree::Leaf(0);

        assert_eq!(t.leftmost_leaf_iter() as *const _, &t as *const _);
    }

    #[test]
    fn task_7() {
        let leftmost = Box::new(Tree::Leaf(3));
        let t = Tree::Node(
            Box::new(Tree::Node(leftmost, 2, Box::new(Tree::Leaf(5)))),
            1,
            Box::new(Tree::Node(
                Box::new(Tree::Leaf(-4)),
                0,
                Box::new(Tree::Leaf(3)),
            )),
        );

        match t.leftmost_leaf_recur() {
            Tree::Leaf(3) => (),
            n => panic!("Wrong tree node: {:?}", n),
        }

        assert_eq!(
            t.leftmost_leaf_recur() as *const _,
            t.leftmost_leaf_recur() as *const _
        );

        let t = Tree::Leaf(0);

        assert_eq!(t.leftmost_leaf_recur() as *const _, &t as *const _);
    }

    #[test]
    fn task_8() {
        let t = Tree::Node(
            Box::new(Tree::Node(
                Box::new(Tree::Leaf(3)),
                2,
                Box::new(Tree::Leaf(5)),
            )),
            1,
            Box::new(Tree::Node(
                Box::new(Tree::Leaf(-4)),
                0,
                Box::new(Tree::Leaf(3)),
            )),
        );

        assert_eq!(t.max_and_min(), (5, -4));
    }

    #[test]
    fn task_9() {
        let t = Tree::Node(
            Box::new(Tree::Node(
                Box::new(Tree::Leaf(3)),
                2,
                Box::new(Tree::Leaf(5)),
            )),
            1,
            Box::new(Tree::Node(
                Box::new(Tree::Leaf(-4)),
                0,
                Box::new(Tree::Leaf(3)),
            )),
        );

        assert_eq!(t.sum(), 10);
    }
}
