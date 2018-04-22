use rand::{Rng, ChaChaRng};

const FEMALE_FIRST_NAMES: &'static str = include_str!("dist.female.first");
const MALE_FIRST_NAMES: &'static str = include_str!("dist.male.first");
const LAST_NAMES: &'static str = include_str!("dist.all.last");

#[derive(Debug)]
pub struct Name {
    pub first_name: &'static str,
    pub last_name: &'static str,
}

impl Name {
    pub fn male(mut rng :&mut ChaChaRng) -> Self {
        Name {
            first_name: get_rand_name(&MALE_FIRST_NAMES, &mut rng),
            last_name:  get_rand_name(&LAST_NAMES, &mut rng),
        }
    }

    pub fn female(mut rng :&mut ChaChaRng) -> Self {
        Name {
            first_name: get_rand_name(&FEMALE_FIRST_NAMES, &mut rng),
            last_name:  get_rand_name(&LAST_NAMES, &mut rng),
        }
    }
}

fn get_rand_name(names: &'static str, rng :&mut ChaChaRng) -> &'static str {
    let name_length = get_name_length();
    let number_of_names: usize = names.len() / name_length;

    let name_index: usize = rng.gen_range(0, number_of_names);

    let line_offset = name_index * name_length;
    let line = &names[line_offset..line_offset+name_length];

    let first_space = line.find(" ").unwrap();

    return &line[0..first_space];
}

//use const fn once it's stable in rust
fn get_name_length() -> usize {
    return LAST_NAMES.find("\n").unwrap() + 1;
}
