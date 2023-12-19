advent_of_code::solution!(19);

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::alpha1,
    combinator::map_res,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Operator {
    GreaterThan,
    LessThan,
}

#[derive(Debug)]
enum Variable {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Condition {
    variable: Variable,
    operator: Operator,
    value: u32,
}

#[derive(Debug)]
enum Rule {
    Do { action: Action },
    If { cond: Condition, action: Action },
}

#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    GoTo { workflow: String },
}

#[derive(Debug, Clone)]
struct Point {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(take_while(|c: char| c.is_numeric()), FromStr::from_str)(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (_, x, _, m, _, a, _, s, _)) = tuple((
        tag("{"),
        preceded(tag("x="), parse_number),
        tag(","),
        preceded(tag("m="), parse_number),
        tag(","),
        preceded(tag("a="), parse_number),
        tag(","),
        preceded(tag("s="), parse_number),
        tag("}"),
    ))(input)?;

    Ok((input, Point { x, m, a, s }))
}

fn parse_action(input: &str) -> IResult<&str, Action> {
    let (input, action) = alpha1(input)?;
    match action {
        "A" => Ok((input, Action::Accept)),
        "R" => Ok((input, Action::Reject)),
        _ => Ok((
            input,
            Action::GoTo {
                workflow: action.to_string(),
            },
        )),
    }
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    if let Ok((input, cond)) = parse_condition(input) {
        let (input, _) = tag(":")(input)?;
        let (input, action) = parse_action(input)?;
        Ok((input, Rule::If { cond, action }))
    } else {
        let (input, action) = parse_action(input)?;
        Ok((input, Rule::Do { action }))
    }
}

fn parse_workflow(input: &str) -> IResult<&str, (String, Vec<Rule>)> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, rules) = nom::multi::separated_list1(tag(","), parse_rule)(input)?;
    let (input, _) = tag("}")(input)?;

    Ok((input, (name.to_string(), rules)))
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (input, (variable, operator, value)) = tuple((
        take_while(|c: char| c.is_alphabetic()),
        take_while(|c: char| c == '<' || c == '>'),
        parse_number,
    ))(input)?;

    let operator = match operator {
        "<" => Operator::LessThan,
        ">" => Operator::GreaterThan,
        _ => panic!("Invalid operator"),
    };

    let variable = match variable {
        "x" => Variable::X,
        "m" => Variable::M,
        "a" => Variable::A,
        "s" => Variable::S,
        _ => panic!("Invalid variable"),
    };

    Ok((
        input,
        Condition {
            variable,
            operator,
            value,
        },
    ))
}

fn parse_input(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Point>) {
    let mut workflows = HashMap::new();
    let mut points = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        } else if line.contains('=') {
            let (_, point) = parse_point(line).unwrap();
            points.push(point);
        } else {
            let (_, (name, rules)) = parse_workflow(line).unwrap();
            workflows.insert(name, rules);
        }
    }

    (workflows, points)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (workflows, points) = parse_input(input);
    Some(
        points
            .iter()
            .filter(|p| is_accepted(&workflows, p))
            .map(|p| p.x + p.m + p.a + p.s)
            .sum(),
    )
}

fn is_accepted(workflows: &HashMap<String, Vec<Rule>>, point: &Point) -> bool {
    let mut curr = "in";

    loop {
        let rules = workflows.get(curr).unwrap();
        for rule in rules {
            match rule {
                Rule::Do { action } => match action {
                    Action::Accept => return true,
                    Action::Reject => return false,
                    Action::GoTo { workflow } => curr = workflow,
                },
                Rule::If { cond, action } => {
                    let value = match cond.variable {
                        Variable::X => point.x,
                        Variable::M => point.m,
                        Variable::A => point.a,
                        Variable::S => point.s,
                    };

                    let result = match cond.operator {
                        Operator::GreaterThan => value > cond.value,
                        Operator::LessThan => value < cond.value,
                    };

                    if result {
                        match action {
                            Action::Accept => return true,
                            Action::Reject => return false,
                            Action::GoTo { workflow } => {
                                curr = workflow;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (workflows, _) = parse_input(input);
    let candidates = find_candidates(&workflows);
    Some(candidates.iter().map(|c| c.find_combinations()).sum())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Candidate {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

impl Candidate {
    fn intersect_var(&self, var: &Variable, range: (u32, u32)) -> Candidate {
        let mut res = self.clone();
        match var {
            Variable::X => {
                res.x = intersect_range(res.x, range);
            }
            Variable::M => {
                res.m = intersect_range(res.m, range);
            }
            Variable::A => {
                res.a = intersect_range(res.a, range);
            }
            Variable::S => {
                res.s = intersect_range(res.s, range);
            }
        }
        res
    }

    fn find_combinations(&self) -> usize {
        let x_range = self.x.1 - self.x.0 + 1;
        let m_range = self.m.1 - self.m.0 + 1;
        let a_range = self.a.1 - self.a.0 + 1;
        let s_range = self.s.1 - self.s.0 + 1;
        x_range as usize * m_range as usize * a_range as usize * s_range as usize
    }
}

fn find_candidates(workflows: &HashMap<String, Vec<Rule>>) -> Vec<Candidate> {
    let initial = Candidate {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };

    let mut to_visit = vec![("in", initial)];
    let mut results = Vec::new();

    while let Some((node, candidate)) = to_visit.pop() {
        let rules = workflows.get(node).unwrap();
        let mut curr = candidate.clone();

        for rule in rules {
            match rule {
                Rule::Do { action } => match action {
                    Action::Accept => {
                        results.push(curr.clone());
                        continue;
                    }
                    Action::Reject => continue,
                    Action::GoTo { workflow } => {
                        to_visit.push((workflow, curr.clone()));
                    }
                },

                Rule::If { cond, action } => {
                    let (var, true_range, false_range) = condition_to_ranges(cond);
                    let true_branch = curr.intersect_var(var, true_range);
                    let false_branch = curr.intersect_var(var, false_range);
                    match action {
                        Action::Accept => {
                            results.push(true_branch);
                        }
                        Action::Reject => {}
                        Action::GoTo { workflow } => {
                            to_visit.push((workflow, true_branch));
                        }
                    }
                    // for every subsequent rule we assume that this condition didn't hold
                    curr = false_branch;
                }
            }
        }
    }

    results
}

fn condition_to_ranges(cond: &Condition) -> (&Variable, (u32, u32), (u32, u32)) {
    match cond {
        Condition {
            variable,
            operator: Operator::GreaterThan,
            value,
        } => (variable, (*value + 1, 4000), (1, *value)),
        Condition {
            variable,
            operator: Operator::LessThan,
            value,
        } => (variable, (1, *value - 1), (*value, 4000)),
    }
}

fn intersect_range((start1, end1): (u32, u32), (start2, end2): (u32, u32)) -> (u32, u32) {
    let start = start1.max(start2);
    let end = end1.min(end2);

    if start < end {
        (start, end)
    } else {
        panic!("Can't create disjoint range: ({},{})", start, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
