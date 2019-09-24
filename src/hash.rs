use sha2::Digest;
use sha2::Sha256;
use std::fs::File;
use std::io;
use std::io::Read;

const CHUNK_SIZE: usize = 1000000;

pub struct StreamHasher {
    hasher: Sha256,
}

impl StreamHasher {
    pub fn new() -> StreamHasher {
        StreamHasher {
            hasher: Sha256::new(),
        }
    }

    pub fn input(&mut self, input: &[u8]) {
        self.hasher.input(input);
    }

    pub fn finish(self) -> String {
        let digest = self.hasher.result();
        let mut output_str = String::new();

        for byte in digest {
            output_str += &format!("{:02x}", byte);
        }
        return output_str;
    }
}

pub fn hash_file(fname: &str) -> io::Result<String> {
    let mut file = File::open(fname)?;

    let mut hasher = StreamHasher::new();

    let mut buffer: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];
    let mut bytes_read = file.read(&mut buffer[..])?;
    while bytes_read > 0 {
        hasher.input(&buffer[0..bytes_read]);
        bytes_read = file.read(&mut buffer[..])?;
    }

    Ok(hasher.finish())
}
