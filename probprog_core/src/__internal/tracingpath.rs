use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone)]
pub enum TracingPathFolderRec<'a> {
    Function(String),
    Loop(usize),
    Recursion(&'a String),
}
//^ The reason we have `Recursion(&'a String)` here is that we want to be able
//to efficiently check whether in `descend_function`, we should take `Function`
//or `Recursion`. If we change to doing `descend_function`/`descend_recursion`
//at the call site, we no longer will need `Recursion` to refer to anything, and
//we can unify `TracingPathFolderRec` and `TracingPathFolder` again.

#[derive(Debug, Clone)]
enum TracingDirectory<'a> {
    Root,
    Descend(TracingPathFolderRec<'a>, &'a TracingDirectory<'a>),
}

#[derive(Debug, Clone)]
pub struct TracingPathRec<'a>(TracingDirectory<'a>, usize);

impl<'a> TracingPathRec<'a> {
    pub fn new() -> Self {
        Self(TracingDirectory::Root, 0)
    }

    fn descend(
        &'a self,
        folder: TracingPathFolderRec<'a>,
    ) -> TracingPathRec<'a> {
        Self(TracingDirectory::Descend(folder, &self.0), 0)
    }

    pub fn descend_function(&'a self, function: &str) -> TracingPathRec<'a> {
        match &self.0 {
            TracingDirectory::Descend(TracingPathFolderRec::Function(f), _)
                if f == function =>
            {
                self.descend(TracingPathFolderRec::Recursion(&f))
            }
            TracingDirectory::Descend(
                TracingPathFolderRec::Recursion(f),
                _,
            ) if *f == function => {
                self.descend(TracingPathFolderRec::Recursion(*f))
            }
            _ => self
                .descend(TracingPathFolderRec::Function(function.to_string())),
        }
    }

    pub fn descend_loop(&'a self) -> TracingPathRec<'a> {
        self.descend(TracingPathFolderRec::Loop(0))
    }

    pub fn increment_loop(&mut self) {
        if let Self(
            TracingDirectory::Descend(TracingPathFolderRec::Loop(i), _),
            j,
        ) = self
        {
            *i += 1;
            *j = 0;
        }
    }

    pub fn next_variable(&mut self) -> TracingPath {
        let r = (&*self).into();
        self.1 += 1;
        r
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]

pub enum TracingPathFolder {
    Function(String),
    Loop(usize),
    Recursion,
}

impl From<&TracingPathFolderRec<'_>> for TracingPathFolder {
    fn from(value: &TracingPathFolderRec<'_>) -> Self {
        match value {
            TracingPathFolderRec::Function(f) => {
                TracingPathFolder::Function(f.clone())
            }
            TracingPathFolderRec::Loop(i) => TracingPathFolder::Loop(*i),
            TracingPathFolderRec::Recursion(_) => TracingPathFolder::Recursion,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TracingPath(Vec<TracingPathFolder>, usize);

impl Display for TracingPath {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for folder in &self.0 {
            match folder {
                TracingPathFolder::Function(f) => {
                    write!(fmt, "/{}", f)?;
                }
                TracingPathFolder::Loop(i) => {
                    write!(fmt, "/@{}", i)?;
                }
                TracingPathFolder::Recursion => {
                    write!(fmt, "/.")?;
                }
            }
        }
        write!(fmt, "/{}", self.1)
    }
}

impl From<&TracingPathRec<'_>> for TracingPath {
    fn from(value: &TracingPathRec<'_>) -> Self {
        /// This function descends into `dir` and collects all folders in
        /// correct order in `vec`. To do so, we pass a mutable vec through.
        fn collect(
            mut vec: VecDeque<TracingPathFolder>,
            dir: &TracingDirectory<'_>,
        ) -> VecDeque<TracingPathFolder> {
            match dir {
                TracingDirectory::Root => vec,
                TracingDirectory::Descend(f, d) => {
                    vec.push_front(f.into());
                    collect(vec, d)
                }
            }
        }
        let v = collect(VecDeque::new(), &value.0).into();
        TracingPath(v, value.1)
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub struct TracingPathEntry(TracingPathFolder, usize);

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub struct TracingPath(Vec<TracingPathEntry>);

// impl TracingPath {
//     pub fn new() -> Self {
//         TracingPath(Vec::new())
//     }

//     pub fn push(&mut self, folder: TracingPathFolder) {
//         self.0.push(TracingPathEntry(folder, 0));
//     }

//     pub fn pop(&mut self) {
//         self.0.pop();
//     }

//     pub fn next_variable(&mut self) -> Option<TracingPathEntry> {
//         let r = self.0.last_mut()?;
//         r.1 += 1;
//         Some(r.clone())
//     }
// }

