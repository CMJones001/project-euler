#!/usr/bin/env python

from sympy import *
import numpy as np

n, m = symbols("n m", integer=True, positive=True)

def pentagonal(a):
    return (3*a**2 - a)/2

values = np.arange(10) + 1
pentagonal_nums = pentagonal(values)

def pretty_format(val) -> str:
    return str(val).replace("**", "^").replace("*", " ")

print("n+1")
res = expand(pentagonal(n+1))
print(pretty_format(res))

print("2n")
res = simplify(pentagonal(n) + pentagonal(n))
print(pretty_format(res))

print("P(n+1) + P(n)")
res = simplify(pentagonal(n+1) - pentagonal(n))
print(pretty_format(res))
