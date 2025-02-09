use ontolius::io::OntologyLoaderBuilder;
use ontolius::ontology::csr::MinimalCsrOntology;






pub fn load_go(path: &str) {
    let loader = OntologyLoaderBuilder::new()
                .obographs_parser()
                .build();
    let go: MinimalCsrOntology = loader.load_from_path(path)
                                .expect("Could not load ontology");

}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_load() {
        let path = "/Users/robin/data/go.json";
        //load_go(path);  NOT WORKING TODO

    }

}

