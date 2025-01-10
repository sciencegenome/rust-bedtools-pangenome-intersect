mod args;
mod intersect;
use crate::intersect::CompareValues;
use crate::intersect::Fasta;
use crate::intersect::IntersectBed1;
use crate::intersect::IntersectBed2;
use args::PangenomeIntersectArgs;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-1-10

 bedtools intersect for the pangenome.

*/

fn main() {
    let args = PangenomeIntersectArgs::parse();
    let output = pangenome_intersect(&args.bed1, &args.bed2, &args.fasta).unwrap();
    println!("Results have been written:{}", output);
}

fn pangenome_intersect(path1: &str, path2: &str, path3: &str) -> Result<String, Box<dyn Error>> {
    let bed1_open = File::open(path1).expect("file not present");
    let bed2_open = File::open(path2).expect("file not present");
    let bed1_read = BufReader::new(bed1_open);
    let bed2_read = BufReader::new(bed2_open);

    let mut bed1: Vec<IntersectBed1> = Vec::new();
    let mut bed2: Vec<IntersectBed2> = Vec::new();

    for i in bed1_read.lines() {
        let line = i.expect("line not present");
        let linevec = line.split("\t").collect::<Vec<_>>();
        bed1.push(IntersectBed1 {
            bed1: linevec[0].to_string(),
            start1: linevec[1].parse::<usize>().unwrap(),
            end1: linevec[2].parse::<usize>().unwrap(),
            matchytpe1: linevec[3].to_string(),
        });
    }

    for i in bed2_read.lines() {
        let line = i.expect("line not present");
        let linevec = line.split("\t").collect::<Vec<_>>();
        bed2.push(IntersectBed2 {
            bed2: linevec[0].to_string(),
            start2: linevec[1].parse::<usize>().unwrap(),
            end2: linevec[2].parse::<usize>().unwrap(),
            matchtype2: linevec[3].to_string(),
        });
    }

    let fastaslice: Vec<Fasta> = Vec::new();

    let mut compare_value_bed1: Vec<CompareValues> = Vec::new();
    let mut compare_value_bed2: Vec<CompareValues> = Vec::new();
    let mut compare_value_bed3: Vec<CompareValues> = Vec::new();

    for i in bed1.iter() {
        for j in bed2.iter() {
            for value in fastaslice.iter() {
                if i.start1 < j.start2 && i.end1 > j.end2 && i.bed1 == value.sequence {
                    compare_value_bed1.push(CompareValues {
                        bed1: i.bed1.clone(),
                        bed2: j.bed2.clone(),
                        start1: i.start1,
                        end1: i.end1,
                        start2: j.start2,
                        end2: j.end2,
                        matchtype1: i.matchytpe1.clone(),
                        matchtype2: j.matchtype2.clone(),
                        difference: j.end2 - j.start2,
                        sequencebed1: Some(value.sequence[i.start1..i.end1].to_string()),
                        sequencebed2: Some(value.sequence[j.start2..j.end2].to_string()),
                        overlap: Some(value.sequence[j.start2..j.end2].to_string()),
                    })
                }
            }
        }
    }

    for i in bed1.iter() {
        for j in bed2.iter() {
            for value in fastaslice.iter() {
                if i.start1 > j.start2 && i.end1 < j.end2 && i.bed1 == value.sequence {
                    compare_value_bed2.push(CompareValues {
                        bed1: i.bed1.clone(),
                        bed2: j.bed2.clone(),
                        start1: i.start1,
                        end1: i.end1,
                        start2: j.start2,
                        end2: j.end2,
                        matchtype1: i.matchytpe1.clone(),
                        matchtype2: j.matchtype2.clone(),
                        difference: i.end1 - i.start1,
                        sequencebed1: Some(value.sequence[i.start1..i.end1].to_string()),
                        sequencebed2: Some(value.sequence[j.start2..j.end2].to_string()),
                        overlap: Some(
                            value.sequence[i.start1..i.end1]
                                .chars()
                                .rev()
                                .map(|x| String::from(x))
                                .collect::<Vec<_>>()
                                .join(""),
                        ),
                    })
                }
            }
        }
    }

    for i in bed1.iter() {
        for j in bed2.iter() {
            for value in fastaslice.iter(){
            if j.start2 < i.start1 && i.start1 < j.end2 && j.end2 < i.end1 {
                compare_value_bed3.push(CompareValues {
                    bed1: i.bed1.clone(),
                    bed2: j.bed2.clone(),
                    start1: i.start1,
                    end1: i.end1,
                    start2: j.start2,
                    end2: j.end2,
                    matchtype1: i.matchytpe1.clone(),
                    matchtype2: j.matchtype2.clone(),
                    difference: j.end2 - i.start1,
                    sequencebed1:Some(value.sequence[i.start1..i.end1].to_string()),
                    sequencebed2: Some(value.sequence[j.start2..j.end2].to_string()),

                })
            }
        }
    }
    }
    println!("{:?}", compare_value_bed3);

    Ok("pangenome intersect results have been written".to_string())
}

fn fasta_estimate(path: &str) -> Result<Vec<Fasta>, Box<dyn Error>> {
    let fastaopen = File::open(path).expect("file not present");
    let fastaread = BufReader::new(fastaopen);
    let mut fastaholder: Vec<Fasta> = Vec::new();
    let mut fastaheader: Vec<String> = Vec::new();
    let mut fastasequence: Vec<String> = Vec::new();
    for i in fastaread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            fastaheader.push(line.replace(">", ""));
        } else {
            fastasequence.push(line);
        }
    }

    for i in 0..fastaheader.len() {
        fastaholder.push(Fasta {
            header: fastaheader[i].clone(),
            sequence: fastasequence[i].clone(),
        })
    }

    Ok(fastaholder)
}
