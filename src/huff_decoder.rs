use crate::types::*;

pub fn decode_string(final_bit_offset: usize, tree: Tree, data: Vec<u8>) -> String {
    let mut byte_cursor: usize = 0;

    let len = data.len();

    let mut output = String::with_capacity(data.len() << 1);

    let mut worker_leaf = &tree;

    loop {
        let mut byte = data[byte_cursor];

        for _ in 0..(if len - 1 != byte_cursor {
            8
        } else {
            final_bit_offset
        }) {
            worker_leaf = if byte >= 128 {
                worker_leaf.left().unwrap().as_ref()
            } else {
                worker_leaf.right().unwrap().as_ref()
            };
            byte <<= 1;

            if worker_leaf.is_leaf() {
                let val = *worker_leaf.leaf_value().unwrap();

                output.push(val);

                worker_leaf = &tree;
            }
        }
        byte_cursor += 1;

        if len == byte_cursor {
            break output;
        }
    }
}

#[cfg(test)]
mod tests {
    const TEST_DATA: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec nec elementum arcu, et consequat odio. Nam at velit feugiat, hendrerit leo a, sagittis magna. Donec sit amet sapien sed urna condimentum dapibus in id nibh. Etiam suscipit aliquam egestas. Nunc sit amet condimentum dui. Sed sodales massa nec elit tristique aliquam. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Duis at massa accumsan, condimentum erat in, porta ex. Curabitur a nisi ac augue tincidunt egestas. Sed ut venenatis lacus. Vestibulum et lectus eu mauris semper ornare sit amet at felis. Maecenas sed augue eu elit pharetra iaculis maximus vel sapien. Suspendisse quis neque mollis, aliquet tellus in, ultricies enim. Ut ut eros venenatis, mollis nisi a, convallis massa. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Suspendisse rutrum magna vitae augue viverra sagittis.";

    use crate::huff_decoder::*;
    use crate::huff_encoder::*;
    #[test]
    fn it_decodes_string() {
        let input = String::from(TEST_DATA);

        let frequencies = frequency_extractor(&input);

        let mut queue = build_queue(frequencies);

        let tree = build_tree(&mut queue);

        let inverse_char_map = build_inverse_char_map(&tree);

        let (out, bit_offset) = encode_string(inverse_char_map, input);

        let output = decode_string(bit_offset, tree, out);

        assert_eq!(output, String::from(TEST_DATA));
    }
}
