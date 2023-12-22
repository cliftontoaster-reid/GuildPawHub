use rand::Rng;

/// A list of furry uwu adjectives for generated ids.
const ADJECTIVES: &[&str] = &[
  "fluffy",
  "cute",
  "cutie",
  "boopy",
  "cuddly",
  "pawsome",
  "floofy",
  "pawesome",
  "whiskery",
  "wiggly",
  "poofy",
  "bouncy",
  "fluff",
  "cloudy",
  "marshmallow",
  "cotton candy",
  "butterscotch",
  "chocolate",
  "vanilla",
  "strawberry",
  "mango",
  "pineapple",
  "orange",
  "grape",
  "cherry",
  "apple",
  "kiwi",
  "grapefruit",
  "banana",
];

/// A list of furry uwu names for generated ids.
const NAMES: &[&str] = &[
  "fluffball",
  "nugget",
  "bean",
  "floof",
  "pouncer",
  "wiggles",
  "whiskers",
  "tailwag",
  "paws",
  "ears",
  "snowbell",
  "ember",
  "mist",
  "stormbringer",
  "thunderpaw",
  "riversong",
  "moonglow",
  "duskweaver",
  "suntail",
];

pub fn gen_id() -> String {
  let mut rng = rand::thread_rng();
  // 1. Pick an adjective
  let adjective = ADJECTIVES[rng.gen_range(0..ADJECTIVES.len())].to_string();

  // 2. Pick a name
  let name = NAMES[rng.gen_range(0..NAMES.len())].to_string();

  // 3. Combine the adjective and name
  format!("{}-{}", adjective, name)
}
