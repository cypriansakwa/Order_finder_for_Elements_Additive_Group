This code effectively finds the order of each element in the additive group $\mathbb{Z}_n$ for the given modulus $n$.
The code finds the smallest integer $k$ such that $k\boldsymbol{\cdot}a\bmod n=0$ for each $a$ in the range $0,\cdots,n-1$
## Features
- Uses the `num-bigint` crate for handling large integers.
- Computes the order of all elements in \( \mathbb{Z}_n \) for a given \( n \).
- Prints the results in a clear format.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository and navigate to the project directory:
## Usage
- Modify the value of $n$ in the main function to represent the size of the additive group $\mathbb{Z}_n $.
- Run the program using the following command: cargo run
- The program will print the order of each element in $\mathbb{Z}_n $.
  ## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

```bash
git clone https://github.com/cypriansakwa/Order_finder_for_Elements_Additive_Group.git
cd cypriansakwa/Order_finder_for_Elements_Additive_Group



