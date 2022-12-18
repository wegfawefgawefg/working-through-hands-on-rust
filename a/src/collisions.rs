use glam::IVec2;

pub struct Bounds {
    top_left: IVec2,
    bottom_right: IVec2,
}

trait Collidable {
    fn get_bounds(&self) -> Bounds;
    fn is_collided(&self, other: &Bounds) -> bool {
        let bounds = self.get_bounds();

        if other.bottom_right.x < bounds.top_left.x
            || other.top_left.x > bounds.bottom_right.x
            || other.bottom_right.y < bounds.top_left.y
            || other.top_left.y > bounds.bottom_right.y
        {
            return false;
        }
        true
    }
}
