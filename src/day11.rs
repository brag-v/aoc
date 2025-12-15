use std::{collections::HashMap, ops::AddAssign, rc::Rc};

enum SearchStatus {
    Unexplored,
    Exploring,
    Explored(PathCounts),
}

#[derive(Clone)]
struct PathCounts {
    simple_paths: usize,
    paths_with_fft: usize,
    paths_with_dac: usize,
    paths_with_both: usize,
}

impl AddAssign<&PathCounts> for PathCounts {
    fn add_assign(&mut self, rhs: &PathCounts) {
        self.simple_paths += rhs.simple_paths;
        self.paths_with_fft += rhs.paths_with_fft;
        self.paths_with_dac += rhs.paths_with_dac;
        self.paths_with_both += rhs.paths_with_both;
    }
}

struct Device {
    connections: Rc<[usize]>,
    search_status: SearchStatus,
}

fn parse_devices(input: &str) -> (Vec<Device>, HashMap<String, usize>) {
    let mut indecies: HashMap<String, usize> = input
        .lines()
        .enumerate()
        .map(|(i, line)| (line[..3].to_owned(), i))
        .collect();

    indecies.insert("out".to_owned(), indecies.len());

    let mut devices: Vec<Device> = input
        .lines()
        .map(|line| Device {
            connections: line
                .split(' ')
                .skip(1)
                .map(|device| indecies[device])
                .collect(),
            search_status: SearchStatus::Unexplored,
        })
        .collect();

    devices.push(Device {
        connections: Rc::new([]),
        search_status: SearchStatus::Explored(PathCounts {
            simple_paths: 1,
            paths_with_fft: 0,
            paths_with_dac: 0,
            paths_with_both: 0,
        }),
    });

    (devices, indecies)
}

fn count_simple_paths(devices: &mut [Device], from: usize) -> usize {
    match &devices[from].search_status {
        SearchStatus::Unexplored => {
            devices[from].search_status = SearchStatus::Exploring;
            let path_count: usize = devices[from]
                .connections
                .clone()
                .iter()
                .map(|connection| count_simple_paths(devices, *connection))
                .sum();
            devices[from].search_status = SearchStatus::Explored(PathCounts {
                simple_paths: path_count,
                paths_with_fft: 0,
                paths_with_dac: 0,
                paths_with_both: 0,
            });
            path_count
        }
        SearchStatus::Exploring => panic!("loop detecte in path"),
        SearchStatus::Explored(path_counts) => path_counts.simple_paths,
    }
}

pub fn task1(input: &str) -> String {
    let (mut devices, indecies) = parse_devices(input);
    count_simple_paths(&mut devices, indecies["you"]).to_string()
}

fn count_transformed_paths(
    devices: &mut [Device],
    from: usize,
    dac: usize,
    fft: usize,
) -> PathCounts {
    match &devices[from].search_status {
        SearchStatus::Unexplored => {
            devices[from].search_status = SearchStatus::Exploring;
            let mut path_counts = PathCounts {
                simple_paths: 0,
                paths_with_fft: 0,
                paths_with_dac: 0,
                paths_with_both: 0,
            };
            for connection in devices[from].connections.clone().iter() {
                path_counts += &count_transformed_paths(devices, *connection, dac, fft);
            }
            if from == dac {
                path_counts.paths_with_both = path_counts.paths_with_fft;
                path_counts.paths_with_dac = path_counts.simple_paths;
            }
            if from == fft {
                path_counts.paths_with_both = path_counts.paths_with_dac;
                path_counts.paths_with_fft = path_counts.simple_paths;
            }
            devices[from].search_status = SearchStatus::Explored(path_counts.clone());
            path_counts
        }
        SearchStatus::Exploring => panic!("loop detecte in path"),
        SearchStatus::Explored(path_counts) => path_counts.clone(),
    }
}

pub fn task2(input: &str) -> String {
    let (mut devices, indecies) = parse_devices(input);
    count_transformed_paths(
        &mut devices,
        indecies["svr"],
        indecies["dac"],
        indecies["fft"],
    )
    .paths_with_both
    .to_string()
}
