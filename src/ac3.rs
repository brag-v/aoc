/// ac3 algorithm for filtering domain of variables
/// where every variable must have an unique assignment
pub fn ac3<T: Clone + Eq>(domains: &mut [Vec<T>]) {
    let mut assigned = vec![false; domains.len()];
    let mut assigned_values = Vec::new();
    if let Some((i, singelton_domain)) = domains
        .iter()
        .enumerate()
        .find(|(_, domain)| domain.len() == 1)
    {
        assigned_values.push(singelton_domain[0].clone());
        assigned[i] = true;
    }
    while let Some(constrained_assignment) = assigned_values.pop() {
        for (i, domain) in domains.iter_mut().enumerate() {
            if assigned[i] {
                continue;
            }
            *domain = domain
                .iter()
                .filter(|assignment| **assignment != constrained_assignment)
                .cloned()
                .collect::<Vec<T>>();
            if domain.len() == 1 {
                assigned_values.push(domain[0].clone());
                assigned[i] = true;
            }
        }
    }
}
