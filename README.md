# Directed Acyclic Graph Cycle Check Utility

run `cargo r && echo $?` to get return code
a test should say 'if given invalid input then output this code'

... vs adjacency list? an adjacency list would require storing multiple references
it stores multiple "to" references, in a hashmap. the alternative would be a more 'raw' layout?
would this require storing everything on heap? what would be difficult about doing this on stack?
layout, I think.

adjacency can be in or out

Next: write down the way this will be done before writing code
Decide what structure will be used
Decide how the iteration will happen (each step)
Decide what intermediate variables are needed
Everything is an intermediate state that the computation uses. Everything is memory.
What I am doing is asserting a particular quality of the structure of that memory; it must be comprehensible.
What I have done so far is assert the input format, placing edges (pairs of character nodes) into a vector.
This will allow for easier assertion. Nodes also implement equality.
Adjacent edges = common vertex

nodes w/indegree - nodes with
Really, a _drected_ adjacency list where the list is all nodes that are _direct decendants_ rather than neighbors
To find root nodes:

1. A root has on or more out degrees and no in-degrees
2. For every node with at least one out-degree, check if there are any in-degrees
   Point #2 Start by obtaining all keys of the adjacency list; conviniently a unique list of nodes with at least one out-degree.
   Then find if there are any in-degrees for those nodes by looking through the edges; if those nodes are ever listed in "in_to"

For every node that has at least one out-degree, check in every edge to see if the node is ever listed as a decendant (or 'in to')
if this is a directed graph without cycles then, there should be at least one node fitting these qualifications

another way is to do a reverse adjacency list? not reverse, but listing the direct ascendants rather than descendants. That structure may not be so useful right now.

let nodes_with_out_degrees: Vec<&Node> = adjacency_list.keys().collect();

let root_nodes: Vec<&Node> = Vec::new();

Filter out nodes without degrees (that have an in degree) to be left with nodes without in degree
if any of the edges has this node as it's 'into' then don't include it

Is C->C a cycle?
// DFS(v):
// if finished(v)
// return
// if visited(v)
// "Cycle found" and return
// visited(v) = true
// for every neighbour w
// DFS(w)
// finished(v) = true

    // finished is represented by the decendants list being empty (iterator done)
    // visited is represented by it not being in the iter
    // add a

    // the stack will be an unknown length

    // these all have values that they are "on" - and are associated with a node
    // [iter(0), iter(1), iter(7)]
    // [iter(0), iter(0), iter(0)] ...to start out with.
    // to find if there is a cycle, then the exact values of the current iteration need to be known.
    // that could be a set of the same length that is added to every time an iteration is added
    // Drain these when done on each iteration
    // current state of the iteration


    path = []

    add first item of adj-list to path...

    loop while path has contents:
        
        (node, index) = path.last()

        if next = g.get(node)[index]: 

            index++

            if g.get(next)
                path.push((next, 0))

        else:
            path.pop()




        .. check if incrementing index is possible (there is a next item)

        .. if there is no decendant of next node AND the next index is valid:



        decendants = adj_list.get(node)

        if
        next = .list[index]
        if the adj-list has
        if last node of path has decendants:

        else:



---

Please write a command line utility that takes in a graph from standard input (in the format described below) and looks for cycles in that graph.

The utility should output (to standard output) an arbitrary cycle (if one exists, nothing otherwise).

The utility’s exit code should be
0 if no cycle was found,
1 if a cycle was found,
2 if the input is invalid.

The input will be given as a series of edges with tab delimited lines (each line separated by a single newline \n).

An edge should have exactly two fields, each referring to a node’s unique identifier, interpreted as a directed from the first node to the second.

Node identifiers may contain any character besides tabs and newlines, but they should have at least a single character.

If a cycle is found, the output should be given as a series of node identifiers separated by newlines (the first node should not be repeated).

If there are multiple cycles, only one should be returned (the choice of which may be arbitrary). For example, if the given graph has a cycle from A to B to A, the output could be A\nB\n.
