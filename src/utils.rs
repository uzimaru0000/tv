pub(crate) fn multi_lines(str_vec: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let split_values = str_vec.iter().map(|x| x.iter().map(|x| x.split("\n")));
    let mapper: Vec<Vec<(usize, usize, usize)>> = split_values
        .clone()
        .map(|xs| {
            (
                xs.len(),
                xs.map(|x| x.clone().collect::<Vec<_>>().len())
                    .max()
                    .unwrap_or_default(),
            )
        })
        .enumerate()
        .flat_map(|(idx, (h, v))| {
            (0..v).map(move |y| (0..h).map(move |x| (idx, x, y)).collect::<Vec<_>>())
        })
        .collect();
    let fields = mapper
        .iter()
        .map(|xs| {
            xs.iter()
                .map(|&(idx, ix, iy)| {
                    split_values
                        .clone()
                        .nth(idx)
                        .and_then(|x| x.clone().nth(ix))
                        .and_then(|x| x.clone().nth(iy))
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fields
        .into_iter()
        .map(|xs| xs.into_iter().map(String::from).collect::<Vec<_>>())
        .collect()
}
