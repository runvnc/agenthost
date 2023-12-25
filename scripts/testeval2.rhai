fn tree(n) {
  let outs = 0;
  for j in 1..n {
    for i in 1..j {
      outs += "*";
    }
    outs += "\n";
  }
  outs
};

print(tree(10))
