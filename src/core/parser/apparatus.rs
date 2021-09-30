use colored::*;
use pest::iterators::Pair;
use std::collections::HashMap;

//use crate::util::print_statement;
use crate::core::parser::{ statement, Rule };
use crate::core::ast::{Apparatus, Statement};


#[allow(dead_code)]
impl Apparatus {
    // pub fn print_profile(&self) {
    //     println!();
    //     println!("{}:\t{}", "MODULE".yellow().bold(), self.name);
    //
    //     let entry = self.entry();
    //     if let Some(_entry) = entry {
    //         println!("{}:\tmain", "ENTRY".yellow().bold());
    //     }
    //
    //     let imports = self.imports();
    //     if let Some(imports) = imports {
    //         if imports.len() > 0 {
    //             println!("{}", "IMPORTS".yellow().bold());
    //             for import in imports {
    //                 print_statement(import);
    //             }
    //         }
    //     }
    //
    //     let functions = self.functions();
    //     if let Some(functions) = functions {
    //         if functions.len() > 0 {
    //             println!("{}:", "FUNCTIONS".yellow().bold());
    //             for function in functions {
    //                 print_statement(function);
    //             }
    //         }
    //     }
    //
    //     println!();
    // }

    fn entry(&self) -> Option<&Statement> {
        self.environment.get("main")
    }

    fn imports(&self) -> Option<Vec<&Statement>> {
        self.environment.keys().filter(|statement| {
            match self.environment
                .get(*statement)
                .expect("Something went wrong") {

                Statement::Import { .. } => return true,
                _ => return false
            }
        }).map(|key| {
            return self.environment.get(key);
        }).collect()
    }

    // Note that this function does not return the `main` function. This is
    // because `main` serves as the entry point to the apparatus and is never meant
    // to be accessed outside the apparatus.
    fn functions(&self) -> Option<Vec<&Statement>> {
        self.environment.keys().filter(|statement| {
            match self.environment
                .get(*statement)
                .expect("Something went wrong") {

                Statement::Function { name, .. } => {
                    return *name != String::from("main");
                },
                _ => return false
            }
        }).map(|key| {
            return self.environment.get(key);
        }).collect()
    }

    fn types(&self) -> Option<Vec<&Statement>> {
        self.environment.keys().filter(|statement| {
            match self.environment
                .get(*statement)
                .expect("Something went wrong") {

                Statement::Class { .. } => return true,
                _ => return false
            }
        }).map(|key| {
            return self.environment.get(key);
        }).collect()
    }
}

pub fn parse(name: &str, apparatus: Pair<Rule>) -> Apparatus {
    let mut environment: HashMap<String, Statement> = HashMap::new();
    for node in apparatus.into_inner() {
        match node.as_rule() {
            Rule::statement => {
                let s: Statement = statement::parse(node);
                match s.clone() {
                    Statement::Import { name, .. } => {
                        environment.insert(name, s);
                    },
                    Statement::Function { name, .. } => {
                        environment.insert(name, s);
                    },
                    Statement::Class { name, .. } => {
                        environment.insert(name, s);
                    }
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }

    Apparatus {
        name: String::from(name),
        environment
    }
}
