// this point supports different types of X1 and Y1 (float or integer)
pub struct Point<X1, Y1> {
    pub x: X1,
    pub y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    // this function returns Point wit mixed coordinates with other point given in params 
    pub fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}