use std::collections::{HashMap, HashSet};

pub fn can_finish(num_courses: i32, prerequisites:Vec<Vec<i32>>) -> bool {
    // create vertices
    let mut graph = HashMap::new();
    for vertex in 0..num_courses {
        graph.insert(vertex, vec![]);
    }
    for edge in prerequisites {
        if edge.len() != 2 {
            return false
        }
        if let Some(course) = graph.get_mut(&edge[1]) {
            course.push(edge[0]);
        }
    }

    //DFS for cycles
    let mut checked = HashSet::new();
    let mut visited = HashSet::new();
    
    for (course, _) in &graph {
        if let Some(_) = checked.get(course) {
            continue;
        }
        if !dfs(&graph, *course, &mut checked, &mut visited) {
            return false;
        }
    }

    true
}

fn dfs(graph: &HashMap<i32, Vec<i32>>, start: i32, checked: &mut HashSet<i32>, visited: &mut HashSet<i32>) -> bool {
    if visited.contains(&start) {
        return false;
    }
    if checked.contains(&start) {
        return true;
    }

    visited.insert(start);

    if let Some(courses) = graph.get(&start) {
        for course in courses {
            if !dfs(graph, *course, checked, visited) {
                return false;
            
            }
        }
    }
    
    visited.remove(&start);
    checked.insert(start);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_finish() {
        assert!(can_finish(2, vec![vec![1,0]]));
        assert!(!can_finish(2, vec![vec![1,0], vec![0,1]]));
    }
}