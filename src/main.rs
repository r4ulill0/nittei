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
