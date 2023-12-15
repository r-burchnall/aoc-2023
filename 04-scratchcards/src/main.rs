fn main() {
    let data = std::fs::read_to_string("scratchcard").expect("no scratchcard file");

    let total = data
        .lines()
        .map(|line| {
           return match line.find(':') {
               Some(i) => &line[i+1..].trim(),
               _ => ""
           };
        })
        .map(|line| {
            println!("{}",line);
            return line.split("|")
                .map(|set| set
                    .split(" ")
                    .filter(|&part|part!="")
                    .map(|numStr| {
                         numStr.trim().parse::<usize>().expect("RADICAL")
                    })
                    .collect::<Vec<_>>()
                )
                .collect::<Vec<_>>()
        })
        .map(|vecs| {
            let winning_nums = vecs.get(0).expect("NOWINNING :C");
            let my_nums = vecs.get(1).expect("NOMYNUMS :(");
            let count = my_nums.iter().filter(|&val| winning_nums.contains(val)).count();
            let total:usize = match count {
                0=>0,
                1=> 1,
                _=>1 << count - 1
            };
            println!("COUNT {count} SHIFTED {total}");
            return total;
        })
        .reduce(|a, b| a + b);

    println!("total of winning cards: {}", total.unwrap())
}
