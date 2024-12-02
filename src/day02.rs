pub fn run(input: &str) {
    let reports: Vec<Vec<i32>> = input.lines().map(|line| {
        line.split_whitespace().filter_map(|num: &str| num.parse::<i32>().ok()).collect()
    }).collect();

    part_one(&reports);
    part_two(&reports);
}

fn part_one(reports: &Vec<Vec<i32>>) {
    let mut safe_reports = reports.len();

    for report in reports {
        if !check_report(report).0 {
            safe_reports -= 1;
        }
    }

    println!("Day 2, part 1 result: {}", safe_reports);
}

fn part_two(reports: &Vec<Vec<i32>>) {
    let mut safe_reports = reports.len();

    for report in reports.iter() {
        let report_check_result = check_report(report);
        if !report_check_result.0 {
            let sub_reports = generate_sub_reports(report);

            let mut sub_reports_failures = 0;
            for sub_report in sub_reports.iter() {
                if !check_report(sub_report).0 {
                    sub_reports_failures += 1;
                }
            }

            if !(sub_reports.len() > sub_reports_failures) {
                safe_reports -= 1;
            }
        }
    }

    println!("Day 2, part 2 result: {}", safe_reports);
}

fn generate_sub_reports(report: &[i32]) -> Vec<Vec<i32>> {
    let mut sub_reports = Vec::new();
    for i in 0..report.len() {
        let mut new_report = report.to_vec();
        new_report.remove(i);
        sub_reports.push(new_report);
    }
    sub_reports
}

fn check_report(report: &Vec<i32>) -> (bool, usize) {
    let mut change_direction = 0;
    for (i, pair) in report.windows(2).enumerate() {
        let level_change = pair[0] - pair[1];
        if level_change.abs() > 3 || level_change == 0 {
            return (false, i);
        }
        if change_direction > 0 && level_change < 0 || change_direction < 0 && level_change > 0 {
            return (false, i);
        }
        change_direction = level_change;
    }
    return (true, 0);
}
