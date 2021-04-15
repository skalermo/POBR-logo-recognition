pub mod opencv_allowed;

pub fn adds_one(number: i32) -> i32 {
    number + 1
}

#[cfg(test)]
#[path = "../tests/tests.rs"]
mod tests;