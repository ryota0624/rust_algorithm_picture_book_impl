use crate::compression::Compressor;
use std::ops::Add;

struct RunLengthCompressor;

impl RunLengthCompressor {
    fn run(&self, remain: String, chunked: String, chunked_len: usize) -> String {
        if remain.is_empty() {
            chunked.add(&chunked_len.to_string())
        } else {
            let (head, next_remain) = remain.split_at(1);
            if head == &chunked {
                self.run(next_remain.to_string(), chunked, chunked_len + 1)
            } else {
                chunked
                    .add(&chunked_len.to_string())
                    .add(&self.run(next_remain.to_string(), head.to_string(), 1))
            }
        }
    }
}

impl Compressor<String> for RunLengthCompressor {
    type Out = String;

    fn execute(self, input: String) -> String {
        let (head, next_remain) = input.split_at(1);
        self.run(next_remain.to_string(), head.to_string(), 1)
    }
}


#[test]
fn test() {
    assert_eq!(RunLengthCompressor.execute("yyyccdddd".to_string()),
    "y3c2d4"
    )
}
