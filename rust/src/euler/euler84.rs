pub fn solve() -> Result<String, String> {
    let matrix = calculate_transition_matrix();

    let mut p: Vec<f64> = vec![0.0; FIELD_COUNT];
    p[0] = 1.0;

    let mut dest: Vec<f64> = vec![0.0; FIELD_COUNT];

    for _ in 0..1000 {
        iterate_markov(&p, &mut dest, &matrix);
        for i in 0..p.len() {
            p[i] = dest[i];
        }
    }

    let mut a = p.iter().enumerate().map(|k| (k.0, *k.1)).collect::<Vec<(usize, f64)>>();
    a.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let res = format!("{:02}{:02}{:02}", a[0].0, a[1].0, a[2].0);
    Ok(res)
}

const FIELDS: &'static [&'static str] = &[
    "GO", "A1", "CC1", "A2", "T1", "R1", "B1", "CH1", "B2", "B3", "JAIL", "C1", "U1", "C2", "C3",
    "R2", "D1", "CC2", "D2", "D3", "FP", "E1", "CH2", "E2", "E3", "R3", "F1", "F2", "U2", "F3",
    "G2J", "G1", "G2", "CC3", "G3", "R4", "CH3", "H1", "T2", "H2",
];

const FIELD_COUNT: usize = FIELDS.len();
const DICE: usize = 4;
const CARD_CHANCE: f64 = 1.0 / 16.0;

fn iterate_markov(p_source: &Vec<f64>, p_dest: &mut Vec<f64>, matrix: &Vec<Vec<f64>>) {
    p_dest.iter_mut().for_each(|e| *e = 0.0);
    for i in 0..p_source.len() {
        for d in 0..p_dest.len() {
            let p = matrix[i][d];
            p_dest[d] += p_source[i] * p;
        }
    }
}

fn get_field(name: &str) -> usize {
    FIELDS.iter().position(|&r| r == name).unwrap()
}

fn get_next_field(current_index: usize, name_prefix: &str) -> usize {
    if let Some(index) = FIELDS[current_index..].iter().position(|&r| r.starts_with(name_prefix)) {
        return index + current_index;
    }
    FIELDS[..current_index].iter().position(|&r| r.starts_with(name_prefix)).unwrap()
}

fn add_transition(p: f64, field: usize, transitions: &mut Vec<f64>) {
    transitions[field] += p;
}

fn make_step(
    p: f64,
    field: usize,
    transitions: &mut Vec<f64>,
    _is_double: bool,
    _doubles_count: usize,
) {
    add_transition(p, field, transitions);
}

fn calculate_step(
    p: f64,
    field: usize,
    transitions: &mut Vec<f64>,
    is_double: bool,
    doubles_count: usize,
) {
    let field_name = FIELDS[field];
    let pcard = p * CARD_CHANCE;
    if field_name.starts_with("CC") {
        make_step(
            pcard,
            get_field("GO"),
            transitions,
            is_double,
            doubles_count,
        );
        make_step(
            pcard,
            get_field("JAIL"),
            transitions,
            is_double,
            doubles_count,
        );
        make_step(pcard * 14.0, field, transitions, is_double, doubles_count);
    } else if field_name.starts_with("CH") {
        make_step(
            pcard,
            get_field("GO"),
            transitions,
            is_double,
            doubles_count,
        );
        make_step(
            pcard,
            get_field("JAIL"),
            transitions,
            is_double,
            doubles_count,
        );
        make_step(
            pcard,
            get_field("C1"),
            transitions,
            is_double,
            doubles_count,
        );
        make_step(
            pcard,
            get_field("E3"),
            transitions,
            is_double,
            doubles_count,
        );
        make_step(
            pcard,
            get_field("H2"),
            transitions,
            is_double,
            doubles_count,
        );
        make_step(
            pcard,
            get_field("R1"),
            transitions,
            is_double,
            doubles_count,
        );
        make_step(
            pcard * 2.0,
            get_next_field(field, "R"),
            transitions,
            is_double,
            doubles_count,
        );
        make_step(
            pcard,
            get_next_field(field, "U"),
            transitions,
            is_double,
            doubles_count,
        );
        // Go back 3 squares
        let back_3_steps = (field - 3 + FIELD_COUNT) % FIELD_COUNT;
        calculate_step(pcard, back_3_steps, transitions, is_double, doubles_count);
        make_step(pcard * 6.0, field, transitions, is_double, doubles_count);
    } else if field_name == "G2J" || field_name == "JAIL" {
        add_transition(p, get_field("JAIL"), transitions);
    } else {
        make_step(p, field, transitions, is_double, doubles_count);
    }
}

fn roll_dice(p: f64, field: usize, doubles_count: usize, transitions: &mut Vec<f64>) {
    let pdice = p * (1.0 / DICE as f64) * (1.0 / DICE as f64);
    for d1 in 1..=DICE {
        for d2 in 1..=DICE {
            let index = (field + d1 + d2) % FIELD_COUNT;
            if d1 == d2 {
                let p_third_double = (1.0 / DICE as f64) * (1.0 / DICE as f64);
                add_transition(pdice * p_third_double, get_field("JAIL"), transitions);
                calculate_step(
                    pdice * (1.0 - p_third_double),
                    index,
                    transitions,
                    false,
                    doubles_count,
                );
            } else {
                calculate_step(pdice, index, transitions, false, doubles_count);
            }
        }
    }
}

fn calculate_transition_matrix() -> Vec<Vec<f64>> {
    let mut markov_matrix: Vec<Vec<f64>> = Vec::new();
    for field in 0..FIELD_COUNT {
        let mut transitions: Vec<f64> = vec![0.0; FIELD_COUNT];
        roll_dice(1.0, field, 0, &mut transitions);
        markov_matrix.push(transitions);
    }
    markov_matrix
}
