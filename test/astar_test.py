import pypathfinding

# Recreating the knight's position function from the example
class Pos:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def distance(self, other):
        return abs(self.x - other.x) + abs(self.y - other.y)

    def successors(self):
        moves = [(1, 2), (1, -2), (-1, 2), (-1, -2),
                 (2, 1), (2, -1), (-2, 1), (-2, -1)]
        # Convert Pos instances to tuples
        return [((self.x + dx, self.y + dy), 1) for dx, dy in moves]

# Lots of converting...
def py_astar(start_pos, goal_pos):
    start = (start_pos.x, start_pos.y)
    goal = (goal_pos.x, goal_pos.y)

    result = pypathfinding.py_astar(
        start=start,
        successors=lambda pos: [
            ((x, y), cost) for ((x, y), cost) in Pos(pos[0], pos[1]).successors()
        ],
        heuristic=lambda pos: Pos(pos[0], pos[1]).distance(goal_pos) // 3,
        success=lambda pos: pos == goal
    )

    if result:
        path, cost = result
        # Convert tuples back to Pos instances for Python
        path = [Pos(x, y) for (x, y) in path]
        return path, cost
    else:
        return None

start_pos = Pos(1, 1)
goal_pos = Pos(4, 6)
result = py_astar(start_pos, goal_pos)

if result:
    path, cost = result
    print(f"Path found with cost {cost}: {[f'({pos.x}, {pos.y})' for pos in path]}")
else:
    print("No path found")
