use std::collections::BinaryHeap;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone)]
struct State {
    time: usize,

    // Bots
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,

    // Resources
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}
impl State {
    fn add_bot(&mut self, bot: BotType, blueprint: &Blueprint) {
        match bot {
            BotType::Ore => {
                self.ore_bots += 1;
                self.ore -= blueprint.ore_robot_ore_cost;
            }
            BotType::Clay => {
                self.clay_bots += 1;
                self.ore -= blueprint.clay_robot_ore_cost;
            }
            BotType::Obsidian => {
                self.obsidian_bots += 1;
                self.ore -= blueprint.obsidian_robot_ore_cost;
                self.clay -= blueprint.obsidian_robot_clay_cost;
            }
            BotType::Geode => {
                self.geode_bots += 1;
                self.ore -= blueprint.geode_robot_ore_cost;
                self.obsidian -= blueprint.geode_robot_obsidian_cost;
            }
        }
    }

    fn can_build(&self, bot: BotType, blueprint: &Blueprint) -> bool {
        match bot {
            BotType::Ore => self.ore >= blueprint.ore_robot_ore_cost,
            BotType::Clay => self.ore >= blueprint.clay_robot_ore_cost,
            BotType::Obsidian => {
                self.ore >= blueprint.obsidian_robot_ore_cost
                    && self.clay >= blueprint.obsidian_robot_clay_cost
            }
            BotType::Geode => {
                self.ore >= blueprint.geode_robot_ore_cost
                    && self.obsidian >= blueprint.geode_robot_obsidian_cost
            }
        }
    }

    fn initial_state() -> Self {
        State {
            time: 0,

            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,

            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }

    /// Run simulation until the specified bot type is built, or until the simulation runs out of time
    fn simulate_until_can_build_bot(&self, blueprint: &Blueprint, bot: &BotType) -> State {
        // Short circuit for unbuildable configs
        if (bot.eq(&BotType::Geode) && self.obsidian_bots < 1)
            || (bot.eq(&BotType::Obsidian) && self.clay_bots < 1)
        {
            let mut new_state = self.clone();
            new_state.time = 24;
            return new_state;
        }

        // Given _enough_ time, we should be able to gather enough resources. Now the question becomes "Do we have enough time"
        let mut new_state = self.clone();
        while new_state.time < 24 {
            new_state.time += 1;
            if new_state.can_build(*bot, blueprint) {
                new_state.gather();
                new_state.add_bot(*bot, blueprint);
                break;
            } else {
                new_state.gather();
            }
        }

        new_state
    }

    fn gather(&mut self) {
        self.ore += self.ore_bots;
        self.clay += self.clay_bots;
        self.obsidian += self.obsidian_bots;
        self.geodes += self.geode_bots;
    }
}

struct Strategy {
    blueprint: Blueprint,
    state: State,
    path: Vec<BotType>,
}
impl Strategy {
    /// Computes an upper bound on the value (in total cracked geodes) of continuing to follow
    /// this path. Note that this is a very loose estimate, used for pruning search space. All
    /// options which even have the _potential_ to exceed the current maximum found will be fully
    /// explored. It is fine to overestimate, which trades additional computation time for simplicity
    fn compute_upper_bound_value(&self) -> usize {
        let mut rounds_remaining = 24 - self.state.time;
        let mut total_geodes = self.state.geodes;
        let mut geode_bots = self.state.geode_bots;
        let mut should_build = self.state.obsidian > 0;
        while rounds_remaining > 0 {
            // Gather
            total_geodes += geode_bots;

            // Build
            if should_build {
                geode_bots += 1;
            } else {
                should_build = !should_build;
            }

            rounds_remaining -= 1;
        }

        total_geodes
    }

    /// Given a current strategy state, generate potential bots to build next
    fn expand(&self) -> Vec<BotType> {
        let mut candidates = vec![BotType::Ore, BotType::Clay];
        if self.state.clay_bots > 0 {
            candidates.push(BotType::Obsidian);
        }
        if self.state.obsidian_bots > 0 {
            candidates.push(BotType::Geode);
        }
        candidates
    }
}
impl PartialEq for Strategy {
    fn eq(&self, other: &Self) -> bool {
        self.compute_upper_bound_value()
            .eq(&other.compute_upper_bound_value())
    }
}
impl Eq for Strategy {}
impl PartialOrd for Strategy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.compute_upper_bound_value()
            .partial_cmp(&other.compute_upper_bound_value())
    }
}
impl Ord for Strategy {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.compute_upper_bound_value()
            .cmp(&other.compute_upper_bound_value())
    }
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: usize,
    ore_robot_ore_cost: usize,
    clay_robot_ore_cost: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
}
impl TryFrom<String> for Blueprint {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts = value.split(' ').collect::<Vec<_>>();
        let id = parts[1]
            .strip_suffix(':')
            .expect("Should have trailing ':'")
            .parse::<usize>()
            .expect("Should be able to parse id");
        let ore_robot_ore_cost = parts[6]
            .parse::<usize>()
            .expect("Should be able to parse ore_robot_ore_cost");
        let clay_robot_ore_cost = parts[12]
            .parse::<usize>()
            .expect("Should be able to parse clay_robot_ore_cost");
        let obsidian_robot_ore_cost = parts[18]
            .parse::<usize>()
            .expect("Should be able to parse obsidian_robot_ore_cost");
        let obsidian_robot_clay_cost = parts[21]
            .parse::<usize>()
            .expect("Should be able to parse obsidian_robot_clay_cost");
        let geode_robot_ore_cost = parts[27]
            .parse::<usize>()
            .expect("Should be able to parse geode_robot_ore_cost");
        let geode_robot_obsidian_cost = parts[30]
            .parse::<usize>()
            .expect("Should be able to parse geode_robot_obsidian_cost");
        Ok(Blueprint {
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        })
    }
}
impl Blueprint {
    /// Return the maximum number of geodes that can be built with a given blueprint
    fn max_geodes(&self) -> usize {
        let mut minimum_number_of_geodes_produced = 0;
        let mut best_strategy = None;
        let mut queue = BinaryHeap::new();
        let strategy = Strategy {
            blueprint: *self,
            state: State::initial_state(),
            path: vec![],
        };
        queue.push(strategy);

        while let Some(next_to_expand) = queue.pop() {
            if next_to_expand.compute_upper_bound_value() <= minimum_number_of_geodes_produced {
                continue;
            }

            for bot_type in next_to_expand.expand() {
                let new_state = next_to_expand
                    .state
                    .simulate_until_can_build_bot(self, &bot_type);
                let mut updated_path = next_to_expand.path.clone();
                updated_path.push(bot_type);
                if new_state.geodes > minimum_number_of_geodes_produced {
                    minimum_number_of_geodes_produced = new_state.geodes;
                    best_strategy = Some(updated_path.clone())
                }
                let new_strategy = Strategy {
                    blueprint: next_to_expand.blueprint,
                    state: new_state,
                    path: updated_path,
                };
                queue.push(new_strategy);
            }
        }

        println!(
            "Best strategy: {:#?}, value {}",
            best_strategy
                .iter()
                .map(|bt| format!("{:#?}", bt))
                .collect::<Vec<_>>()
                .join(", "),
            minimum_number_of_geodes_produced
        );

        minimum_number_of_geodes_produced
    }

    /// Compute the quality metric for the blueprint
    fn quality_level(&self) -> usize {
        let max_geodes = self.max_geodes();
        let res = self.id * max_geodes;
        res
    }
}

struct Day19 {}
impl AoCProblem for Day19 {
    fn name(&self) -> String {
        "day-19".to_owned()
    }
}
impl Solution for Day19 {
    fn solution(&self, path: &str) {
        let blueprints = read_lines(path)
            .expect("Should be able to read file")
            .map(|line| line.expect("Should be able to read line"))
            .map(Blueprint::try_from)
            .map(|blueprint| blueprint.expect("Should be able to parse blueprint"));

        let quality_levels = blueprints.map(|bp| bp.quality_level()).sum::<usize>();
        println!("Part one: {:#?}", quality_levels);
    }
}

fn main() {
    Day19 {}.test_and_run();
}
