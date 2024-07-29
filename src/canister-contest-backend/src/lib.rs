use std::cell::RefCell;

thread_local! {
    static LEADERBOARD: RefCell<Vec<Record>> = RefCell::default();
}

#[derive(Clone)]
struct Record {
    name: String,
    score: String,
    name2: String,
    score2: String,
    scoresdiff: String,
}

#[ic_cdk::update]
fn add(name: String, score: String, name2: String, score2: String, scoresdiff: String) {
    let record = Record {
        name,
        score,
        name2,
        score2,
        scoresdiff,
    };
    LEADERBOARD.with(|records| {
        records.borrow_mut().push(record);
    });
    sort();
}

#[ic_cdk::query]
fn show() -> Vec<Vec<String>> {
    LEADERBOARD.with(|records| {
        records.borrow().iter().map(|record| {
            vec![
                record.name.clone(),
                record.score.clone(),
                record.name2.clone(),
                record.score2.clone(),
                record.scoresdiff.clone(),
            ]
        }).collect()
    })
}

#[ic_cdk::update]
fn sort() {
    LEADERBOARD.with(|records| {
        let mut leaderboard = records.borrow_mut();
        leaderboard.sort_by(|x, y| {
            let score_x = x.scoresdiff.parse::<u64>().unwrap_or(0);
            let score_y = y.scoresdiff.parse::<u64>().unwrap_or(0);
            score_y.cmp(&score_x)
        });
    });
}
