import pytest
from pypathfinding import Graph, Pose, py_astar, py_dijkstra

def build_graph(n: int):
    """Return an nxn fully connected graph"""
    ps = [Pose(i, j) for i in range(n) for j in range(n)]
    g = Graph()

    # Connect neighbors
    for p in ps[:-1]:
        p1 = Pose(p.x(), p.y() + 1)
        p2 = Pose(p.x() + 1, p.y() + 1)
        g.add_edge(p, p1)
        g.add_edge(p, p2)

    return g

def test_corner_to_corner_a_star(benchmark):
    n = 100
    g = build_graph(n + 1)
    start = Pose(0, 0)
    goal = Pose(n, n)

    @benchmark
    def bench():
        result = py_astar(start, goal, g)
        assert result is not None, "A* did not find a path."

def test_corner_to_corner_dijkstra(benchmark):
    n = 100
    g = build_graph(n + 1)
    start = Pose(0, 0)
    goal = Pose(n, n)

    @benchmark
    def bench():
        result = py_dijkstra(start, goal, g)
        assert result is not None, "A* did not find a path."
