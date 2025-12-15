use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
    rc::Rc,
};

enum SearchStatus {
    Unexplored,
    Exploring,
    Explored(PathCounts),
}

#[derive(Clone)]
struct PathCounts {
    simple_paths: usize,
    fft_paths: usize,
    dac_paths: usize,
    both_paths: usize,
}

impl AddAssign<&PathCounts> for PathCounts {
    fn add_assign(&mut self, rhs: &PathCounts) {
        self.simple_paths += rhs.simple_paths;
        self.fft_paths += rhs.fft_paths;
        self.dac_paths += rhs.dac_paths;
        self.both_paths += rhs.both_paths;
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
            fft_paths: 0,
            dac_paths: 0,
            both_paths: 0,
        }),
    });

    (devices, indecies)
}

fn count_simple_paths(devices: &[Device], from: usize, to: usize) -> usize {
    if from == to {
        1
    } else {
        devices[from]
            .connections
            .iter()
            .map(|connection| count_simple_paths(devices, *connection, to))
            .sum()
    }
}

pub fn task1(input: &str) -> String {
    let (devices, indecies) = parse_devices(input);
    count_simple_paths(&devices, indecies["you"], indecies["out"]).to_string()
}

fn count_transformed_paths(devices: &mut [Device], from: usize, dac: usize, fft: usize) -> PathCounts {
    match &devices[from].search_status {
        SearchStatus::Unexplored => {
            devices[from].search_status = SearchStatus::Exploring;
            let mut path_counts = PathCounts {
                simple_paths: 0,
                fft_paths: 0,
                dac_paths: 0,
                both_paths: 0,
            };
            for connection in devices[from].connections.clone().iter() {
                path_counts += &count_transformed_paths(devices, *connection, dac, fft);
            }
            if from == dac {
                path_counts.both_paths = path_counts.fft_paths;
                path_counts.dac_paths = path_counts.simple_paths;
            }
            if from == fft {
                path_counts.both_paths = path_counts.dac_paths;
                path_counts.fft_paths = path_counts.simple_paths;
            }
            devices[from].search_status = SearchStatus::Explored(path_counts.clone());
            path_counts
        }
        SearchStatus::Exploring => panic!("loop"),
        SearchStatus::Explored(path_counts) => path_counts.clone(),
    }
}

pub fn task2(input: &str) -> String {
    let (mut devices, indecies) = parse_devices(input);
    devices[indecies["out"]].search_status = SearchStatus::Explored(PathCounts {
        simple_paths: 1,
        fft_paths: 0,
        dac_paths: 0,
        both_paths: 0,
    });
    count_transformed_paths(&mut devices, indecies["svr"], indecies["dac"], indecies["fft"])
        .both_paths
        .to_string()
}
