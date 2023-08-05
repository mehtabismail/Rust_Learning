struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![Test { score: 10 }, Test { score: 20 }, Test { score: 30 }];

    for test in my_scores {
        println!("score = {:?}", test.score)
    }
}
