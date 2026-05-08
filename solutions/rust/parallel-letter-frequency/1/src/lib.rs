use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() || worker_count == 0 {
        return HashMap::new();
    }

    let chunk_size = (input.len() as f64 / worker_count as f64).ceil() as usize;

    let line_chunks = input.chunks(chunk_size);

    let frequency = thread::scope(|s| {
        let mut handlers = Vec::new();

        for lines in line_chunks {
            let handler = s.spawn(move || {
                let mut local_frequency = HashMap::new();

                for &line in lines {
                    for c in line
                        .chars()
                        .flat_map(|c| c.to_lowercase())
                        .filter(|c| c.is_alphabetic())
                    {
                        local_frequency
                            .entry(c)
                            .and_modify(|freq| *freq += 1)
                            .or_insert(1);
                    }
                }
                local_frequency
            });

            handlers.push(handler);
        }
        handlers
            .into_iter()
            .fold(HashMap::new(), |mut acc, handler| {
                let local_map = handler.join().unwrap();
                for (key, value) in local_map {
                    acc.entry(key)
                        .and_modify(|freq| *freq += value)
                        .or_insert(value);
                }
                acc
            })
    });

    frequency
}
