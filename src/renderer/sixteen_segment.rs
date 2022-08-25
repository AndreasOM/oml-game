use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::math::Vector2;

pub struct SixteenSegment {}

/*
		 a1     a2
		----   ----
	  | \h  i|   j/ |
	f |  \   |   /  | b
	  |   \  |  /   |
	  |    \ | /    |

		----   ----
		 g1     g2
	  |     /| \    |
	e |    / |  \   | c
	  |   /  |   \  |
	  |  /k  |l  m\ |
		----   ----
		 d1     d2

	0.0 / 0.0 => lower left
	a1
	a2
	b
	c
	d1
	d2
	e
	f
	g1
	g2
	h
	i
	j
	k
	l
	m

*/

const POINTS: &[Vector2] = &[
	Vector2::new(0.0, 1.0),  // 0 - top left
	Vector2::new(0.25, 1.0), // 1 - top middle
	Vector2::new(0.5, 1.0),  // 2 - top right
	Vector2::new(0.0, 0.5),  // 3 - left
	Vector2::new(0.25, 0.5), // 4 - middle
	Vector2::new(0.5, 0.5),  // 5 - right
	Vector2::new(0.0, 0.0),  // 6 - bottom left
	Vector2::new(0.25, 0.0), // 7 - bottom middle
	Vector2::new(0.5, 0.0),  // 8 - bottom right
];

const SEGMENTS: &[(usize, usize)] = &[
	(0, 1), // a1
	(1, 2), // a2
	(2, 5), // b
	(5, 8), // c
	(6, 7), // d1
	(7, 8), // d2
	(6, 3), // e
	(3, 0), // f
	(3, 4), // g1
	(4, 5), // g2
	(0, 4), // h
	(1, 4), // i
	(2, 4), // j
	(6, 4), // k
	(7, 4), // l
	(8, 4), // m
];

/*
		 a1     a2
		----   ----
	  | \h  i|   j/ |
	f |  \   |   /  | b
	  |   \  |  /   |
	  |    \ | /    |

		----   ----
		 g1     g2
	  |     /| \    |
	e |    / |  \   | c
	  |   /  |   \  |
	  |  /k  |l  m\ |
		----   ----
		 d1     d2
*/
lazy_static! {
	static ref CHARACTERS: HashMap<char, [u8;16]> = {
		let mut map = HashMap::new();
		// :TODO: we could just have a list with enabled segments, but I find this more readable, for now
		//               a a b c d d e f g g h i j k l m
		map.insert('0', [1,1,1,1,1,1,1,1,0,0,0,0,1,1,0,0]);
		map.insert('1', [0,0,0,0,0,0,0,0,0,0,0,1,0,0,1,0]);
		map.insert('2', [1,1,1,0,1,1,1,0,1,1,0,0,0,0,0,0]);
		map.insert('3', [1,1,1,1,1,1,0,0,1,1,0,0,0,0,0,0]);
		map.insert('4', [0,0,1,1,0,0,0,1,1,1,0,0,0,0,0,0]);
		map.insert('5', [1,1,0,1,1,1,0,1,1,1,0,0,0,0,0,0]);
		map.insert('6', [1,1,0,1,1,1,1,1,1,1,0,0,0,0,0,0]);
		map.insert('7', [1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0]);
		map.insert('8', [1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0]);
		map.insert('9', [1,1,1,1,1,1,0,1,1,1,0,0,0,0,0,0]);
		map.insert('A', [1,1,1,1,0,0,1,1,1,1,0,0,0,0,0,0]);
		map.insert('B', [1,1,1,1,1,1,0,0,0,1,0,1,0,0,1,0]);
		map.insert('C', [1,1,0,0,1,1,1,1,0,0,0,0,0,0,0,0]);
		map.insert('D', [1,1,1,1,1,1,0,0,0,0,0,1,0,0,1,0]);
		map.insert('E', [1,1,0,0,1,1,1,1,1,1,0,0,0,0,0,0]);
		map.insert('F', [1,1,0,0,0,0,1,1,1,1,0,0,0,0,0,0]);
		map.insert('G', [1,1,0,1,1,1,1,1,0,1,0,0,0,0,0,0]);
		map.insert('H', [0,0,1,1,0,0,1,1,1,1,0,0,0,0,0,0]);
		map.insert('I', [0,0,0,0,0,0,0,0,0,0,0,1,0,0,1,0]);
		map.insert('J', [0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0]);
		map.insert('K', [0,0,0,0,0,0,1,1,0,0,0,0,1,1,0,1]);
		map.insert('L', [0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0]);
		map.insert('M', [1,1,1,1,0,0,1,1,0,0,0,1,0,0,0,0]);
/*
		 a1     a2
		----   ----
	  | \h  i|   j/ |
	f |  \   |   /  | b
	  |   \  |  /   |
	  |    \ | /    |

		----   ----
		 g1     g2
	  |     /| \    |
	e |    / |  \   | c
	  |   /  |   \  |
	  |  /k  |l  m\ |
		----   ----
		 d1     d2
*/
		//               a a b c d d e f g g h i j k l m
		map.insert('N', [0,0,1,1,0,0,1,1,0,0,1,0,0,0,0,1]);
		map.insert('O', [1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0]);
		map.insert('P', [1,1,1,0,0,0,1,1,1,1,0,0,0,0,0,0]);
		map.insert('Q', [1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,1]);
		map.insert('R', [1,1,1,0,0,0,1,1,1,1,0,0,0,0,0,1]);
		map.insert('S', [1,1,0,1,1,1,0,1,1,1,0,0,0,0,0,0]);
		map.insert('T', [1,1,0,0,0,0,0,0,0,0,0,1,0,0,1,0]);
		map.insert('U', [0,0,1,1,1,1,1,1,0,0,0,0,0,0,0,0]);
		map.insert('V', [0,0,0,0,0,0,1,1,0,0,0,0,1,1,0,0]);
		map.insert('W', [0,0,1,1,0,0,1,1,0,0,0,0,0,1,0,1]);
		map.insert('X', [0,0,0,0,0,0,0,0,0,0,1,0,1,1,0,1]);
		map.insert('Y', [0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,0]);
		map.insert('Z', [1,1,0,0,1,1,0,0,0,0,0,0,1,1,0,0]);
		map.insert('-', [0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0]);
		map.insert('+', [0,0,0,0,0,0,0,0,1,1,0,1,0,0,1,0]);
		map.insert('*', [0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1]);
		map.insert('/', [0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0]);
		map.insert(' ', [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
		map
	};
}

impl SixteenSegment {
	pub fn lines_for_character(c: char) -> Vec<(Vector2, Vector2)> {
		let mut rv = Vec::new();

		// v.push( ( Vector2::new( 0.0, 0.0 ), Vector2::new( 1.0, 1.0 ) ) );

		if c == 'V' {
			// :CHEAT: the real 16 seg 'V" looks bad, so we cheat
			rv.push((POINTS[0], POINTS[7]));
			rv.push((POINTS[7], POINTS[2]));
		} else if let Some(cs) = CHARACTERS.get(&c) {
			for (i, v) in cs.iter().enumerate() {
				if *v > 0 {
					let s = SEGMENTS[i];
					rv.push((POINTS[s.0], POINTS[s.1]));
				}
			}
		} else {
			println!("No segments found for {} -> using all", c);
			for s in SEGMENTS {
				rv.push((POINTS[s.0], POINTS[s.1]));
			}
		}

		rv
	}
}
