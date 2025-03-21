use std::collections::HashMap;
use std::ops::AddAssign;

/// MP W D L P
struct Point(i32, i32, i32, i32, i32);

impl Point {
    fn new() -> Self {
        Point(0, 0, 0, 0, 0)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
        self.4 += rhs.4;
    }
}

pub fn tally(match_results: &str) -> String {
    let mut map: HashMap<String, Point> = HashMap::new();

    let fun = |team: &str, map: &mut HashMap<String, Point>, point: Point| {
        *map.entry(team.to_string()).or_insert(Point::new()) += point;
    };

    for line in match_results.lines() {
        let parts: Vec<&str> = line.split(";").collect();
        let (team1, team2, res) = (parts[0], parts[1], parts[2]);

        let (point1, point2) = if res == "win" {
            (Point(1, 1, 0, 0, 3), Point(1, 0, 0, 1, 0))
        } else if res == "draw" {
            (Point(1, 0, 1, 0, 1), Point(1, 0, 1, 0, 1))
        } else if res == "loss" {
            (Point(1, 0, 0, 1, 0), Point(1, 1, 0, 0, 3))
        } else {
            panic!("错误的输入！")
        };

        fun(team1, &mut map, point1);
        fun(team2, &mut map, point2);
    }

    let mut list: Vec<(String, Point)> = map.into_iter().collect();
    list.sort_by(|(k1, v1), (k2, v2)| v2.4.cmp(&v1.4).then(k1.cmp(k2)));
    let mut title = vec![format!("{:30} | MP |  W |  D |  L |  P", "Team")];
    title.extend(
        list.iter()
            .map(|(k, v)| {
                format!(
                    "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
                    k, v.0, v.1, v.2, v.3, v.4
                )
            })
            .collect::<Vec<String>>(),
    );
    title.join("\n")
}
