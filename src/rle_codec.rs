use std::iter::FromIterator;

use wasm_bindgen::prelude::*;

use crate::*;

/******************************************************************************
 * Encoder
 *****************************************************************************/
type RLEContent = Vec<(usize, char)>;

#[wasm_bindgen]
impl Universe {
pub fn to_rle(&self) -> String {
    let rle_content = self.cells
        .chunks(self.width as usize)
        .enumerate()
        .flat_map(|(row_index, cells)| {
            let mut row = cells
                .iter().map(|cell| match cell {
                    Cell::Alive => 'o',
                    Cell::Dead => 'b',
                })
                .collect::<Vec<char>>();

            // remove trailing dead cells
            if let Some(last_alive_index) = row
                .iter()
                .rev()
                .position(|&c| c == 'o')
            {
                row.truncate(cells.len() - last_alive_index);
            } else {
                row.clear();
            }
            
            row.push(if row_index as i32 == self.height - 1 {
                '!'
            } else {
                '$'
            });
            row
        })
        .fold(RLEContent::new(), |mut rle_content, c| {
            match rle_content.last_mut() {
                Some(last) if last.1 == c => {
                    last.0 += 1;
                },
                _ => {
                    rle_content.push((1, c));
                },
            }
            rle_content
        });

    let mut rle = String::new();

    // header
    rle.push_str(&format!(
        "x = {}, y = {}, rule = B3/S23",
        self.width, self.height
    ));

    // content
    let rle_content_str = rle_content
        .iter()
        .flat_map(|(count, c)| {
            match count {
                1 => Vec::from_iter(c.to_string().bytes()),
                _ => Vec::from_iter(format!("{}{}", count, c).bytes()),
            }
        })
        .collect::<Vec<u8>>();

    rle_content_str.chunks(70).for_each(|chunk| {
        rle.push_str("\n");
        rle.push_str(&String::from_utf8(chunk.to_vec()).unwrap());
    });

    rle
}}

/******************************************************************************
 * Decoder
 *****************************************************************************/

pub enum RLEDecoderError {
    InternalError,
    InvalidType,
    InvalidNumber,
    InvalidTag,
    InvalidHeader,
}

impl std::fmt::Display for RLEDecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RLEDecoderError::InternalError => write!(f, "internal error"),
            RLEDecoderError::InvalidType => write!(f, "invalid type"),
            RLEDecoderError::InvalidNumber => write!(f, "invalid number"),
            RLEDecoderError::InvalidTag => write!(f, "invalid tag"),
            RLEDecoderError::InvalidHeader => write!(f, "invalid header"),
        }
    }
}

impl From<RLEDecoderError> for JsValue {
    fn from(err: RLEDecoderError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

fn check_if(cond: bool, err: RLEDecoderError) -> Result<(), RLEDecoderError> {
    if cond { Ok(()) } else { Err(err) }
}

enum RLEToken {
    Cell(Cell),
    Count(i32),
    EndOfLine,
}

struct RLETokenIterator {
    str: String,
    curr: usize,
    prev: usize,
}

impl RLETokenIterator {
    fn new(str: String) -> Self {
        Self { str, curr: 0, prev: 0 }
    }

    fn read_number_token(&mut self) -> Result<RLEToken, RLEDecoderError> {
        let bytes = self.str.as_bytes();

        check_if(self.curr < bytes.len(), RLEDecoderError::InternalError)?;

        while self.curr < bytes.len()
            && bytes[self.curr].is_ascii_digit() {
            self.curr += 1;
        }

        let s = std::str::from_utf8(&bytes[self.prev..self.curr])
            .or(Err(RLEDecoderError::InternalError))?;

        let v = s.parse::<i32>()
            .or(Err(RLEDecoderError::InvalidNumber))?;

        self.prev = self.curr;

        return Ok(RLEToken::Count(v));
    }

    fn read_tag_token(&mut self) -> Result<RLEToken, RLEDecoderError> {
        let bytes = self.str.as_bytes();

        check_if(self.curr < bytes.len(), RLEDecoderError::InternalError)?;

        let token = match bytes[self.curr] {
            b'o' => RLEToken::Cell(Cell::Alive),
            b'b' => RLEToken::Cell(Cell::Dead),
            b'$' => RLEToken::EndOfLine,
            _ => return Err(RLEDecoderError::InternalError),
        };

        self.curr += 1;
        self.prev = self.curr;

        return Ok(token);
    }
}

impl Iterator for RLETokenIterator {
    type Item = Result<RLEToken, RLEDecoderError>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.str.as_bytes();

        if self.curr >= bytes.len() {
            return None;
        }

        return match bytes[self.curr] {
            b'0'|b'1'|b'2'|b'3'|b'4'|b'5'|b'6'|b'7'|b'8'|b'9' => {
                // number
                Some(self.read_number_token())
            },
            b'o'|b'b'|b'$' => {
                // tag
                Some(self.read_tag_token())
            },
            b'!' => {
                None
            },
            _ => {
                return Some(Err(RLEDecoderError::InvalidTag));
            },
        }
    }
}

pub fn parse_size_value(dim: &str, s: &str) -> Result<u32, RLEDecoderError> {
    let parts = s
        .trim()
        .split('=')
        .map(str::trim)
        .collect::<Vec<_>>();

    check_if(parts.len() == 2, RLEDecoderError::InvalidHeader)?;
    check_if(parts[0] == dim, RLEDecoderError::InvalidHeader)?;

    parts[1].parse::<u32>().or(Err(RLEDecoderError::InvalidHeader))
}

#[wasm_bindgen]
impl Universe {
pub fn from_rle(value: JsValue) -> Result<Universe, RLEDecoderError> {
    check_if(value.is_string(), RLEDecoderError::InvalidType)?;

    let rle_string = value.as_string().unwrap();
    let mut lines = rle_string
        .lines()
        .filter(|line| !line.starts_with("#"));

    // parse the header
    let headers = lines
        .next().ok_or(RLEDecoderError::InvalidHeader)?
        .split(',').map(str::trim).collect::<Vec<_>>();

    let width = parse_size_value("x", headers[0])?;
    let height = parse_size_value("y", headers[1])?;

    let mut universe = Universe::new(width, height);
    let mut it = RLETokenIterator::new(String::from_iter(lines));
    let mut count = 1;
    let mut row = 0;
    let mut col = 0;

    while let Some(token) = it.next() {
        match token? {
            RLEToken::Count(value) => {
                count = value;
            },
            RLEToken::Cell(cell) => {
                (0..count).for_each(|i| {
                    universe.set_cell(col + i, row, cell);
                });
                col += count;
                count = 1;
            },
            RLEToken::EndOfLine => {
                col = 0;
                row += count;
                count = 1;
            },
        }
    }

    Ok(universe)
}}
