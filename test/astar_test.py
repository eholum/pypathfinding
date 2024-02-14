import unittest
from pypathfinding import Graph, Pose, py_astar

class TestAStarAlgorithm(unittest.TestCase):
    def test_astar_pathfinding_simple(self):
        # Create a graph instance
        g = Graph()

        # Define start and goal poses
        start = Pose(0, 0)
        goal = Pose(1, 0)

        # Add nodes to the graph
        g.add_node(start)
        g.add_node(goal)
        g.add_edge(start, goal)

        # Call the py_astar function
        result = py_astar(start, goal, g)

        # Check that a path is found
        self.assertIsNotNone(result, "A* did not find a path.")

        # Unpack the result
        path, cost = result
        path = [p.get_coordinates() for p in path]

        # Check the path length and cost
        self.assertEqual(path, [(0, 0), (1, 0)])
        self.assertEqual(len(path), 2)
        self.assertAlmostEqual(cost, 1.0, delta=0.01)


    def test_astar_pathfinding_harder(self):
         # Create a graph instance
        g = Graph()

        # Define start and goal poses
        start = Pose(0, 0)
        goal = Pose(2, 3)
        g.add_node(start)
        g.add_node(goal)

        poses = [Pose(i, j) for i in range(3) for j in range(3)]
        for p in poses:
            g.add_node(p)
        
        g.add_edge(start, poses[1])
        g.add_edge(start, poses[3])    
        g.add_edge(poses[1], poses[2])
        g.add_edge(poses[1], poses[4])
        g.add_edge(poses[2], poses[5])
        g.add_edge(poses[3], poses[4])
        g.add_edge(poses[3], poses[6])
        g.add_edge(poses[5], poses[8])
        g.add_edge(poses[6], poses[7])
        g.add_edge(poses[8], goal)

        # Call the py_astar function
        result = py_astar(start, goal, g)

        # Check that a path is found
        self.assertIsNotNone(result, "A* did not find a path.")

        # Unpack the result
        path, cost = result
        path = [p.get_coordinates() for p in path]

        # Check the path length and cost
        self.assertEqual(path, [(0.0, 0.0), (0.0, 1.0), (0.0, 2.0), (1.0, 2.0), (2.0, 2.0), (2.0, 3.0)])
        self.assertEqual(len(path), 6)
        self.assertAlmostEqual(cost, 5.0, delta=0.01)


if __name__ == '__main__':
    unittest.main()
