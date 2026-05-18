#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars()
            .enumerate()
            .find(|(_, c)| !matches!(c, 'A' | 'C' | 'G' | 'T'))
            .map_or(
                Ok(Self {
                    sequence: dna.to_string(),
                }),
                |(i, _)| Err(i),
            )
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            sequence: self
                .sequence
                .chars()
                .map(|c| match c {
                    'A' => 'U',
                    'C' => 'G',
                    'G' => 'C',
                    'T' => 'A',
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars()
            .enumerate()
            .find(|(_, c)| !matches!(c, 'A' | 'C' | 'G' | 'U'))
            .map_or(
                Ok(Self {
                    sequence: rna.to_string(),
                }),
                |(i, _)| Err(i),
            )
    }
}
