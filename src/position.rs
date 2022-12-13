use bevy::prelude::Component;

#[derive(Component, Eq, PartialEq)]
pub struct Position(pub usize, pub usize);

impl Position {
    fn is_neighbor(&self, other: &Self) -> bool {
        if self == other {
            return false;
        }

        (-1..1)
            .zip(-1..1)
            .filter_map(|(x, y)| {
                if self.0 == 0 && x < 0 {
                    return None;
                }

                if self.1 == 0 && y < 0 {
                    return None;
                }

                Some(Position(
                    (self.0 as i32 + x).try_into().unwrap(),
                    (self.1 as i32 + y).try_into().unwrap(),
                ))
            })
            .any(|o| &o == other)
    }
}
