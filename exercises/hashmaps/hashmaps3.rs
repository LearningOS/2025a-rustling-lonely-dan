// hashmaps3.rs
use std::collections::HashMap;

// 定义 Team 结构体（根据上下文补充）
#[derive(Debug, PartialEq, Eq)]
struct Team {
    goals_scored: u32,
    goals_conceded: u32,
}

// 假设核心功能是记录一场比赛的比分
fn update_scores(
    scores: &mut HashMap<String, Team>,
    team_1_name: &str,
    team_1_goals: u32,
    team_2_name: &str,
    team_2_goals: u32,
) {
    // 修正：and_modify 必须在 entry() 之后、or_insert() 之前调用
    scores
        .entry(team_1_name.to_string())
        .and_modify(|t| {
            t.goals_scored += team_1_goals;
            t.goals_conceded += team_2_goals;
        })
        .or_insert(Team {
            goals_scored: team_1_goals,
            goals_conceded: team_2_goals,
        });

    scores
        .entry(team_2_name.to_string())
        .and_modify(|t| {
            t.goals_scored += team_2_goals;
            t.goals_conceded += team_1_goals;
        })
        .or_insert(Team {
            goals_scored: team_2_goals,
            goals_conceded: team_1_goals,
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_scores_correctly() {
        let mut scores = HashMap::new();
        update_scores(&mut scores, "Brazil", 2, "Germany", 1);
        
        assert_eq!(
            scores.get("Brazil"),
            Some(&Team {
                goals_scored: 2,
                goals_conceded: 1
            })
        );
        assert_eq!(
            scores.get("Germany"),
            Some(&Team {
                goals_scored: 1,
                goals_conceded: 2
            })
        );

        // 测试更新已有球队的分数
        update_scores(&mut scores, "Brazil", 1, "France", 3);
        assert_eq!(
            scores.get("Brazil"),
            Some(&Team {
                goals_scored: 3,  // 2+1
                goals_conceded: 4  // 1+3
            })
        );
        assert_eq!(
            scores.get("France"),
            Some(&Team {
                goals_scored: 3,
                goals_conceded: 1
            })
        );
    }
}