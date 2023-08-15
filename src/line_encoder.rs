#[derive(Debug, Clone)]
pub struct LineEncoder {
    lines: Vec<usize>,
    counts: Vec<usize>,
}

/// Encoder to wrap the lines that repeat more than one time
impl LineEncoder {
    /// Decode all the lines and returns a Vec with all the lines
    pub fn decode_lines(&self) -> Vec<usize> {
        let mut decoded_lines = Vec::new();

        for (line, &count) in self.lines.iter().zip(&self.counts) {
            for _ in 0..count {
                decoded_lines.push(*line);
            }
        }

        decoded_lines
    }

    /// Encode all the lines into a vector with just the line number and
    /// push the number of times it repeats in another vector called counts
    pub fn encode_lines(&mut self, lines: Vec<usize>) {
        if lines.is_empty() {
            return;
        }

        let mut current_line = lines[0];
        let mut count = 1;

        for &line in lines.iter().skip(1) {
            if line == current_line {
                count += 1;
            } else {
                self.lines.push(current_line);
                self.counts.push(count);
                current_line = line;
                count = 1;
            }
        }

        self.lines.push(current_line);
        self.counts.push(count);
    }

    /// Search for the line in the given position, if it doesn't exists returns None
    pub fn get_line(&self, index: usize) -> Option<usize> {
        let mut current_index = 0;

        for (i, &count) in self.counts.iter().enumerate() {
            current_index += count;
            if index < current_index {
                return Some(self.lines[i]);
            }
        }

        None
    }

    pub fn new() -> Self {
        LineEncoder {
            lines: Vec::new(),
            counts: Vec::new(),
        }
    }
}
