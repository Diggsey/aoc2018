fn main() {
    let input = 110201;
    //let input = 9;

    let mut scores = vec![3, 7];
    let (mut a, mut b) = (0, 1);

    while scores.len() < input + 10 {
        let new_score = scores[a] + scores[b];
        if new_score >= 10 {
            scores.push(new_score/10);
        }
        scores.push(new_score % 10);
        a = (a + scores[a] + 1) % scores.len();
        b = (b + scores[b] + 1) % scores.len();
    }
    println!();
    println!("{:?}", scores);
    println!("{:?}", &scores[input..]);
}
