use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar="calendar.pest"]
pub struct CalendarParser;
fn main() {
    let example_entry = "*-7-2 #extend 1w | Bob's birthday";
    let parse_tree = CalendarParser::parse(Rule::calendar, example_entry);
    println!("{:#?}", parse_tree);
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{CalendarParser, Rule};
    use lazy_static::lazy_static;
    use pest_test::{PestTester, TestError};

    lazy_static! {
        static ref COLORIZE: bool = {
                    option_env!("CARGO_TERM_COLOR").unwrap_or("always") != "never"
                };
        static ref TESTER: PestTester<Rule, CalendarParser> = 
        // TODO bug in pest_test 0.1.6
        PestTester::new("tests/pest","txt", Rule::calendar, HashSet::new());
        //PestTester::from_defaults(Rule::calendar, HashSet::new());
    }

    #[test]
    fn test_every_day_task() -> Result<(), TestError<Rule>> {
        let res = (*TESTER).evaluate_strict("every_day_task");
        if let Err(pest_test::TestError::Diff { ref diff }) = res {
                        diff.print_test_result(*COLORIZE).unwrap();
                    }
        res
    }

    #[test]
    fn event_early_notification() -> Result<(), TestError<Rule>> {
        let res = (*TESTER).evaluate_strict("event_early_notification");
        if let Err(pest_test::TestError::Diff { ref diff }) = res {
                        diff.print_test_result(*COLORIZE).unwrap();
                    }
        res
    }

    #[test]
    fn small_calendar() -> Result<(), TestError<Rule>> {
        let res = (*TESTER).evaluate_strict("small_calendar");
        if let Err(pest_test::TestError::Diff { ref diff }) = res {
                        diff.print_test_result(*COLORIZE).unwrap();
                    }
        res
    }
}
