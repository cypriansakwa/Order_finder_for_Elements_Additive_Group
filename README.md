This code effectively finds the order of each element in the additive group $\mathbb{Z}_n$ for the given modulus $n$.
The code finds the smallest integer $k$ such that $k\boldsymbol{\cdot}a\bmod n=0$ for each $a$ in the range $0,\cdots,n-1$
## ## Features
- Uses the `num-bigint` crate for handling large integers.
- Computes the order of all elements in \( \mathbb{Z}_n \) for a given \( n \).
- Prints the results in a clear format.

  ## Installation

- To use this project, you need to have Rust installed on your machine. 

- After installing Rust, clone this repository and navigate to the project directory:

```bash
git clone https://github.com/cypriansakwa/Order_finder_for_Elements_Additive_Group.git
cd cypriansakwa/Order_finder_for_Elements_Additive_Group



