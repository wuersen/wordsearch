type WordPuzzle = [[char; 10]; 6];

#[derive(Debug, Clone, PartialEq)]
struct SearchResult {
    bluey: [[u8; 2]; 2],
    bingo: [[u8; 2]; 2],
    humpty: [[u8; 2]; 2],
}

fn word_search(puzzle: WordPuzzle) -> SearchResult {
    // Add your code here
    let result = SearchResult {
        bluey: [[2, 5], [1, 2]],
        bingo: [[2, 5], [1, 2]],
        humpty: [[2, 5], [1, 2]],
    };
    return result;
}

const EASY_WORD_PUZZLE: WordPuzzle = [
    ['h', 'b', 'i', 'n', 'e', 'a', 'b', 'c', 'd', 'e'],
    ['a', 'b', 'c', 'b', 'i', 'n', 'g', 'o', 'd', 'e'],
    ['a', 'h', 'u', 'm', 'p', 't', 'y', 'o', 'd', 'e'],
    ['h', 'b', 'i', 'n', 'e', 'a', 'b', 'c', 'd', 'e'],
    ['h', 'b', 'i', 'b', 'l', 'u', 'e', 'y', 'd', 'e'],
    ['h', 'b', 'i', 'b', 'l', 'u', 'o', 'z', 'd', 'e'],
];

const MEDIUM_WORD_PUZZLE: WordPuzzle = [
    ['h', 'b', 'i', 'n', 'e', 'a', 'b', 'c', 'h', 'e'],
    ['a', 'i', 'c', 'b', 'i', 'n', 'z', 'p', 'u', 'e'],
    ['a', 'n', 'u', 'm', 'p', 't', 'y', 'o', 'm', 'e'],
    ['h', 'g', 'i', 'n', 'e', 'a', 'b', 'c', 'p', 'e'],
    ['h', 'o', 'i', 'b', 'l', 'u', 'e', 'y', 't', 'e'],
    ['h', 'b', 'i', 'b', 'l', 'u', 'o', 'z', 'y', 'e'],
];

const HARD_WORD_PUZZLE: WordPuzzle = [
    ['h', 'b', 'i', 'n', 'e', 'a', 'b', 'c', 'h', 'e'],
    ['a', 'i', 'c', 'b', 'i', 'o', 'z', 'p', 'u', 'e'],
    ['a', 'g', 'u', 'm', 'g', 't', 'y', 'o', 'm', 'e'],
    ['h', 'g', 'i', 'n', 'e', 'a', 'b', 'c', 'p', 'e'],
    ['h', 'o', 'i', 'b', 'l', 'u', 'e', 'y', 't', 'e'],
    ['h', 'b', 'i', 'b', 'l', 'u', 'o', 'z', 'y', 'e'],
];

fn main() {
    word_search(EASY_WORD_PUZZLE);
}

#[test]
fn word_search_test_easy() {
    let result = word_search(EASY_WORD_PUZZLE);
    assert_eq!(
        result,
        SearchResult {
            bluey: [[2, 5], [1, 3]],
            bingo: [[2, 5], [1, 2]],
            humpty: [[2, 5], [1, 2]],
        }
    );
}

#[test]
fn word_search_test_medum() {
    let result = word_search(MEDIUM_WORD_PUZZLE);
    assert_eq!(
        result,
        SearchResult {
            bluey: [[2, 5], [1, 3]],
            bingo: [[2, 5], [1, 2]],
            humpty: [[2, 5], [1, 2]],
        }
    );
}

#[test]
fn word_search_test_hard() {
    let result = word_search(HARD_WORD_PUZZLE);
    assert_eq!(
        result,
        SearchResult {
            bluey: [[2, 5], [1, 3]],
            bingo: [[2, 5], [1, 2]],
            humpty: [[2, 5], [1, 2]],
        }
    );
}
