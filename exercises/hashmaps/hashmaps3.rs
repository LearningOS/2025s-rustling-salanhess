// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

/*
让我帮你完成这个 hashmaps3.rs 的练习。这是一个关于足球比赛得分统计的练习，需要我们使用 HashMap 来存储每个队伍的进球和失球数据。

让我先分析一下测试用例和要求：
1. 需要统计每个队伍的进球数(goals_scored)和失球数(goals_conceded)
2. 每行输入格式为："队伍1,队伍2,队伍1得分,队伍2得分"
3. 需要为每个出现的队伍创建记录并更新其数据

我将编辑代码来完成这个练习：
我已经完成了代码的编写，让我解释一下实现的逻辑：

1. 对于每一行比赛记录，我们需要更新两支队伍的数据
2. 使用 `HashMap` 的 `entry` API 来处理队伍是否已存在的情况：
   - 如果队伍不存在，创建一个新的 `Team` 结构体，初始化得分和失球都为 0
   - 如果队伍已存在，则获取该队伍的记录
3. 对于每支队伍：
   - 增加他们在本场比赛中的进球数（goals_scored）
   - 增加他们在本场比赛中的失球数（goals_conceded）

代码中使用了以下 Rust 特性：
- `entry()` 方法：提供了一种方便的方式来处理键可能存在或不存在的情况
- `or_insert()`：如果键不存在，则插入提供的值
- `clone()`：因为 HashMap 需要拥有键的所有权，所以我们克隆了队伍名称字符串

现在这段代码应该能通过所有测试了，包括：
- 检查是否包含了所有队伍
- 验证英格兰队的得分记录（进5球，失4球）
- 验证西班牙队的得分记录（进0球，失2球）

*/

use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
        
        // Update team 1's stats
        let team1 = scores.entry(team_1_name.clone()).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team1.goals_scored += team_1_score;
        team1.goals_conceded += team_2_score;

        // Update team 2's stats
        let team2 = scores.entry(team_2_name.clone()).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team2.goals_scored += team_2_score;
        team2.goals_conceded += team_1_score;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
