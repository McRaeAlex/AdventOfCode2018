import sys

class Node:
    def __init__ (self, val):
        self.parents = []
        self.children = []
        self.value = val
        self.lookedAt = False

    def __cmp__(self, other):
        return cmp(self.value, other.value)

    def __repr__(self):
        return ("%s %s" % (self.value, self.children))

    def isValid(self):
        self.parents.sort(key=lambda x: x.value)
        for node in self.parents:
            if not node.lookedAt:
                return False
        return True

    def getCause(self):
        self.parents.sort(key=lambda x: x.value)
        for node in self.parents:
            if not node.lookedAt:
                return node


lines = sys.stdin.readlines()
lines = open('/home/alex/Coding/AdventOfCode2018/day7/input.txt', 'r').readlines()

# parse the input
nodes = {}
for line in lines:
    gt = line[5]
    lt = line[36]
    print(gt, lt)
    if not gt in nodes:
        nodes[gt] = Node(gt)
    if not lt in nodes:
        nodes[lt] = Node(lt)
    nodes[gt].children.append(nodes[lt])
    nodes[lt].parents.append(nodes[gt])

# find the root
start = nodes['A']
while(len(start.parents) != 0):
    start = start.parents[0]

# go through the graph
def DisplayGraph(node):
    if not node.isValid():
        return
    print(node.value, end='')
    node.lookedAt = True
    node.children.sort(key=lambda x: x.value)
    for nod in node.children:
        DisplayGraph(nod)

print(start)
DisplayGraph(start)
print('')