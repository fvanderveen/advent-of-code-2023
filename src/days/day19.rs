use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::days::Day;
use crate::util::number::parse_usize;
use crate::util::parser::Parser;

pub const DAY19: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let system = WorkflowSystem::parse(input).unwrap();

    println!("Rating of accepted gears: {}", system.get_accepted_rating())
}

fn puzzle2(input: &String) {
    let system = WorkflowSystem::parse(input).unwrap();

    println!("Distinct combinations of accepted gears: {}", system.get_accepted_combinations())
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn get_value(&self, gear: &Gear) -> usize {
        match self {
            Self::X => gear.x,
            Self::M => gear.m,
            Self::A => gear.a,
            Self::S => gear.s,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Condition {
    None,
    LT(Category, usize),
    GT(Category, usize),
}

impl Condition {
    fn matches(&self, gear: &Gear) -> bool {
        match self {
            Self::None => true,
            Self::LT(cat, value) => cat.get_value(gear) < *value,
            Self::GT(cat, value) => cat.get_value(gear) > *value,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Action {
    Jump(String),
    Accept,
    Reject,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Rule {
    condition: Condition,
    action: Action,
}

impl Rule {
    fn apply(&self, gear: &Gear) -> Option<Action> {
        if self.condition.matches(gear) {
            Some(self.action.clone())
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn get_result(&self, gear: &Gear) -> Action {
        // A workflow should always have a catch-all rule, so we should be able to unwrap
        self.rules.iter().find_map(|r| r.apply(gear)).unwrap()
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Gear {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct WorkflowSystem {
    workflows: Vec<Workflow>,
    gears: Vec<Gear>,
}

impl WorkflowSystem {
    fn parse(input: &str) -> Result<WorkflowSystem, String> {
        if let [workflows_input, gears_input] = input.split("\n\n").collect::<Vec<_>>()[..] {
            let workflows = workflows_input.lines().map(|l| l.parse::<Workflow>()).collect::<Result<Vec<_>, _>>()?;
            let gears = gears_input.lines().map(|l| l.parse::<Gear>()).collect::<Result<Vec<_>, _>>()?;
            Ok(WorkflowSystem { workflows, gears })
        } else {
            Err("Could not split input on a blank line correctly.".to_string())
        }
    }

    fn get_workflow(&self, name: &str) -> Workflow {
        self.workflows.iter().find(|w| w.name.eq(name)).unwrap().clone()
    }

    fn accepts(&self, gear: &Gear) -> bool {
        // We assume the 'in' workflow exists
        let mut current = self.get_workflow("in");

        loop {
            match current.get_result(gear) {
                Action::Jump(target) => {
                    current = self.get_workflow(&target);
                }
                Action::Accept => return true,
                Action::Reject => return false
            }
        }
    }

    fn get_accepted_rating(&self) -> usize {
        self.gears.iter().filter(|g| self.accepts(g)).map(|g| g.x + g.m + g.a + g.s).sum()
    }

    fn get_accepted_combinations(&self) -> usize {
        // Ehh...
        // This mean we need to find paths from in => 'A' states, and determine which ranges of x, m, a, and s lead there.
        // Starting at 'in', we can follow all rules, keeping track of the xmas ranges (which are initially 1..=4000)
        // Since every condition is either '>' or '<', we should be able to trim the ranges until we arrive at an 'A'
        // Then we "just" need to merge the accepted ranges and multiply the results.

        #[derive(Debug, Clone)]
        struct Ranges {
            x: RangeInclusive<usize>,
            m: RangeInclusive<usize>,
            a: RangeInclusive<usize>,
            s: RangeInclusive<usize>
        }

        let initial = Ranges { x: 1..=4000, m: 1..=4000, a: 1..=4000, s: 1..=4000 };
        let mut accepted_ranges: Vec<Ranges> = vec![];

        fn make_unmatching(rule: &Rule, ranges: &Ranges) -> Ranges {
            match &rule.condition {
                Condition::None => Ranges { x: 1..=0, m: 1..=0, a: 1..=0, s: 1..=0 },
                Condition::LT(cat, value) => match cat {
                    Category::X => Ranges { x: *value..=*ranges.x.end(), ..ranges.clone() },
                    Category::M => Ranges { m: *value..=*ranges.m.end(), ..ranges.clone() },
                    Category::A => Ranges { a: *value..=*ranges.a.end(), ..ranges.clone() },
                    Category::S => Ranges { s: *value..=*ranges.s.end(), ..ranges.clone() },
                },
                Condition::GT(cat, value) => match cat {
                    Category::X => Ranges { x: *ranges.x.start()..=*value, ..ranges.clone() },
                    Category::M => Ranges { m: *ranges.m.start()..=*value, ..ranges.clone() },
                    Category::A => Ranges { a: *ranges.a.start()..=*value, ..ranges.clone() },
                    Category::S => Ranges { s: *ranges.s.start()..=*value, ..ranges.clone() },
                }
            }
        }

        fn follow_rule(system: &WorkflowSystem, rule: &Rule, ranges: &Ranges, accepted: &mut Vec<Ranges>) {
            let ranges = match &rule.condition {
                Condition::None => ranges.clone(),
                Condition::GT(cat, value) => match cat {
                    Category::X => Ranges { x: *value+1..=*ranges.x.end(), ..ranges.clone() },
                    Category::M => Ranges { m: *value+1..=*ranges.m.end(), ..ranges.clone() },
                    Category::A => Ranges { a: *value+1..=*ranges.a.end(), ..ranges.clone() },
                    Category::S => Ranges { s: *value+1..=*ranges.s.end(), ..ranges.clone() },
                },
                Condition::LT(cat, value) => match cat {
                    Category::X => Ranges { x: *ranges.x.start()..=*value-1, ..ranges.clone() },
                    Category::M => Ranges { m: *ranges.m.start()..=*value-1, ..ranges.clone() },
                    Category::A => Ranges { a: *ranges.a.start()..=*value-1, ..ranges.clone() },
                    Category::S => Ranges { s: *ranges.s.start()..=*value-1, ..ranges.clone() },
                }
            };

            match &rule.action {
                Action::Jump(workflow) => {
                    // Follow this workflow with the new ranges
                    follow_workflow(system, workflow, &ranges, accepted);
                }
                Action::Accept => { accepted.push(ranges); }
                Action::Reject => {} // do nothing
            }
        }

        fn follow_workflow(system: &WorkflowSystem, workflow: &str, ranges: &Ranges, accepted: &mut Vec<Ranges>) {
            let workflow = system.get_workflow(workflow);
            // Note: we cannot just follow every rule; not following the first rule will also mutate the ranges to ensure it _doesn't_ match.
            let mut ranges = ranges.clone();
            for rule in workflow.rules {
                follow_rule(system, &rule, &ranges, accepted);
                ranges = make_unmatching(&rule, &ranges);
            }
        }

        follow_workflow(self, "in", &initial, &mut accepted_ranges);

        // println!("Accepted ranges:\n{}", accepted_ranges.iter().cloned().map(|r| format!("{:?} => {}", r.clone(), r.x.count() * r.m.count() * r.a.count() * r.s.count())).collect::<Vec<_>>().join("\n"));

        // And now... how to make a number of accepted combinations from this result...?!
        // For the test data, this results in separate ranges... I'm a bit worries about the real data, though.

        accepted_ranges.iter().cloned().map(|r| r.x.count() * r.m.count() * r.a.count() * r.s.count()).sum()
    }
}


#[cfg(test)]
mod tests {
    use crate::days::day19::{Action, Category, Condition, Gear, Rule, Workflow, WorkflowSystem};

    #[test]
    fn test_parse_rule() {
        assert_eq!("a<2006:qkq".parse::<Rule>(), Ok(Rule { condition: Condition::LT(Category::A, 2006), action: Action::Jump("qkq".to_string()) }));
        assert_eq!("m>2090:A".parse::<Rule>(), Ok(Rule { condition: Condition::GT(Category::M, 2090), action: Action::Accept }));
        assert_eq!("rfg".parse::<Rule>(), Ok(Rule { condition: Condition::None, action: Action::Jump("rfg".to_string()) }));
        assert_eq!("A".parse::<Rule>(), Ok(Rule { condition: Condition::None, action: Action::Accept }));
        assert_eq!("R".parse::<Rule>(), Ok(Rule { condition: Condition::None, action: Action::Reject }));
    }

    #[test]
    fn test_parse_workflow() {
        assert_eq!("pv{a>1716:R,A}".parse::<Workflow>(), Ok(Workflow {
            name: "pv".to_string(),
            rules: vec![
                Rule { condition: Condition::GT(Category::A, 1716), action: Action::Reject },
                Rule { condition: Condition::None, action: Action::Accept },
            ],
        }));
    }

    #[test]
    fn test_parse_gear() {
        assert_eq!("{x=1679,m=44,a=2067,s=496}".parse::<Gear>(), Ok(Gear { x: 1679, m: 44, a: 2067, s: 496 }));
    }

    #[test]
    fn test_workflow_system_accepts() {
        let system = WorkflowSystem::parse(TEST_INPUT).unwrap();
        assert_eq!(system.accepts(&system.gears[0]), true);
        assert_eq!(system.accepts(&system.gears[1]), false);
        assert_eq!(system.accepts(&system.gears[2]), true);
        assert_eq!(system.accepts(&system.gears[3]), false);
        assert_eq!(system.accepts(&system.gears[4]), true);
    }

    #[test]
    fn test_workflow_system_accepted_rating() {
        let system = WorkflowSystem::parse(TEST_INPUT).unwrap();
        assert_eq!(system.get_accepted_rating(), 19114);
    }

    #[test]
    fn test_workflow_system_get_accepted_combinations() {
        let system = WorkflowSystem::parse(TEST_INPUT).unwrap();
        assert_eq!(system.get_accepted_combinations(), 167409079868000);
    }

    const TEST_INPUT: &str = "\
        px{a<2006:qkq,m>2090:A,rfg}\n\
        pv{a>1716:R,A}\n\
        lnx{m>1548:A,A}\n\
        rfg{s<537:gd,x>2440:R,A}\n\
        qs{s>3448:A,lnx}\n\
        qkq{x<1416:A,crn}\n\
        crn{x>2662:A,R}\n\
        in{s<1351:px,qqz}\n\
        qqz{s>2770:qs,m<1801:hdj,R}\n\
        gd{a>3333:R,R}\n\
        hdj{m>838:A,pv}\n\
        \n\
        {x=787,m=2655,a=1222,s=2876}\n\
        {x=1679,m=44,a=2067,s=496}\n\
        {x=2036,m=264,a=79,s=2244}\n\
        {x=2461,m=1339,a=466,s=291}\n\
        {x=2127,m=1623,a=2188,s=1013}\
    ";
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // note: split should result in three parts, with the last being empty. We just ignore any other parts, just want at least two.
        if let [name, rules_str] = s.split(&['{', '}']).collect::<Vec<_>>()[0..=1] {
            let rules = rules_str.split(',').map(|r| r.parse::<Rule>()).collect::<Result<Vec<_>, _>>()?;
            Ok(Workflow { name: name.to_string(), rules })
        } else {
            Err(format!("Could not parse workflow: '{}'", s))
        }
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(':').collect();

        match parts[..] {
            [action] => Ok(Rule { condition: Condition::None, action: action.parse()? }),
            [condition, action] => Ok(Rule { condition: condition.parse()?, action: action.parse()? }),
            _ => Err(format!("Invalid rule: {}", s))
        }
    }
}

impl FromStr for Condition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let category: Category = s[0..=0].parse()?;
        let op = &s[1..=1];
        let value = parse_usize(&s[2..])?;
        match op {
            "<" => Ok(Self::LT(category, value)),
            ">" => Ok(Self::GT(category, value)),
            _ => Err(format!("Invalid operator: {}", op))
        }
    }
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(format!("Invalid category {}", s))
        }
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Jump(s.to_string())
        })
    }
}

impl FromStr for Gear {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        parser.literal("{")?;
        parser.literal("x=")?;
        let x = parser.usize()?;
        parser.literal(",")?;
        parser.literal("m=")?;
        let m = parser.usize()?;
        parser.literal(",")?;
        parser.literal("a=")?;
        let a = parser.usize()?;
        parser.literal(",")?;
        parser.literal("s=")?;
        let s = parser.usize()?;
        parser.literal("}")?;
        parser.ensure_exhausted()?;

        Ok(Self { x, m, a, s })
    }
}