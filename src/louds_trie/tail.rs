use std;
use config::TailMode;
use entry;
use entry::Entry;
use vector::bit_vec::BitVec;

#[derive(Debug)]
pub struct Tail {
    buf_: Vec<u8>,
    end_flags_: BitVec,
}

impl Tail {
    pub fn new() -> Tail {
        Tail { buf_: Vec::new(), end_flags_: BitVec::new() }
    }

    pub fn build<'a>(entries: &mut Vec<Entry<'a>>, offsets: &mut Vec<u32>,
                     mode: TailMode) -> Tail {
        let mode = match mode {
            TailMode::Text => {
                if entries.iter().any(
                  |entry| entry.iter().any(|x| *x == 0)) {
                    TailMode::Binary
                } else {
                    TailMode::Text
                }
            }
            x @ TailMode::Binary => x,
        };

        for (i, entry) in entries.iter_mut().enumerate() {
            assert!(i <= std::u32::MAX as usize);
            entry.set_id(i as u32);
        }

        let mut out = Tail::new();

        // FIXME: marisa-trie used "multi-key quicksort"/"three-way radix
        //        quicksort" here. Consider bringing that back.
        entries.sort_by(&entry::cmp_slice);

        let mut tmp: Vec<u32> = Vec::new();
        tmp.resize(entries.len(), 0);

        let mut optLast: Option<&Entry> = None;
        for entry in entries.iter().rev() {
            assert!(!entry.is_empty(), "MARISA_RANGE_ERROR");

            let doPush = match optLast {
                Some(last) => {
                    if entry.common_count(last) == entry.len() {
                        let diff = last.len() - entry.len();
                        assert!(diff <= std::u32::MAX as usize);
                        let diff = diff as u32;
                        tmp[entry.get_id() as usize] =
                            tmp[last.get_id() as usize] + diff;
                        false
                    } else {
                        true
                    }
                }
                None => true,
            };

            if doPush {
                tmp[entry.get_id() as usize] = out.buf_.len() as u32;

                out.buf_.extend(entry.iter().rev());

                match mode {
                    TailMode::Text => { out.buf_.push(0); },
                    TailMode::Binary => {
                        for _ in 1..entry.len() {
                            out.end_flags_.push(false);
                        }
                        out.end_flags_.push(true);
                    }
                }
                assert!(out.buf_.len() <= std::u32::MAX as usize,
                        "MARISA_SIZE_ERROR");
            }
            optLast = Some(&entry);
        }
        out.buf_.shrink_to_fit();

        *offsets = tmp;
        out
    }

    pub fn restore(&self, offset: usize, key_out: &mut Vec<u8>) {
        assert!(!self.buf_.is_empty(), "MARISA_STATE_ERROR");

        if self.end_flags_.is_empty() {
            for &c in self.buf_.iter().skip(offset) {
                if 0 == c { break; } // null-terminated
                key_out.push(c);
            }
        } else {
            for (i, &c) in self.buf_.iter().skip(offset).enumerate() {
                key_out.push(c);
                if self.end_flags_.at(i + offset) { break; }
            }
        }
    }

/*
    void map(Mapper &mapper);
    void read(Reader &reader);
    void write(Writer &writer) const;
    void map_(Mapper &mapper);
    void read_(Reader &reader);
    void write_(Writer &writer) const;

void Tail::map(Mapper &mapper) {
  Tail temp;
  temp.map_(mapper);
  swap(temp);
}

void Tail::read(Reader &reader) {
  Tail temp;
  temp.read_(reader);
  swap(temp);
}

void Tail::write(Writer &writer) const {
  write_(writer);
}
void Tail::map_(Mapper &mapper) {
  buf_.map(mapper);
  end_flags_.map(mapper);
}

void Tail::read_(Reader &reader) {
  buf_.read(reader);
  end_flags_.read(reader);
}

void Tail::write_(Writer &writer) const {
  buf_.write(writer);
  end_flags_.write(writer);
}
*/

    pub fn clear(&mut self) {
        *self = Tail::new();
    }

/*
    const char &operator[](usize offset) const {
      MARISA_DEBUG_IF(offset >= buf_.len(), MARISA_BOUND_ERROR);
      return buf_[offset];
    }
*/

    pub fn mode(&self) -> TailMode {
        if self.end_flags_.is_empty() { TailMode::Text }
        else { TailMode::Binary }
    }

    pub fn is_empty(&self) -> bool {
        self.buf_.is_empty()
    }
    pub fn len(&self) -> usize {
        self.buf_.len()
    }
/*
    usize total_size() const {
      return buf_.total_size() + end_flags_.total_size();
    }
    usize io_size() const {
      return buf_.io_size() + end_flags_.io_size();
    }
*/
}

