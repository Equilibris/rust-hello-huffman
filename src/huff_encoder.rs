use crate::header::Header;
use crate::push_bits;
use crate::types::*;
use crate::utils::math::num_bits;

// Done
pub fn frequency_extractor(input: &String) -> CharMap {
    let mut map = CharMap::with_capacity(128);

    for chr in input.chars() {
        match map.get(&chr) {
            Some(i) => {
                let insert_value = i + 1;
                map.insert(chr, insert_value);
            }
            None => {
                map.insert(chr, 1);
            }
        }
    }

    map
}

pub fn build_queue(char_map: CharMap) -> Queue {
    let mut queue = Queue::with_capacity(char_map.len());

    for (key, value) in char_map {
        queue.push(Tree::Leaf(key), (value as i32) * -1);
    }

    queue
}

// Done
pub fn build_tree(queue: &mut Queue) -> Tree {
    loop {
        match (queue.pop(), queue.pop()) {
            (Some((output, _)), None) => break output,
            (Some((left, left_priority)), Some((right, right_priority))) => {
                queue.push(
                    Tree::Node(Box::new(left), Box::new(right)),
                    left_priority + right_priority,
                );
            }
            _ => {
                panic!("Queue is empty / broken");
            }
        }
    }
}

fn internal_build_inverse_char_map(tree: &Tree, base_sum: u32) -> CharMap {
    let mut map = CharMap::new();

    match tree {
        &Tree::Leaf(ref key) => {
            let insert_value = ((base_sum << 1) + 1) << base_sum.leading_zeros() - 1;
            let insert_value = insert_value << 1;

            map.insert(*key, insert_value);
        }
        &Tree::Node(ref left, ref right) => {
            let next_base_sum = base_sum << 1;

            map.extend(
                internal_build_inverse_char_map(left.as_ref(), next_base_sum + 1).into_iter(),
            );
            map.extend(internal_build_inverse_char_map(right.as_ref(), next_base_sum).into_iter());
        }
    }

    map
}
pub fn build_inverse_char_map(tree: &Tree) -> CharMap {
    internal_build_inverse_char_map(tree, 1)
}

// Tree symbols
// 00 = leaf
// 01 = node followed by leaf
// 10 = node followed by node
// 11 = node followed by 2 leafs (localized root)

/*
10   01        'd'  01        'q'  10   11        'C'       'f'   01        'D'  01        ';'  11        'M'       'L'        00   'n'
Node(Node(Leaf('d'),Node(Leaf('q'),Node(Node(Leaf('C'),Leaf('f')),Node(Leaf('D'),Node(Leaf(';'),Node(Leaf('M'),Leaf('L'))))))),Leaf('n'))
*/

fn encode_tree_iteration(
    node: &Tree,
    bit_cursor: &mut u16,
    byte_cursor: &mut usize,
    vec: &mut Vec<u8>,
) {
    if vec.len() + 4 > *byte_cursor {
        vec.extend([0; 8]);
    }
    match node {
        Tree::Leaf(val) => {
            let val = ((*val) as u16) << 8 >> *bit_cursor;

            for (index, val) in val.to_be_bytes().iter().enumerate() {
                vec[*byte_cursor + index] |= *val;
            }
            *byte_cursor += 1;
        }
        Tree::Node(left, right) => {
            if node.is_localized_root() {
                let val = (0b1100000000000000 as u16) >> *bit_cursor;

                push_bits!(vec, *bit_cursor, *byte_cursor, val, 2);

                encode_tree_iteration(left, bit_cursor, byte_cursor, vec);
                encode_tree_iteration(right, bit_cursor, byte_cursor, vec);
            } else if left.is_leaf() {
                let val = (0b0100000000000000 as u16) >> *bit_cursor;

                push_bits!(vec, *bit_cursor, *byte_cursor, val, 2);

                encode_tree_iteration(left, bit_cursor, byte_cursor, vec);
                encode_tree_iteration(right, bit_cursor, byte_cursor, vec);
            } else {
                let val = (0b1000000000000000 as u16) >> *bit_cursor;

                push_bits!(vec, *bit_cursor, *byte_cursor, val, 2);

                encode_tree_iteration(left, bit_cursor, byte_cursor, vec);

                if right.is_leaf() {
                    let val = (0b0000000000000000 as u16) >> *bit_cursor;

                    push_bits!(vec, *bit_cursor, *byte_cursor, val, 2);
                }

                encode_tree_iteration(right, bit_cursor, byte_cursor, vec);
            }
        }
    }
}

pub fn encode_tree(tree: &Tree) -> Vec<u8> {
    let mut vec = vec![0 as u8; 32];

    let mut bit_cursor: u16 = 0;
    let mut byte_cursor: usize = 0;

    encode_tree_iteration(tree, &mut bit_cursor, &mut byte_cursor, &mut vec);

    vec.resize(byte_cursor + (bit_cursor > 0) as usize, 0);

    vec
}

pub fn encode_string(map: CharMap, input_string: String) -> (Vec<u8>, usize) {
    let mut vec = vec![0 as u8; input_string.len()];

    let mut bit_cursor: usize = 0;
    let mut byte_cursor: usize = 0;

    for symbol in input_string.chars() {
        match map.get(&symbol) {
            Some(symbol) => {
                let symbol = *symbol;

                let symbol_size = num_bits::<u32>() - (symbol.trailing_zeros() + 1) as usize;
                let symbol = symbol >> bit_cursor;
                let trailing: u32 = symbol.trailing_zeros();

                let symbol = symbol >> (trailing + 1) << trailing + 1;

                push_bits!(vec, bit_cursor, byte_cursor, symbol, symbol_size);
            }
            None => panic!("Symbol map lacking symbol {}", symbol),
        }
    }

    // Bit offset has to be encoded in header
    vec.resize(byte_cursor + (bit_cursor > 0) as usize, 0);
    (vec, bit_cursor)
}

pub fn encoder(input: String) -> Vec<u8> {
    let frequencies = frequency_extractor(&input);

    let mut queue = build_queue(frequencies);

    let tree = build_tree(&mut queue);

    let encoded_tree = encode_tree(&tree);

    let inverse_char_map = build_inverse_char_map(&tree);

    let (encoded_string, bit_offset) = encode_string(inverse_char_map, input);

    let header = Header::new(
        bit_offset as u8,
        encoded_tree.len() as u32,
        encoded_string.len() as u32,
    );

    let mut output = Vec::with_capacity(encoded_tree.len() + encoded_string.len() + 8);

    output.extend(header.serialize());
    output.extend(encoded_tree);
    output.extend(encoded_string);

    output
}

#[cfg(test)]
mod tests {
    use crate::huff_encoder::*;

    const TEST_STR: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";

    #[test]
    fn experiments() {
        let mut vec = vec![0 as u8; 10];
        let mut bit_cursor: usize = 0;
        let mut byte_cursor: usize = 0;

        let value: u16 = 0xFFFF;

        push_bits!(vec, bit_cursor, byte_cursor, value, 16);

        println!("{:?} {} {}", vec, bit_cursor, byte_cursor);
    }

    #[test]
    fn it_extracts_frequencies() {
        let test_string: String = String::from(TEST_STR);

        let frequencies = frequency_extractor(&test_string);

        // generated with
        // s = "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
        // print(*map(lambda x: f"assert_eq!(frequencies.get(&'{x}'), Some(&({s.count(x)} as u32)));", set([*s])))

        assert_eq!(frequencies.get(&'c'), Some(&(3 as u32)));
        assert_eq!(frequencies.get(&'p'), Some(&(2 as u32)));
        assert_eq!(frequencies.get(&' '), Some(&(7 as u32)));
        assert_eq!(frequencies.get(&'d'), Some(&(2 as u32)));
        assert_eq!(frequencies.get(&'n'), Some(&(2 as u32)));
        assert_eq!(frequencies.get(&'L'), Some(&(1 as u32)));
        assert_eq!(frequencies.get(&'u'), Some(&(2 as u32)));
        assert_eq!(frequencies.get(&'o'), Some(&(4 as u32)));
        assert_eq!(frequencies.get(&'r'), Some(&(3 as u32)));
        assert_eq!(frequencies.get(&'.'), Some(&(1 as u32)));
        assert_eq!(frequencies.get(&'i'), Some(&(6 as u32)));
        assert_eq!(frequencies.get(&'g'), Some(&(1 as u32)));
        assert_eq!(frequencies.get(&'s'), Some(&(4 as u32)));
        assert_eq!(frequencies.get(&'l'), Some(&(2 as u32)));
        assert_eq!(frequencies.get(&'e'), Some(&(5 as u32)));
        assert_eq!(frequencies.get(&'m'), Some(&(3 as u32)));
        assert_eq!(frequencies.get(&'t'), Some(&(5 as u32)));
        assert_eq!(frequencies.get(&'a'), Some(&(2 as u32)));
        assert_eq!(frequencies.get(&','), Some(&(1 as u32)));

        assert_eq!(frequencies.get(&'æ'), None);
    }

    // #[test]
    // fn it_builds_the_optimal_queue() {
    //     todo!();

    //     use itertools::Itertools;

    //     let test_string: String = String::from(test_str);

    //     let frequencies = frequency_extractor(&test_string);

    //     let mut queue = build_queue(frequencies);

    //     // for (key, priority) in frequencies
    //     //     .into_iter()
    //     //     .sorted_by_key(|(_, priority)| -(*priority as i32))
    //     // {
    //     //     assert_eq!(queue.pop(), Some((Tree::Leaf(key), -(priority as i32))))
    //     // }
    // }

    #[test]
    fn it_has_unambiguous_inverse_char_map() {
        let test_string: String = String::from(TEST_STR);

        let frequencies = frequency_extractor(&test_string);

        let mut queue = build_queue(frequencies);

        let tree = build_tree(&mut queue);

        let inverse_char_map = build_inverse_char_map(&tree);

        for (char, key) in inverse_char_map.into_iter() {
            println!("{}: {:#032b}", char, key);
        }
    }

    #[test]
    fn it_encodes_tree() {
        let input = String::from(TEST_STR);

        let frequencies = frequency_extractor(&input);

        let mut queue = build_queue(frequencies);

        let tree = build_tree(&mut queue);

        for i in encode_tree(&tree) {
            print!("{:08b}", i);
        }
    }
}
