const BLOCK_CHARS: &[&str] = &[" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

pub fn simple_bar_graph<D: IntoIterator<Item = f64>>(
    width: usize,
    height: usize,
    data: D,
) -> String {
    let buckets = densities(width, data);

    let max = *buckets.iter().max().unwrap_or(&0);

    let scale = (height as f64) / (max as f64);
    let scaled_data: Vec<f64> =
        buckets.iter().map(|&x| (x as f64) * scale).collect();

    let mut result = String::new();
    for l in (0..height).rev() {
        for &x in &scaled_data {
            let r = (x - (l as f64)).clamp(0., 1.);
            let i = (r * ((BLOCK_CHARS.len() - 1) as f64)) as usize;
            result += BLOCK_CHARS[i];
        }
        result += "\r\n";
    }
    return result;
}

fn densities<D: IntoIterator<Item = f64>>(width: usize, data: D) -> Vec<usize> {
    let data: Vec<_> = data.into_iter().collect();
    let min = data
        .iter()
        .fold(f64::INFINITY, |acc, &x| if x < acc { x } else { acc });
    let max =
        data.iter()
            .fold(f64::NEG_INFINITY, |acc, &x| if x > acc { x } else { acc });

    let mut buckets = vec![0; width];

    for d in data {
        let mut i = (((d - min) * (width as f64)) / (max - min)) as usize;
        if i == width {
            i = width - 1;
        }
        buckets[i] += 1;
    }

    buckets
}
