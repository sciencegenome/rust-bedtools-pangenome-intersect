/*
 * holding all the structs in the separate files so that they
 * can be easily called as a reference call in the result.
 *
 *
 * */
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct IntersectBed1 {
    pub bed1: String,
    pub start1: usize,
    pub end1: usize,
    pub matchytpe1: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct IntersectBed2 {
      pub bed2: String,
      pub start2: usize,
      pub end2: usize,
      pub matchtype2: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct CompareValues {
   pub bed1: String,
   pub start1: usize,
   pub end1: usize,
   pub matchtype1: String,
   pub bed2: String,
   pub start2: usize,
   pub end2: usize,
   pub matchtype2: String,
   pub difference: usize,
   pub sequencebed1: Option<String>,
   pub sequencebed2: Option<String>,
   pub overlap: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Fasta {
    pub header: String,
    pub sequence: String,
}
