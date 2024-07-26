use std::cell::RefCell;

thread_local! {
    static LEADERBOARD: RefCell<Vec<Vec<String>>> = RefCell::default();
}

#[ic_cdk::update]
fn add_record(name: String, score: String, name2: String, score2: String, scoresdiff: String) {
    LEADERBOARD.with(|records| {
        records.borrow_mut().push(vec![name, score, name2, score2, scoresdiff]);
    });
}

#[ic_cdk::query]
fn read_leaderboard() -> Vec<Vec<String>> {
    sort_leaderboard();
    LEADERBOARD.with(|records| {
        records.borrow().clone()
    })
}

#[ic_cdk::update]
fn clear_leaderboard() {
    LEADERBOARD.with(|records| {
        records.borrow_mut().clear();
    });
}

#[ic_cdk::update]
fn sort_leaderboard() {
    LEADERBOARD.with(|records| {
        let mut leaderboard = records.borrow_mut();
        
        leaderboard.sort_by(|a, b| {
            let score_a = a[4].parse::<u64>().unwrap_or(0);
            let score_b = b[4].parse::<u64>().unwrap_or(0);
            score_b.cmp(&score_a)
        });
    });
}
