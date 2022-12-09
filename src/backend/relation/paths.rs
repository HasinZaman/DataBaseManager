use std::collections::{HashMap, LinkedList};

use lazy_static::lazy_static;
use petgraph::{Graph, adj::NodeIndex, visit::NodeIndexable, Incoming, Outgoing};
use regex::Regex;

use super::{Relation, view::View, table::Table};

pub type DependencyGraph = Graph<NodeIndex, NodeIndex>;

pub fn get_dependency_graph(relations: &Vec<Relation>) -> DependencyGraph {
    let name_to_index: HashMap<String, usize> = name_to_index_hashmap(relations);
    let mut index_to_NodeIndex: HashMap<usize, NodeIndex> = HashMap::new();

    let mut dependency_tree: DependencyGraph = Graph::new();

    let edges: Vec<(NodeIndex, NodeIndex)> = {
        let mut edges: Vec<(usize,usize)> = Vec::new();

        relations.iter()
        .enumerate()
        .for_each(|(index, r)| {
            let tmp = dependency_tree.add_node(index as u32).index();

            index_to_NodeIndex.insert(
                index, 
                tmp.try_into().unwrap()
            );
            match r {
                Relation::Table(table) => add_table_edges(table, &mut edges, index, &name_to_index),
                Relation::View(view) => add_view_edges(view, &name_to_index, &mut edges, index),
            }
        });

        edges.iter()
            .map(|(i1, i2)| {
                (
                    *index_to_NodeIndex.get(i1).unwrap(),
                    *index_to_NodeIndex.get(i2).unwrap(),
                )
            })
            .collect()
    };

    dependency_tree.extend_with_edges(&edges);

    dependency_tree
}

fn name_to_index_hashmap(relations: &Vec<Relation>) -> HashMap<String, usize> {
    let name_to_index: HashMap<String, usize> = {
        let mut name_to_index: HashMap<String, usize> = HashMap::new();

        relations.iter()
            .enumerate()
            .for_each(|(i, r)| {name_to_index.insert(r.name(), i);});
    
        name_to_index
    };
    name_to_index
}

fn add_view_edges(view: &View, name_to_index: &HashMap<String, usize>, edges: &mut Vec<(usize, usize)>, index: usize) {
    let query = &{
        let mut query = (&*view.query).clone();

        lazy_static!{
            static ref INNER_JOIN : Regex = Regex::new("[Ii][Nn][Nn][Ee][r] [Jj][Oo][Ii][Nn]").unwrap();
        };

        if INNER_JOIN.is_match(&query) {
            query = INNER_JOIN.replace_all(&query, ", ").to_string();
        }
        
        lazy_static!{
            static ref OUTER_JOIN : Regex = Regex::new("[Oo][Uu][Tt][Ee][Rr] [Jj][Oo][Ii][Nn]").unwrap();
        };

        if OUTER_JOIN.is_match(&query) {
            query = OUTER_JOIN.replace_all(&query, ", ").to_string();
        }
        
        lazy_static!{
            static ref JOIN : Regex = Regex::new("[Jj][Oo][Ii][Nn]").unwrap();
        };

        if JOIN.is_match(&query) {
            query = JOIN.replace_all(&query, ", ").to_string();
        }

        query
    };
    

    let dependencies = {
        lazy_static!{
            static ref DEPENDENCIES_REGEX : Regex = Regex::new("[Ff][Rr][Oo][Mm](?:[\\n\\t]+| )([a-zA-Z][a-zA-Z0-9_]+(?:, ?[a-zA-Z][a-zA-Z0-9_]+)*)").unwrap();
        };
        
        if !DEPENDENCIES_REGEX.is_match(query) {
            return ;
        }

        DEPENDENCIES_REGEX.captures(query).unwrap()
    };

    dependencies.iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if i % 2 == 1 {
                return Some(c)
            }
            return None
        })
        .filter(|capture| match capture {
            Some(_) => true,
            None => false,
        })
        .map(|capture| capture.unwrap().as_str())
        .for_each(|capture| {
            
            lazy_static!{
                static ref DEPENDENCIES_REGEX : Regex = Regex::new("([a-zA-Z0-9_]+)[^,]").unwrap();
            };

            DEPENDENCIES_REGEX.captures_iter(capture)
                .for_each(|capture| {
                    let capture = capture.get(0).unwrap().as_str();

                    if let Some(val) = name_to_index.get(capture) {
                        edges.push(
                            (
                                *val,
                                index
                            )
                        );
                    }
                });
        });
}

fn add_table_edges(table: &Table, edges: &mut Vec<(usize, usize)>, index: usize, name_to_index: &HashMap<String, usize>) {
    if let Some(foreign_keys) = table.get_foreign_keys() {
        foreign_keys.iter()
            .for_each(
                |(table_name, _attribute_name)| {
                    edges.push(
                        (
                            *name_to_index.get(table_name).unwrap(),
                            index
                        )
                    );
                }
            );
    }
}

pub fn get_generation_path(relations: &Vec<Relation>, dependency_tree: &DependencyGraph) -> Vec<usize> {
    let mut visited = vec![false; relations.len()];

    let mut order = LinkedList::new();

    'node_loop: for i in 0..relations.len() {
        if visited[i] {
            continue 'node_loop;
        }
        println!("explore:{i}");

        if let Some(mut path) = add_dependency(relations, &dependency_tree, &mut visited, i) {
            order.append(&mut path);
        }
    };

    let mut result: Vec<usize> = Vec::with_capacity(order.len());

    for i in order{
        result.push(i);
    }

    result
}

fn add_dependency(relations: &Vec<Relation>, dependency_tree: &DependencyGraph, visited: &mut Vec<bool>, node: usize) -> Option<LinkedList<usize>> {
    if visited[node] {
        return None;
    }
    
    let mut order : LinkedList<usize> = LinkedList::new();
    visited[node] = true;


    let node_index = dependency_tree.from_index(node);


    dependency_tree.neighbors_directed(node_index, Incoming)
        .for_each(
            |pre_node| {
                let dependencies = add_dependency(
                    relations,
                    dependency_tree,
                    visited,
                    *dependency_tree.node_weight(pre_node).unwrap() as usize
                );
                if let Some(mut dependencies) = dependencies {
                    order.append(
                        &mut dependencies
                    );
                }
            }
        );
    
    order.push_back(node);

    dependency_tree.neighbors_directed(node_index, Outgoing)
        .for_each(
            |post_node| {
                let dependents = add_dependency(
                    relations,
                    dependency_tree,
                    visited,
                    *dependency_tree.node_weight(post_node).unwrap() as usize
                );
                if let Some(mut dependents) = dependents {
                    order.append(
                        &mut dependents
                    );
                }
            }
        );
    
    
    return Some(order);
}

mod tests{
    use std::{collections::{HashSet, HashMap}, f32::consts::E};

    use petgraph::{Graph, adj::NodeIndex, visit::NodeIndexable, Incoming, Outgoing};

    use crate::backend::{
        relation::{
            Relation,
            table::{Table, Attribute, AttributeType, Constraint},
            paths::{get_dependency_graph, DependencyGraph},
            view::View
        },
        sql::SQL
    };

    use super::get_generation_path;

    //assert_eq_graph checks if two graphs are the same
    macro_rules! assert_eq_graph {
        ($actual: ident, $expected: ident, $relations: ident) => {
            println!("{:?}", $relations);

            assert_eq!($actual.node_count(), $expected.node_count());
            assert_eq!($actual.edge_count(), $expected.edge_count());

            //check if each node have the same incoming & outgoing edges
            for i in 0..$relations.len() {
                println!("node:{}", i);

                {
                    let actual = $actual.neighbors_directed(
                        $actual.from_index(i),
                        Incoming
                    );
                    let expected = $expected.neighbors_directed(
                        $expected.from_index(i),
                        Incoming
                    );

                    assert_eq!(actual.size_hint(), expected.size_hint());
        
                    actual.zip(expected)
                        .for_each(|(actual,expected)| assert_eq!(actual, expected));
                }

                {
                    let actual = $actual.neighbors_directed(
                        $actual.from_index(i),
                        Outgoing
                    );
                    let expected = $expected.neighbors_directed(
                        $expected.from_index(i),
                        Outgoing
                    );

                    assert_eq!(actual.size_hint(), expected.size_hint());
        
                    actual.zip(expected)
                        .for_each(|(actual,expected)| assert_eq!(actual, expected));
                }
            }
        }
    }
    macro_rules! foreign_relation {
        [] => {
            Attribute{
                name: String::from("attr_1"),
                data_type: AttributeType::Text(10),
                constraint: HashSet::new()
            }
        };
        [$table_name:literal] => {
            Attribute{
                name: String::from("attr_2"),
                data_type: AttributeType::Text(10),
                constraint: HashSet::from(
                    [
                        Constraint::ForeignKey{
                            table_name: String::from($table_name),
                            attribute_name: String::from("attr_1")
                        }
                    ]
                )
            }
            
        };
    }
    #[test]
    fn dependency_test_1() {
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_2"),
                    attributes: vec![
                        foreign_relation!["table_1"]
                    ],
                    primary_key: None,
                }
            ),
        ];

        let actual = get_dependency_graph(&relations);

        let mut expected: DependencyGraph = Graph::new();
        let v1 = expected.add_node(0);
        let v2 = expected.add_node(1);

        expected.extend_with_edges(&[(v1,v2)]);

        assert_eq_graph!(actual, expected, relations);
    }

    #[test]
    fn dependency_test_2() {
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_2"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: None,
                }
            ),
        ];

        let actual = get_dependency_graph(&relations);
        
        let mut expected: DependencyGraph = Graph::new();
        let v1 = expected.add_node(0);
        let v2 = expected.add_node(1);

        assert_eq_graph!(actual, expected, relations);
    }
    
    #[test]
    fn dependency_test_3() {
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_2"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(1),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_3"),
                    attributes: vec![
                        foreign_relation!["table_1"],
                        foreign_relation!["table_2"]
                    ],
                    primary_key: None,
                }
            ),
        ];

        let actual = get_dependency_graph(&relations);

        let mut expected: DependencyGraph = Graph::new();
        let v1 = expected.add_node(0);
        let v2 = expected.add_node(1);
        let v3 = expected.add_node(2);

        expected.extend_with_edges(&[(v1, v3), (v2, v3)]);

        assert_eq_graph!(actual, expected, relations);
    }

    #[test]
    fn dependency_test_4() {
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::View(
                View{
                    name: String::from("view_1"),
                    query: SQL::from("SELECT * FROM table_1").unwrap().qdl().unwrap().clone()
                }
            )
        ];

        let actual = get_dependency_graph(&relations);

        let mut expected: DependencyGraph = Graph::new();
        let v1 = expected.add_node(0);
        let v2 = expected.add_node(1);

        expected.extend_with_edges(&[(v1, v2)]);

        assert_eq_graph!(actual, expected, relations);
    }
 
    #[test]
    fn dependency_test_5() {
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_2"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: None,
                }
            ),
            Relation::View(
                View{
                    name: String::from("view_1"),
                    query: SQL::from("SELECT * FROM table_1, table_2").unwrap().qdl().unwrap().clone()
                }
            )

        ];

        let actual = get_dependency_graph(&relations);

        let mut expected: DependencyGraph = Graph::new();
        let v1 = expected.add_node(0);
        let v2 = expected.add_node(1);
        let v3 = expected.add_node(2);

        expected.extend_with_edges(&[(v1, v3), (v2, v3)]);

        assert_eq_graph!(actual, expected, relations);
    }
    
    #[test]
    fn dependency_test_6() {
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_2"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_3"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::View(
                View{
                    name: String::from("view_1"),
                    query: SQL::from("SELECT * FROM table_1, table_2, table_3").unwrap().qdl().unwrap().clone()
                }
            )

        ];

        let actual = get_dependency_graph(&relations);

        let mut expected: DependencyGraph = Graph::new();
        let v1 = expected.add_node(0);
        let v2 = expected.add_node(1);
        let v3 = expected.add_node(2);
        let v4 = expected.add_node(3);

        expected.extend_with_edges(&[(v1, v4), (v2, v4), (v3, v4)]);

        assert_eq_graph!(actual, expected, relations);
    }

    //assert_path checks the validity of a path by brute force checking every relation. Making sure that all dependency relations are in front of a given relation that being checked
    macro_rules! assert_path {
        ($relations: ident, $dependency_tree: ident, $actual: ident) => {
            println!("{:?}", $actual);
            println!("{:?}", $dependency_tree);
            for i in 0..$relations.len() {

                let index = $actual.iter()
                    .enumerate()
                    .filter(|(_, val)| **val == i)
                    .map(|(index, _)| index)
                    .next()
                    .unwrap();

                let splice = &$actual[0..index];
                println!("check: {:?}\t index:{:?}", i, index);
                println!("splice: {:?}", splice);
                
                let node_index = $dependency_tree.from_index(i);
                let dependencies: Vec<usize> = $dependency_tree.neighbors_directed(node_index, Incoming)
                    .map(|index| *$dependency_tree.node_weight(index).unwrap() as usize)
                    .collect();

                println!("dependencies: {:?}", dependencies);
    
                if splice.len() == 0 {
                    assert!(dependencies.len() == 0)
                }
                else {
                    let mut visited_cond: HashMap<usize, bool> = HashMap::new();
                    
                    for i in 0..dependencies.len(){
                        visited_cond.insert(dependencies[i], false);
                    }
    
                    for relation_index in splice.iter().rev() {
                        if dependencies.contains(relation_index) {
                            visited_cond.insert(*relation_index, true);
                        }
                    }
    
                    assert!(visited_cond.values().all(|val| *val == true));
                }
            }
        }
    }

    #[test]
    fn generation_path_test_1(){
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
        ];

        let dependency_tree = get_dependency_graph(&relations);

        let actual = get_generation_path(&relations, &dependency_tree);

        assert_path!(relations, dependency_tree, actual);
    }
    
    #[test]
    fn generation_path_test_2(){
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_2"),
                    attributes: vec![
                        foreign_relation!["table_1"]
                    ],
                    primary_key: Some(0),
                }
            ),
        ];

        let dependency_tree = get_dependency_graph(&relations);

        let actual = get_generation_path(&relations, &dependency_tree);

        assert_path!(relations, dependency_tree, actual);
    }
    
    #[test]
    fn generation_path_test_3(){
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation!["table_2"]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_2"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
        ];

        let dependency_tree = get_dependency_graph(&relations);

        let actual = get_generation_path(&relations, &dependency_tree);

        assert_path!(relations, dependency_tree, actual);
    }
    
    #[test]
    fn generation_path_test_4(){
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_2"),
                    attributes: vec![
                        foreign_relation!["table_1"]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_3"),
                    attributes: vec![
                        foreign_relation!["table_2"]
                    ],
                    primary_key: Some(0),
                }
            ),
        ];

        let dependency_tree = get_dependency_graph(&relations);

        let actual = get_generation_path(&relations, &dependency_tree);

        assert_path!(relations, dependency_tree, actual);
    }

    #[test]
    fn generation_path_test_5(){
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_2"),
                    attributes: vec![
                        foreign_relation!["table_1"]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_3"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
        ];

        let dependency_tree = get_dependency_graph(&relations);

        let actual = get_generation_path(&relations, &dependency_tree);

        assert_path!(relations, dependency_tree, actual);
    }

    #[test]
    fn generation_path_test_6(){
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation!["table_3"]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_2"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_3"),
                    attributes: vec![
                        foreign_relation!["table_2"]
                    ],
                    primary_key: Some(0),
                }
            ),
        ];

        let dependency_tree = get_dependency_graph(&relations);

        let actual = get_generation_path(&relations, &dependency_tree);

        assert_path!(relations, dependency_tree, actual);
    }

    #[test]
    fn generation_path_test_7(){
        let relations: Vec<Relation> = vec![
            Relation::Table(
                Table{
                    name: String::from("table_1"),
                    attributes: vec![
                        foreign_relation![]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_2"),
                    attributes: vec![
                        foreign_relation!["table_1"]
                    ],
                    primary_key: Some(0),
                }
            ),//0,1,2
            Relation::Table(
                Table{
                    name: String::from("table_3"),
                    attributes: vec![
                        foreign_relation!["table_2"]
                    ],
                    primary_key: Some(0),
                }
            ),
            Relation::Table(
                Table{
                    name: String::from("table_4"),
                    attributes: vec![
                        foreign_relation!["table_2"]
                    ],
                    primary_key: Some(0),
                }
            ),
        ];

        let dependency_tree = get_dependency_graph(&relations);

        let actual = get_generation_path(&relations, &dependency_tree);

        assert_path!(relations, dependency_tree, actual);
    }

}