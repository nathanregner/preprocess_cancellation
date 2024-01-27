use std::io::{self, BufRead, Seek, SeekFrom, Write};
use std::ops::Range;

#[derive(Default, Debug)]
pub struct Patch {
    edits: Vec<Hunk>,
}

impl Patch {
    pub fn new() -> Self {
        Self { edits: Vec::new() }
    }

    pub fn replace(&mut self, range: Range<u64>, text: impl Into<String>) {
        self.edits.push(Hunk::Replace(range, text.into()));
    }

    pub fn insert(&mut self, pos: u64, text: impl Into<String>) {
        self.edits.push(Hunk::Insert(pos, text.into()));
    }

    pub fn apply(mut self, mut src: impl BufRead + Seek, mut dst: impl Write) -> crate::Result<()> {
        src.seek(SeekFrom::Start(0))?;

        self.edits.sort_by_key(|e| match e {
            Hunk::Insert(pos, _) => *pos,
            Hunk::Replace(range, _) => range.start,
        });

        for edit in self.edits {
            match edit {
                Hunk::Insert(pos, text) => {
                    copy_to(&mut src, &mut dst, pos)?;
                    write!(dst, "{}", text)?;
                }
                Hunk::Replace(range, text) => {
                    copy_to(&mut src, &mut dst, range.start)?;
                    write!(dst, "{}", text)?;
                    src.seek(SeekFrom::Current(range.clone().count() as i64))?;
                }
            }
        }

        io::copy(&mut src, &mut dst)?;
        dst.flush()?;
        Ok(())
    }
}

#[derive(Debug)]
enum Hunk {
    Insert(u64, String),
    Replace(Range<u64>, String),
}

fn copy_to(mut src: impl BufRead + Seek, dst: &mut impl Write, end: u64) -> io::Result<u64> {
    let pos = src.stream_position()?;
    let count = end - pos;
    io::copy(&mut src.take(count), dst)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn apply() {
        let src = "Line 1\n\
                   L 3\n\
                   Line 5\n";

        let mut patch = Patch::new();
        patch.insert(7, "Line 2\n");
        patch.replace(7..8, "Line");
        patch.insert(11, "Line 4\n");

        let mut buf = vec![];
        patch
            .apply(Cursor::new(src), Cursor::new(&mut buf))
            .expect("to apply");

        assert_eq!(
            String::from_utf8(buf).expect("to utf8"),
            "Line 1\n\
             Line 2\n\
             Line 3\n\
             Line 4\n\
             Line 5\n"
        );
    }
}
