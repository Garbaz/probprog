const BLOCK_CHARS: &[&str] = &[" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

pub fn simple_bar_graph(height: usize, data: &Vec<usize>) -> String {
    let max = if let Some(&max) = data.iter().max() {
        max
    } else {
        0
    };

    let scale = (height as f64) / (max as f64);
    let scaled_data : Vec<f64> = data.iter().map(|&x| (x as f64) * scale).collect();

    let mut result = String::new();
    for l in (0..height).rev() {
        for &x in &scaled_data {
            let r = (x - (l as f64)).clamp(0., 1.);
            let i = (r * ((BLOCK_CHARS.len()-1) as f64)) as usize;
            result += BLOCK_CHARS[i];
        }
        result += "\r\n";
    }
    return result;
}
