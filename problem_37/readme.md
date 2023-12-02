# Problem 37: Truncatable primes

The number 37 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.

Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

NOTE:
2, 3, 5, and 7 are not considered to be truncatable primes.

## Notes

In most positions we cannot have an even digit or a 5; as these digits as a final digit would make the number composite.  

The only exception is the first digit, which can be 2, 3, 5, or 7.
We can also not have a 0 as a first digit.

The sum of the digits also cannot be divisible by 3, as this would make the number composite.

## Number assembly

If we start from the digits position, we can use only the digits 1, 3, 7, and 9.

If the digit is 3, or 9 then we can only use the digits 1 or 7 for the next digit, as this would cause the sum of digits 
to be divisible by 3. This applies to all digits except the left-most digit. 

In the right-most position we can only have the primes 3, 7. 5 and 2 cannot be in this position as these cannot be the 
right-most digit in multi-digit primes. 

# Conclusion

This careful generation of the candidate lists means that we only have to check primality of ~280 candidates, to 
include the all the values up to 1e7!

Hard coding the primes could be faster, but feels like cheating.