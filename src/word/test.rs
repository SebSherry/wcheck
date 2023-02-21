use super::*;

macro_rules! word_tests {
    ($($name:ident: $state:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (dictionary, word, expected_result) = $state;
            let test_word = Word { word, line_nr: 1};
            assert_eq!(test_word.is_correct_spelling(&dictionary), expected_result)
        }
    )*
    }
}

word_tests! {
    sanity: (vec!["team".to_string()], "team".to_string(), Ok(())),
    uppercase: (vec!["team".to_string()], "TEAM".to_string(), Ok(())),
    not_a_word: (vec!["team".to_string()], "adsf".to_string(), Err(vec!["adsf".to_string()])),
    camel_case: (vec!["team".to_string(),"work".to_string()], "teamWork".to_string(), Ok(())),
    camel_case_long: (vec!["team".to_string(),"work".to_string()], "teamWorkWorkTeam".to_string(), Ok(())),
    camel_case_middle_incorrect: (vec!["team".to_string(),"work".to_string()], "teamWorkWrkTeam".to_string(), Err(vec!["Wrk".to_string()])),
    camel_case_multiple_incorrect: (vec!["team".to_string(),"work".to_string()], "teamWorkWrkTem".to_string(), Err(vec!["Wrk".to_string(),"Tem".to_string()])),
    snake_case: (vec!["team".to_string(),"work".to_string()], "team_work".to_string(), Ok(())),
    snake_case_long: (vec!["team".to_string(),"work".to_string()], "team_work_work_team".to_string(), Ok(())),
    snake_case_middle_incorrect: (vec!["team".to_string(),"work".to_string()], "team_work_wrk_team".to_string(), Err(vec!["wrk".to_string()])),
    snake_case_multiple_incorrect: (vec!["team".to_string(),"work".to_string()], "team_work_wrk_tem".to_string(), Err(vec!["wrk".to_string(),"tem".to_string()])),
}
