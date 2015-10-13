use std::marker::PhantomData;
use std::{fmt, fs, io, mem};

use memmap::MmapMut;
use tempdir::TempDir;
use uuid::Uuid;

use pair::cursor::Cursor;

use super::{AlignmentMatrix as AlignmentMatrixTrait, AlignmentMatrixCell};

pub struct AlignmentMatrix<T> {
    width: usize,
    height: usize,
    mmap: MmapMut,
    _phantom: PhantomData<T>,
}

impl<T> AlignmentMatrix<T> {
    fn new(width: usize, height: usize, mmap: MmapMut) -> Self {
        let _phantom = PhantomData;
        Self {
            width,
            height,
            mmap,
            _phantom,
        }
    }

    fn offset(&self, cursor: &Cursor) -> usize {
        cursor.x + (cursor.y * self.width)
    }
}

impl<T> AlignmentMatrixTrait for AlignmentMatrix<T> {
    type Score = T;

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
        let count = (width + 1) * (height + 1);
        let len = count * mem::size_of::<AlignmentMatrixCell<Self::Score>>();
        file.set_len(len as u64)?;
        let mmap = unsafe { MmapMut::map_mut(&file)? };
        Ok(Self::new(width, height, mmap))
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn cell(&self, cursor: &Cursor) -> &AlignmentMatrixCell<Self::Score> {
        let offset = self.offset(cursor);
        let mmap_ptr = self.mmap.as_ptr() as *mut AlignmentMatrixCell<Self::Score>;
        unsafe {
            let cell_ptr = mmap_ptr.offset(offset as isize);
            &*cell_ptr
        }
    }

    fn cell_mut(&mut self, cursor: &Cursor) -> &mut AlignmentMatrixCell<Self::Score> {
        let offset = self.offset(cursor);
        let mmap_ptr = self.mmap.as_mut_ptr() as *mut AlignmentMatrixCell<Self::Score>;
        unsafe {
            let cell_ptr = mmap_ptr.offset(offset as isize);
            &mut *cell_ptr
        }
    }
}

impl<T> fmt::Debug for AlignmentMatrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let cursor = Cursor::new(x, y);
                write!(fmt, "{:?}\t", self.cell(&cursor))?;
            }
            writeln!(fmt)?;
        }
        writeln!(fmt)
    }
}
