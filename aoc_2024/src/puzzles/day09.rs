#[test]
fn test() {
    solve(String::from("2333133121414131402"));
}

pub fn solve(data: String) {
    let chars: Vec<char> = data.trim().chars().collect();
    let mut file_lengths = Vec::new();
    let mut free_lengths = Vec::new();

    for i in 0..chars.len() {
        let val = chars[i].to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            // Even index: file length
            file_lengths.push(val);
        } else {
            // Odd index: free length
            free_lengths.push(val);
        }
    }

    let mut file_id = 0;
    let mut disk = Vec::new();

    // Build the disk layout
    for i in 0..file_lengths.len() {
        let flen = file_lengths[i];
        for _ in 0..flen {
            disk.push(file_id);
        }
        if flen > 0 {
            file_id += 1;
        }

        if i < free_lengths.len() {
            let slen = free_lengths[i];
            for _ in 0..slen {
                disk.push(-1); // Using -1 to represent empty space
            }
        }
    }

    // Compact the disk
    loop {
        let mut last_file_pos = -1;
        for i in (0..disk.len()).rev() {
            if disk[i] != -1 {
                last_file_pos = i as i32;
                break;
            }
        }
        if last_file_pos == -1 {
            break;
        }

        let mut gap_pos = -1;
        for i in 0..disk.len() {
            if disk[i] == -1 {
                gap_pos = i as i32;
                break;
            }
        }

        if gap_pos == -1 || gap_pos >= last_file_pos {
            break;
        }

        let gap_pos = gap_pos as usize;
        let rightmost_file_pos = last_file_pos as usize;

        disk[gap_pos] = disk[rightmost_file_pos];
        disk[rightmost_file_pos] = -1;
    }

    let mut checksum_part1 = 0u128;
    for i in 0..disk.len() {
        if disk[i] != -1 {
            checksum_part1 += (i as u128) * (disk[i] as u128);
        }
    }

    println!("Part 1: {}", checksum_part1);

    // Rebuild the original disk layout
    file_id = 0;
    disk.clear();
    for i in 0..file_lengths.len() {
        let flen = file_lengths[i];
        for _ in 0..flen {
            disk.push(file_id);
        }
        if flen > 0 {
            file_id += 1;
        }

        if i < free_lengths.len() {
            let slen = free_lengths[i];
            for _ in 0..slen {
                disk.push(-1);
            }
        }
    }

    let total_files = file_id;

    // Find contiguous free spans
    fn find_free_spans(disk: &Vec<i32>) -> Vec<(usize, usize)> {
        let mut spans = Vec::new();
        let mut start = -1;

        for i in 0..disk.len() {
            if disk[i] == -1 {
                if start == -1 {
                    start = i as i32;
                }
            } else if start != -1 {
                spans.push((start as usize, i - 1));
                start = -1;
            }
        }
        if start != -1 {
            spans.push((start as usize, disk.len() - 1));
        }
        spans
    }

    // Move whole files
    for fid in (0..total_files).rev() {
        let mut positions = Vec::new();
        for i in 0..disk.len() {
            if disk[i] == fid as i32 {
                positions.push(i);
            }
        }

        if positions.is_empty() {
            continue;
        }

        let flen = positions.len();
        let leftmost_file_pos = positions[0];
        let free_spans = find_free_spans(&disk);

        let mut candidate_span = (-1, -1);
        for span in free_spans {
            if span.1 < leftmost_file_pos && (span.1 - span.0 + 1) >= flen {
                candidate_span = (span.0 as i32, span.1 as i32);
                break;
            }
        }

        if candidate_span.0 != -1 {
            let span_start = candidate_span.0 as usize;
            for &pos in &positions {
                disk[pos] = -1;
            }

            for offset in 0..flen {
                disk[span_start + offset] = fid as i32;
            }
        }
    }

    let mut checksum_part2 = 0u128;
    for i in 0..disk.len() {
        if disk[i] != -1 {
            checksum_part2 += (i as u128) * (disk[i] as u128);
        }
    }

    println!("Part 2: {}", checksum_part2);
}
