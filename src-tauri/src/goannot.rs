use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Map;
use num::Integer;
use std::str::FromStr;
use tauri::command;
use serde::Serialize;
use std::sync::Mutex;
use lazy_static::lazy_static;


// Use `lazy_static` to keep the dataset in memory
lazy_static! {
    static ref GO_ANNOTATIONS: Mutex<Vec<GoAnnot>> = Mutex::new(Vec::new());
}

/// Gene product to GO term relations
/// enables links a gene product to a Molecular Function it executes.
/// contributes to links a gene product to a Molecular Function executed by a macromolecular complex, in which the Molecular Function cannot be ascribed to an individual subunit of that complex. Only the complex subunits required for the Molecular Function are annotated to the Molecular Function term with ‘contributes to’.
/// Relations between a gene product and a Biological Process:
/// involved in links a gene product and a Biological Process in which the gene product’s Molecular Function plays an integral role.
/// acts upstream of or within links a gene product and a Biological Process when the mechanism relating the gene product’s activity to the Biological Process is not known.
/// Relations between a gene product and a Cellular Component:
/// is active in links a gene product to the cellular location in which it enables its Molecular Function.
/// located in links a gene product and the Cellular Component, specifically a cellular anatomical anatomy or virion component, in which a gene product has been detected.
/// part of links a gene product and a protein-containing complex.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
enum GoTermRelation {
    Enables,
    ContributesTo,
    InvolvedIn,
    ActsUpstreamOf,
    ActsWithin,
    IsActiveIn,
    LocatedIn,
    PartOf
}

impl std::fmt::Display for GoTermRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let relation_str = match self {
            GoTermRelation::Enables => "enables",
            GoTermRelation::ContributesTo => "contributes_to",
            GoTermRelation::InvolvedIn => "involved_in",
            GoTermRelation::ActsUpstreamOf => "acts_upstream_of",
            GoTermRelation::ActsWithin => "acts_within",
            GoTermRelation::IsActiveIn => "is_active_in",
            GoTermRelation::LocatedIn => "located_in",
            GoTermRelation::PartOf => "part_of"
        };
        write!(f, "{}", relation_str)
    }
}

impl FromStr for GoTermRelation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "enables" => Ok(GoTermRelation::Enables),
            "contributes_to" => Ok(GoTermRelation::ContributesTo),
            "involved_in" => Ok(GoTermRelation::InvolvedIn),
            "located_in" => Ok(GoTermRelation::LocatedIn),
            "acts_upstream_of" => Ok(GoTermRelation::ActsUpstreamOf),
            "acts_within" => Ok(GoTermRelation::ActsWithin),
            "is_active_in" => Ok(GoTermRelation::IsActiveIn),
            "part_of" => Ok(GoTermRelation::PartOf),
            _ => Err(format!("Did not recognize '{}' as a GOA relation.",s)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
enum EviCode {
    EXP, // inferred from experiment
    HTP, //  Inferred from High Throughput Experiment 
    PHYLO, // Phylogenetically inferred annotations
    COMPUTATIONAL, // computational analysis evidence codes i
    AUTHOR, // Author statement evidence
    IC, // Curator statement 
    ND, // No biological Data available 
    IEA, // Inferred from Electronic Annotation (IEA)
}

impl FromStr for EviCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EXP" | "IDA" | "IPI" | "IMP" | "IGI" | "IEP" => Ok(EviCode::EXP),
            "HTP" | "HDA" | "HMP" | "HGI" | "HEP"  => Ok(EviCode::HTP),
            "IBA" | "IBD" | "IKR" | "IRD"  => Ok(EviCode::PHYLO),
            "ISS" | "ISO" | "ISA" | "ISM" |"IGC" |"RCA"  => Ok(EviCode::COMPUTATIONAL),
            "TAS" | "NAS" => Ok(EviCode::AUTHOR),
            "IC" => Ok(EviCode::IC),
            "ND" => Ok(EviCode::ND),
            "IEA" => Ok(EviCode::IEA),
            _ => Err(format!("Did not recognize '{}' as EvidenceCode.",s)),
        }
    }
}
#[derive(Serialize, Clone)]
enum Aspect {
    F,
    P,
    C
}

impl FromStr for Aspect {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F" => Ok(Aspect::F),
            "P" => Ok(Aspect::P),
            "C" => Ok(Aspect::C),
            _ => Err(format!("Did not recognize '{}' as Aspect.",s)),
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct TermId {
    pub value: String
}

impl TermId {
    pub fn new(prfx: &str, id: &str) -> Self {
        TermId {
            value: format!("{}:{}", prfx, id)
        }
    }

    pub fn from_curie(curie: &str) -> Result<Self, String> {
        let tokens: Vec<&str> = curie.split(':').collect();
        if tokens.iter().count() != 2 {
            return Err(format!("CURIE expected to have 2 fields, but had {} fields: {}", 
                tokens.iter().count(), curie));
        }
        Ok( TermId{value: curie.to_string()})
    }
}


#[derive(Clone)]
struct GoAnnot {
    gene_product_id: TermId,
    gene_product_symbol: String,
    relation: GoTermRelation,
    go_id: TermId,
    evidence_code: EviCode,
    aspect: Aspect,
}

impl GoAnnot {

    pub fn new<T: Into<String>>(term: TermId,
                            symbol: T,
                            relation: GoTermRelation,
                            gene_ontology_id: TermId,
                            evicode: EviCode,
                            aspect: Aspect
                         ) -> Self {
        GoAnnot {
            gene_product_id: term,
            gene_product_symbol: symbol.into(),
            relation: relation, 
            go_id: gene_ontology_id,
            evidence_code: evicode,
            aspect: aspect
        }


    }
}


struct GoAnnotations {
    annotation_list: Vec<GoAnnot>,
    version: String
}

/// To be used for serialization to display the most interesting characteristics of the annotation as a table
#[derive(Serialize)]
struct AnnotationStat {
    key: String,
    value: String
}

impl AnnotationStat {
    pub fn from_string(item: &str, val: &str) -> Self {
        AnnotationStat{key: item.to_string(), value: val.to_string()}
    }

    pub fn from_int<T: Integer+std::fmt::Display>(item: &str, val: T) -> Self {
        AnnotationStat{key: item.to_string(), value: format!("{}",val)}
    }
}

fn annotation_descriptive_stats(go_annots: &Vec<GoAnnot>) -> Vec<AnnotationStat> {
    let mut annots = Vec::new();
    let annot_count = go_annots.len();
    annots.push(AnnotationStat::from_int("length", annot_count));
    let unique_symbols: HashSet<_> = go_annots.iter().map(|annot| &annot.gene_product_symbol).collect();
    annots.push(AnnotationStat::from_int("genes", unique_symbols.len()));
    // Count relation types
    let mut relation_counts = HashMap::new();

    for annot in go_annots {
        *relation_counts.entry(annot.relation.clone()).or_insert(0) += 1;
    }
    for (relation, count) in &relation_counts {
        annots.push(AnnotationStat::from_int(&relation.to_string(), *count));
    }
    annots
}

const GOA_EXPECTED_FIELDS: usize = 17;

/// Process a line in go-annotation-file-gaf-format-2.2
fn process_annotation_line(line: &str) -> Result<GoAnnot, String> {
    let tokens: Vec<&str> = line.split('\t').collect();
    if tokens.iter().count() != GOA_EXPECTED_FIELDS {
        return Err(format!("GOA lines expected to have {} fields, but line had {} fields: {}", 
            GOA_EXPECTED_FIELDS, tokens.iter().count(), line));
    }
    let gene_product_id = TermId::new(tokens[0], tokens[1]);
    let symbol = tokens[2];
    let relation = GoTermRelation::from_str(tokens[3])?; // return on error immediately
    let go_id = TermId::from_curie(tokens[4])?;// return on error immediately
    let evidence = EviCode::from_str(tokens[6])?;// return on error immediately
    let aspect = Aspect::from_str(tokens[8])?;// return on error immediately
    Ok(GoAnnot::new(gene_product_id, symbol, relation, go_id, evidence, aspect))
}

#[command]
pub fn process_file(path: String) -> Result<String, String> {
    let file = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);
    
    let mut annotations = vec![];
    let mut annotation_stats: Vec<AnnotationStat> = vec![];
    let mut c = 0;
    for line in reader.lines() {
        c +=1;
        if c > 100000 {
            break;
        }
        match line {
            Ok(content) => {
                if content.starts_with("!") {
                    print!("{}", content);
                    if content.starts_with("!date-generated: ") {
                        let date_gen = &content[("!date-generated: ".len()+1)..];
                        annotation_stats.push(AnnotationStat::from_string("date generation", date_gen));
                    }
                } else {
                    let goann = process_annotation_line(&content);
                    match goann {
                        Ok(go_annotation) => annotations.push(go_annotation),
                        Err(e) => print!("ERROR: {}", e.to_string())
                    }
                }
            },
            Err(e) => return Err(format!("Error reading file: {}", e)),
        }
    }
    print!("Parsed {} annotations", annotations.len());
    let mut dataset = GO_ANNOTATIONS.lock().unwrap();
    *dataset = annotations.clone(); // Overwrite dataset
    let mut stats_map = annotation_descriptive_stats(&annotations);
    serde_json::to_string(&stats_map).map_err(|e| format!("Serialization error: {}", e))
}








#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_valid_evidence_codes() {
        let tests = vec![("EXP", EviCode::EXP),
                        ("IDA", EviCode::EXP),
                        ("IPI", EviCode::EXP),
                        ("IMP", EviCode::EXP),
                        ("IEP", EviCode::EXP),
                        ("HTP", EviCode::HTP),
                        ("HDA", EviCode::HTP),
                        ("HMP", EviCode::HTP),
                        ("HGI", EviCode::HTP),
                        ("HEP", EviCode::HTP),
                        ("IBA", EviCode::PHYLO),
                        ("IBD", EviCode::PHYLO),
                        ("IKR", EviCode::PHYLO),
                        ("IRD", EviCode::PHYLO),
                        ("ISS", EviCode::COMPUTATIONAL),
                        ("ISO", EviCode::COMPUTATIONAL),
                        ("ISA", EviCode::COMPUTATIONAL),
                        ("ISM", EviCode::COMPUTATIONAL),
                        ("ISS", EviCode::COMPUTATIONAL),
                        ("ISS", EviCode::COMPUTATIONAL),
                        ("TAS", EviCode::AUTHOR),
                        ("NAS", EviCode::AUTHOR),
                        ("IC", EviCode::IC),
                        ("ND", EviCode::ND),
                        ("IEA", EviCode::IEA), 
                        ];
        for test in tests {
            let ecode = EviCode::from_str(test.0);
            assert!(ecode.is_ok());
            assert_eq!(ecode.unwrap(), test.1);
        }
    
    }

    /// Make sure we get an error with an invalid evidence code
    #[test]
    fn test_invalid_evidence_code() {
        let ecode = EviCode::from_str("something");
        assert!(ecode.is_err());
        match ecode {
            Err(e) => {
                assert_eq!(e.to_string(), "Did not recognize 'something' as EvidenceCode.".to_string());
            },
            Ok(_) => panic!("Expected an error, but got Ok."),
        }
    }



}
