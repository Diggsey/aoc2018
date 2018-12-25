fn main() {
    let input = [1,1,0,2,0,1];
    //let input = [5,1,5,8,9];
    //let input = [5,9,4,1,4];
    //let input = [9,2,5,1,0];

    let mut scores = vec![3, 7];
    let (mut a, mut b) = (0, 1);

    loop {
        let new_score = scores[a] + scores[b];
        if new_score >= 10 {
            scores.push(new_score/10);
            if scores.len() >= input.len() {
                if scores[scores.len() - input.len()..] == input {
                    break;
                }
            }
        }
        scores.push(new_score % 10);
        if scores.len() >= input.len() {
            if scores[scores.len() - input.len()..] == input {
                break;
            }
        }

        a = (a + scores[a] + 1) % scores.len();
        b = (b + scores[b] + 1) % scores.len();
    }

    let offset = scores.len() - input.len();
    println!();
    println!("{}", offset);
    println!("{:?}", &scores[offset..]);
}
