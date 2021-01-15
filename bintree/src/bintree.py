class Node:
  def __init__(self, value):
    self.value = value
    self.left = None
    self.right = None

class Tree:
  def __init__(self, initial_value):
    self.root = Node(initial_value)

  def insert(self, root, item):
    if item < root.value:
      if root.left == None: 
        root.left = Node(item)
      else:
        self.insert(root.left, item)
    elif item > root.value:
      if root.right == None: 
        root.right = Node(item)
      else:
        self.insert(root.right, item)

  def traverse(self, root):
    if root.left:
      self.traverse(root.left)
    print(root.value)
    if root.right:
        self.traverse(root.right)


if __name__ == "__main__":
    field = [4,7,2,12,3,1,100]
    tree = Tree(5)
    for item in field:
      tree.insert(tree.root, item)

    tree.traverse(tree.root)

