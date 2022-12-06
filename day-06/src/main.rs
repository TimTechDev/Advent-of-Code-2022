use std::collections::HashSet;

fn main() {
    let data = std::fs::read_to_string("./data/06.txt").unwrap();
    println!(
        "Day 06: The first StartofPackage Marker is at index {}",
        start_of_packet_marker_pos(&data).unwrap()
    );
    println!(
        "Day 06: The first StartofMessage Marker is at index {}",
        start_of_message_marker_pos(&data).unwrap()
    );
}

fn only_unique_bytes_in_sliding_window(data: &[u8], window_length: usize) -> Option<usize> {
    if window_length > data.len() {
        return None;
    }
    let mut i = window_length;
    while i < data.len() {
        let window: HashSet<&u8> = HashSet::from_iter(&data[i - window_length..i]);
        if window.len() == window_length {
            return Some(i);
        }
        i += 1;
    }
    return None;
}

fn start_of_packet_marker_pos(data: &str) -> Option<usize> {
    return only_unique_bytes_in_sliding_window(data.as_bytes(), 4);
}

fn start_of_message_marker_pos(data: &str) -> Option<usize> {
    return only_unique_bytes_in_sliding_window(data.as_bytes(), 14);
}

#[cfg(test)]
mod day_06 {
    use super::*;

    #[test]
    fn start_of_packet_marker_index() {
        assert_eq!(
            Some(7),
            start_of_packet_marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        );
        assert_eq!(
            Some(5),
            start_of_packet_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz")
        );
        assert_eq!(
            Some(6),
            start_of_packet_marker_pos("nppdvjthqldpwncqszvftbrmjlhg")
        );
        assert_eq!(
            Some(10),
            start_of_packet_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
        assert_eq!(
            Some(11),
            start_of_packet_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }

    #[test]
    fn start_of_message_marker_index() {
        assert_eq!(
            Some(19),
            start_of_message_marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        );
        assert_eq!(
            Some(23),
            start_of_message_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz")
        );
        assert_eq!(
            Some(23),
            start_of_message_marker_pos("nppdvjthqldpwncqszvftbrmjlhg")
        );
        assert_eq!(
            Some(29),
            start_of_message_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
        assert_eq!(
            Some(26),
            start_of_message_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }
}
