use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct SliceSlidingWindows<'a, T> {
    slice: &'a [T],
    width: usize,
    stride: usize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for SliceSlidingWindows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.len() == 0 {
            None
        } else {
            let window = self.slice.get(0..self.width).unwrap_or(&self.slice[..]);
            let slice = self.slice.get(self.stride..).unwrap_or(&self.slice[0..0]);
            self.slice = slice;
            Some(window)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let s = self.slice.len() / self.stride;
        (s, Some(s+1))
    }
}

pub fn sliding_windows_from_slice<'a, T>(slice: &'a [T], width: usize, stride: usize) -> SliceSlidingWindows<'a, T> {
    SliceSlidingWindows {
        slice: slice,
        width: width,
        stride: stride,
        _phantom: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct IterSlidingWindows<I>
where
    I: Iterator,
    <I as Iterator>::Item: std::fmt::Debug + Clone,
{
    iter: I,
    width: usize,
    stride: usize,
    buffer: Vec<<I as Iterator>::Item>,
}

impl<I> Iterator for IterSlidingWindows<I>
where
    I: Iterator,
    <I as Iterator>::Item: std::fmt::Debug + Clone,
{
    type Item = Vec<<I as Iterator>::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() {
            for _ in 0..self.width {
                match self.iter.next() {
                    Some(item) => { self.buffer.push(item); },
                    None => break,
                }
            }
            if self.buffer.is_empty() {
                None
            } else {
                Some(self.buffer.clone())
            }
        } else {
            let mut window = Vec::with_capacity(self.width);
            if self.width <= self.stride {
                for _ in 0..(self.stride-self.width) {
                    match self.iter.next() {
                        Some(_) => continue,
                        None => break,
                    }
                }
                for _ in 0..self.width {
                    match self.iter.next() {
                        Some(item) => { window.push(item); }
                        None => break,
                    }
                }
                if window.is_empty() {
                    return None;
                }
            } else {
                match self.buffer.get(self.stride..) {
                    Some(slice) => {
                        window.extend_from_slice(slice);
                        for _ in 0..self.stride {
                            match self.iter.next() {
                                Some(item) => { window.push(item); }
                                None => break,
                            }
                        }
                        if window.is_empty() {
                            return None;
                        }
                    },
                    None => {
                        return None;
                    },
                }
            }
            self.buffer = window.clone();
            Some(window)
        }
    }
}

pub fn sliding_windows_from_iter<I>(iter: I, width: usize, stride: usize) -> IterSlidingWindows<I>
where
    I: Iterator,
    <I as Iterator>::Item: std::fmt::Debug + Clone,
{
    IterSlidingWindows {
        iter: iter,
        width: width,
        stride: stride,
        buffer: Vec::with_capacity(width),
    }
}

#[derive(Debug, Clone)]
pub struct ExactSliceSlidingWindows<'a, T> {
    sliding_windows: SliceSlidingWindows<'a, T>,
    _phantom: PhantomData<&'a T>,
}

pub fn exact_sliding_windows_from_slice<'a, T>(slice: &'a [T], width: usize, stride: usize) -> ExactSliceSlidingWindows<'a, T> {
    ExactSliceSlidingWindows {
        sliding_windows: sliding_windows_from_slice(slice, width, stride),
        _phantom: PhantomData,
    }
}

impl<'a, T> Iterator for ExactSliceSlidingWindows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        match self.sliding_windows.next() {
            Some(w) if w.len() < self.sliding_windows.width => None,
            Some(w) => Some(w),
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExactIterSlidingWindows<I>
where
    I: Iterator,
    <I as Iterator>::Item: std::fmt::Debug + Clone,
{
    sliding_windows: IterSlidingWindows<I>,
}

pub fn exact_sliding_windows_from_iter<I>(iter: I, width: usize, stride: usize) -> ExactIterSlidingWindows<I>
where
    I: Iterator,
    <I as Iterator>::Item: std::fmt::Debug + Clone,
{
    ExactIterSlidingWindows {
        sliding_windows: sliding_windows_from_iter(iter, width, stride),
    }
}

impl<I> Iterator for ExactIterSlidingWindows<I>
where
    I: Iterator,
    <I as Iterator>::Item: std::fmt::Debug + Clone,
{
    type Item = Vec<<I as Iterator>::Item>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.sliding_windows.next() {
            Some(w) => {
                if w.len() < self.sliding_windows.width {
                    None
                } else {
                    Some(w)
                }
            },
            None => None,
        }
    }
}
