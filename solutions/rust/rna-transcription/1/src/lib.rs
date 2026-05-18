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
        for (i, c) in dna.chars().enumerate() {
            if !['A', 'C', 'G', 'T'].contains(&c) {
                return Err(i);
            }
        }

        Ok(Self {
            sequence: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna_seq: String = self
            .sequence
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => ' ',
            })
            .collect();
        Rna::new(rna_seq.as_str()).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.chars().enumerate() {
            if !['A', 'C', 'G', 'U'].contains(&c) {
                return Err(i);
            }
        }

        Ok(Self {
            sequence: rna.to_string(),
        })
    }
}
