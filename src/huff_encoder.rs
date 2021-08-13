use crate::binary_tree::BinaryTree;
use crate::utils::math::num_bits;
use priority_queue::PriorityQueue;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

type Symbol = char;

type CharMap = HashMap<Symbol, u32>;
type Queue = PriorityQueue<BinaryTree<Symbol>, i32, RandomState>;

// Done
fn frequency_extractor(input: &String) -> CharMap {
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

fn build_queue(char_map: CharMap) -> Queue {
    let mut queue = PriorityQueue::with_capacity(char_map.len());

    for (key, value) in char_map {
        queue.push(BinaryTree::Leaf(key), (value as i32) * -1);
    }

    queue
}

// Done
fn build_tree(queue: &mut Queue) -> BinaryTree<Symbol> {
    loop {
        match (queue.pop(), queue.pop()) {
            (Some((output, _)), None) => break output,
            (Some((left, left_priority)), Some((right, right_priority))) => {
                queue.push(
                    BinaryTree::Node(Box::new(left), Box::new(right)),
                    left_priority + right_priority,
                );
            }
            _ => {
                panic!("Queue is empty / broken");
            }
        }
    }
}

fn internal_build_inverse_char_map(tree: &BinaryTree<Symbol>, base_sum: u32) -> CharMap {
    let mut map = CharMap::new();

    match tree {
        &BinaryTree::Leaf(ref key) => {
            let insert_value = ((base_sum << 1) + 1) << base_sum.leading_zeros() - 1;
            let insert_value = insert_value << 1;
            println!(
                "ins: {:#034b} b sum: {:b} bx sum: {:b}",
                insert_value,
                base_sum,
                ((base_sum << 1) + 1)
            );

            map.insert(*key, insert_value);
        }
        &BinaryTree::Node(ref left, ref right) => {
            let next_base_sum = base_sum << 1;

            map.extend(
                internal_build_inverse_char_map(left.as_ref(), next_base_sum + 1).into_iter(),
            );
            map.extend(internal_build_inverse_char_map(right.as_ref(), next_base_sum).into_iter());
        }
    }

    map
}
fn build_inverse_char_map(tree: &BinaryTree<Symbol>) -> CharMap {
    internal_build_inverse_char_map(tree, 1)
}

pub fn encode_string(map: CharMap, input_string: String) -> (Vec<u8>, usize) {
    let mut vec = vec![0 as u8; input_string.len()];

    let mut bit_cursor: usize = 0;
    let mut byte_cursor: usize = 0;

    let len = vec.len();

    for symbol in input_string.chars() {
        match map.get(&symbol) {
            Some(symbol) => {
                // println!("Bit: {}  Byte: {}", bit_cursor, byte_cursor);

                let symbol = *symbol;

                let symbol_size = num_bits::<u32>() - (symbol.trailing_zeros() + 1) as usize;
                let symbol = symbol >> bit_cursor;
                let trailing: u32 = symbol.trailing_zeros();

                let symbol = symbol >> (trailing + 1) << trailing + 1;

                // println!("{:#034b} {}", symbol, symbol_size);

                for (index, byte) in symbol.to_be_bytes().iter().enumerate() {
                    vec[byte_cursor + index * ((byte_cursor + index < len) as usize)] |= *byte;
                }

                bit_cursor += symbol_size;

                byte_cursor += bit_cursor / 8;
                bit_cursor %= 8;
            }
            None => panic!("Symbol map lacking symbol {}", symbol),
        }
    }

    // Bit offset has to be encoded in header
    vec.resize(byte_cursor + (bit_cursor > 0) as usize, 0);
    (vec, bit_cursor)
}

pub fn encoder(input: String) {
    let frequencies = frequency_extractor(&input);

    let mut queue = build_queue(frequencies);

    let tree = build_tree(&mut queue);

    println!("{:#?}", tree);

    let inverse_char_map = build_inverse_char_map(&tree);

    let (out, bit_offset) = encode_string(inverse_char_map, input);

    for i in out {
        print!("{:08b} ", i);
    }
    print!("-{}-", bit_offset);
}

#[cfg(test)]
mod tests {
    use crate::huff_encoder::*;

    const test_str: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";

    #[test]
    fn it_extracts_frequencies() {
        let test_string: String = String::from(test_str);

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
    fn it_builds_the_optimal_queue() {
        todo!();

        use itertools::Itertools;

        let test_string: String = String::from(test_str);

        let frequencies = frequency_extractor(&test_string);

        let mut queue = build_queue(frequencies);

        // for (key, priority) in frequencies
        //     .into_iter()
        //     .sorted_by_key(|(_, priority)| -(*priority as i32))
        // {
        //     assert_eq!(queue.pop(), Some((BinaryTree::Leaf(key), -(priority as i32))))
        // }
    }

    #[test]
    fn it_has_unambiguous_inverse_char_map() {
        let test_string: String = String::from(test_str);

        let frequencies = frequency_extractor(&test_string);

        let mut queue = build_queue(frequencies);

        let tree = build_tree(&mut queue);

        let inverse_char_map = build_inverse_char_map(&tree);

        for (char, key) in inverse_char_map.into_iter() {
            println!("{}: {:#032b}", char, key);
        }
    }
}
