use day14test::Polyline;

fn main() {
    for line in include_str!("../input.txt").lines() {
        let polyline = Polyline::parse(line);
        println!("{polyline:?}");
        for p in polyline.path_points() {
            println!("{p:?}");
        }
    }
}