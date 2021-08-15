use crate::bit_byte_increase;
use crate::header::Header;
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

// Assume there are only power of 2 sized symbols in the data since there are.
// This means that the bit_cursor is an even number
fn decode_tree_iteration(data: &Vec<u8>, bit_cursor: &mut u16, byte_cursor: &mut usize) -> Tree {
    let operator = data[*byte_cursor] << *bit_cursor;

    if operator >= 0b11000000 {
        let val = ((data[*byte_cursor] as u32) << 24)
            | ((data[*byte_cursor + 1] as u32) << 16)
            | ((data[*byte_cursor + 2] as u32) << 8);

        let val = (val << *bit_cursor << 2 >> 16) as u16;

        let bytes: [u8; 2] = val.to_be_bytes();

        bit_byte_increase!(*bit_cursor, *byte_cursor, 18);

        Tree::Node(
            Box::new(Tree::Leaf(bytes[0] as Symbol)),
            Box::new(Tree::Leaf(bytes[1] as Symbol)),
        )
    } else if operator >= 0b10000000 {
        bit_byte_increase!(*bit_cursor, *byte_cursor, 2);

        let left = decode_tree_iteration(data, bit_cursor, byte_cursor);

        let right = decode_tree_iteration(data, bit_cursor, byte_cursor);

        Tree::Node(Box::new(left), Box::new(right))
    } else if operator >= 0b01000000 {
        let val = ((data[*byte_cursor] as u16) << 8) | (data[*byte_cursor + 1] as u16);

        let val = (val << *bit_cursor << 2 >> 8) as u8;

        bit_byte_increase!(*bit_cursor, *byte_cursor, 10);

        Tree::Node(
            Box::new(Tree::Leaf(val as Symbol)),
            Box::new(decode_tree_iteration(data, bit_cursor, byte_cursor)),
        )
    } else {
        let val = ((data[*byte_cursor] as u16) << 8) | (data[*byte_cursor + 1] as u16);

        let val = (val << *bit_cursor << 2 >> 8) as u8;

        bit_byte_increase!(*bit_cursor, *byte_cursor, 10);

        Tree::Leaf(val as Symbol)
    }
}

pub fn decode_tree(data: Vec<u8>) -> Tree {
    let mut bit_cursor: u16 = 0;
    let mut byte_cursor: usize = 0;

    decode_tree_iteration(&data, &mut bit_cursor, &mut byte_cursor)
}

pub fn decoder(data: Vec<u8>) -> String {
    let header = Header::from_raw(u64::from_be_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]));

    let header_end = (header.symbol_size as usize) + 8;
    let content_end = (header.content_size as usize) + header_end;

    let tree = decode_tree(data[8..header_end].to_vec());

    decode_string(
        header.offset as usize,
        tree,
        data[header_end..content_end].to_vec(),
    )
}

#[cfg(test)]
mod tests {
    const TEST_STR: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec nec elementum arcu, et consequat odio. Nam at velit feugiat, hendrerit leo a, sagittis magna. Donec sit amet sapien sed urna condimentum dapibus in id nibh. Etiam suscipit aliquam egestas. Nunc sit amet condimentum dui. Sed sodales massa nec elit tristique aliquam. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Duis at massa accumsan, condimentum erat in, porta ex. Curabitur a nisi ac augue tincidunt egestas. Sed ut venenatis lacus. Vestibulum et lectus eu mauris semper ornare sit amet at felis. Maecenas sed augue eu elit pharetra iaculis maximus vel sapien. Suspendisse quis neque mollis, aliquet tellus in, ultricies enim. Ut ut eros venenatis, mollis nisi a, convallis massa. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Suspendisse rutrum magna vitae augue viverra sagittis.";

    use crate::huff_decoder::*;
    use crate::huff_encoder::*;

    #[test]
    fn it_decodes_string() {
        let input = String::from(TEST_STR);

        let frequencies = frequency_extractor(&input);

        let mut queue = build_queue(frequencies);

        let tree = build_tree(&mut queue);

        let inverse_char_map = build_inverse_char_map(&tree);

        let (out, bit_offset) = encode_string(inverse_char_map, input);

        let output = decode_string(bit_offset, tree, out);

        assert_eq!(output, String::from(TEST_STR));
    }

    fn do_tree_test(tree: Tree) {
        let encoded_tree = encode_tree(&tree);

        let decoded_tree = decode_tree(encoded_tree);

        assert_eq!(tree, decoded_tree);
    }

    #[test]
    fn it_decodes_tree() {
        {
            let tree = Tree::Node(Box::new(Tree::Leaf('x')), Box::new(Tree::Leaf('a')));

            do_tree_test(tree);
        }
        {
            let tree = Tree::Node(
                Box::new(Tree::Node(
                    Box::new(Tree::Leaf('x')),
                    Box::new(Tree::Leaf('a')),
                )),
                Box::new(Tree::Node(
                    Box::new(Tree::Leaf('z')),
                    Box::new(Tree::Leaf('b')),
                )),
            );

            do_tree_test(tree);
        }
        {
            let tree = Tree::Node(
                Box::new(Tree::Node(
                    Box::new(Tree::Leaf('x')),
                    Box::new(Tree::Leaf('a')),
                )),
                Box::new(Tree::Leaf('z')),
            );

            do_tree_test(tree);
        }
        {
            let tree = Tree::Node(
                Box::new(Tree::Leaf('z')),
                Box::new(Tree::Node(
                    Box::new(Tree::Leaf('x')),
                    Box::new(Tree::Leaf('a')),
                )),
            );

            do_tree_test(tree);
        }
        {
            let input = String::from(TEST_STR);

            let frequencies = frequency_extractor(&input);

            let mut queue = build_queue(frequencies);

            let tree = build_tree(&mut queue);

            do_tree_test(tree);
        }
    }

    #[test]
    fn it_decodes() {
        let input = String::from(TEST_STR);

        let encoded = encoder(input);

        let decoded = decoder(encoded);

        assert_eq!(decoded, String::from(TEST_STR));
    }
}
