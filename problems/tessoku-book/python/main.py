class Node:
    def __init__(self, value):
        self.value = value
        self.next = None
        
    def __str__ (self):
        return str(self.value)

node1 = Node(10)
print(node1)
node2 = Node(20)
print(node2)
node1.next = node2
node3 = Node(30)
node2.next = node3
node4 = Node(40)
node3.next = node4

def print_list(top):
    print("<", end='')
    node = top
    while node is not None:
        print(node, end=",")
        node = node.next
    print(">")

def get(top, n):
    node = top
    for i in range(n):
        node = node.next
    return node

print(get(node1, 3))