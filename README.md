# rust-pangenome-intersect
 
 - rust pangenome bedtools intersect.
 - deals with the cpg, exon, and other bindings overlap. 
 - in addition to bedtools, this pangenome bedtools reports also the overlap sequence, sequence of the bed and the features that dont overlap.  
 - to make it multithreaded use rayon.
 - please see the last commit message and if it says compiled binary then it is completed or else still in development version.

 ```
 cargo build 
 
 ```

 ```
 Usage: rust-pangenome-intersect <BED1> <BED2> <FASTA>

 Arguments:
  <BED1>   please provide the path to the first alignment file
  <BED2>   please provide the reference fasta file
  <FASTA>  please provide the path to the reference fasta file

 Options:
  -h, --help     Print help
  -V, --version  Print version
 ```

 Gaurav Sablok
