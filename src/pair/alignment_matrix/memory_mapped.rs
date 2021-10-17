use std::{fmt, fs, io, mem};

use memmap::MmapMut;
use tempdir::TempDir;
use uuid::Uuid;

use pair::cursor::Cursor;
use pair::step_mask::StepMask;

use super::AlignmentMatrix as AlignmentMatrixTrait;

pub struct AlignmentMatrix {
    width: usize,
    height: usize,
    mmap: MmapMut,
}

impl AlignmentMatrix {
    fn offset(&self, cursor: &Cursor) -> usize {
        cursor.x + (cursor.y * self.width)
    }
}

impl AlignmentMatrixTrait for AlignmentMatrix {
    type Error = io::Error;

    fn new(width: usize, height: usize) -> Result<Self, Self::Error> {
        let directory = TempDir::new("seal").unwrap();
        let uuid = Uuid::new_v4();
        let filename = uuid.to_simple().to_string();
        let path = directory.path().join(filename);
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let size = (width + 1) * (height + 1);
        file.set_len(size as u64)?;
        let mmap = unsafe { MmapMut::map_mut(&file)? };
        Ok(Self {
            width,
            height,
            mmap,
        })
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn at(&self, cursor: &Cursor) -> StepMask {
        let offset = self.offset(cursor);
        unsafe {
            let byte = self.mmap.get_unchecked(offset);
            mem::transmute::<u8, StepMask>(*byte)
        }
    }

    fn set_at(&mut self, cursor: &Cursor, step_mask: StepMask) {
        let offset = self.offset(cursor);
        let byte = unsafe { mem::transmute::<StepMask, u8>(step_mask) };
        let byte_ref = unsafe { self.mmap.get_unchecked_mut(offset) };
        *byte_ref = byte;
    }
}

impl fmt::Debug for AlignmentMatrix {
    fn fmt(&self, form: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let cursor = Cursor { x, y };
                let _ = write!(form, "{:?}\t", self.at(&cursor));
            }
            let _ = writeln!(form);
        }
        writeln!(form)
    }
}
