This code effectively finds the order of each element in the additive group $\mathbb{Z}_n$ for the given modulus $n$.
The code finds the smallest integer $k$ such that $k\boldsymbol{\cdot}a\bmod n=0$ for each $a$ in the range $0,\cdots,n-1$

- num_bigint::BigUint: This library supports arbitrary-precision arithmetic on unsigned integers. It's utilized here because BigUint can handle extremely large numbers, which is useful in cryptography and mathematical applications.
- num_traits::{One, Zero}: These traits provide methods to create 1 and 0 values for BigUint.

git clone https://github.com/cypriansakwa/Order_of_Element_Additive_Group.git
cd cypriansakwa/Order_of_Element_Additive_Group
