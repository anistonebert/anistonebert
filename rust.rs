use std::f64::consts::PI;
enum Shape<T> {Circle(T), Triangle(T, T), Rectangle(T, T)}
trait AreaCalculation<T> {fn calculate_area(&self) -> f64;}
impl<T: Into<f64> + Copy> AreaCalculation<T> for Shape<T> {
    fn calculate_area(&self) -> f64 {
        match self {
            Shape::Circle(d) => {
                let r = (*d).into() / 2.0;
                PI * r.powf(2.0)
            }
            Shape::Triangle(b, h) => {
                let b = (*b).into();
                let h = (*h).into();
                0.5 * b * h
            }
            Shape::Rectangle(w, l) => {
                let w = (*w).into();
                let l = (*l).into();
                w * l}}}}
fn main() {
    let triangle = Shape::Triangle(24_u8, 24_u8);
    let triangle_area = triangle.calculate_area();
    let rectangle = Shape::Rectangle(12_u8, 24_u8);
    let rectangle_area = rectangle.calculate_area();
    let circle = Shape::Circle(45_u8);
    let circle_area = circle.calculate_area();
    println!(
        "Triangle with base of 24 and height of 24 has area {:.5}",
        triangle_area
    );
    println!(
        "Rectangle with width of 12 and length of 24 has area {:.5}",
        rectangle_area
    );
    println!(
        "Circle with diameter of 45 has area {:.5}",
        circle_area
    );
}
