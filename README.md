# Mutatis Mutandis

> **Mutatis mutandis** is a Medieval Latin phrase meaning "_with things changed that should be changed_" or "_once the necessary changes have been made_".
> (Source : https://en.wikipedia.org/wiki/Mutatis_mutandis)

## Mutation testing

**Mutation testing** is a technique used to evaluate the quality of your unit tests. It involves introducing small changes (_mutations_) to your code, and then checking if your tests can detect these changes by failing. If a test passes despite a mutation, it indicates that the test suite might not be comprehensive enough.

### How it works:
1. **Original code**: You have a function or a set of functions covered by unit tests.
2. **Introduce mutation**: Small changes are made to the code (_e.g., flipping a boolean, changing an operator_).
3. **Run tests**: The unit tests are executed against the mutated code.
4. **Check results**:
   - If the tests fail, they "kill" the mutation, meaning the test suite is effective.
   - If the tests pass, the mutation "survives," indicating that the test suite might not cover that part of the code well.

### Example:

Here’s a simple code example in Rust:

#### Original function:
```rust
fn is_even(x: u32) -> bool {
    x % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(3), false);
    }
}
```

#### Mutated version:
A mutation could be flipping the boolean condition in `is_even`:

```rust
fn is_even(x: u32) -> bool {
    x % 2 != 0  // Changed from `==` to `!=`
}
```

If the tests fail, the mutation was successfully caught. If they pass, you have a "surviving mutation," indicating that the tests may not be robust enough.

### Benefits:
- Helps ensure that your tests are meaningful.
- Encourages stronger, more effective tests.

Mutation testing tools automate this process by applying mutations and running your test suite.

## Resources

**Mutation testing:**
- [Mutatis mutandis - Wikipedia](https://en.wikipedia.org/wiki/Mutatis_mutandis)
- [Mutatis mutandis — Wikipédia](https://fr.wikipedia.org/wiki/Mutatis_mutandis)
- [Mutation Testing - Software Testing - GeeksforGeeks](https://www.geeksforgeeks.org/software-testing-mutation-testing/)
- [Understanding Mutation Testing: A Comprehensive Guide - testRigor AI-Based Automated Testing Tool](https://testrigor.com/blog/understanding-mutation-testing-a-comprehensive-guide/)


**Testing tools:**
- [Increase Solana test speed 10x using Bankrun - YouTube](https://www.youtube.com/watch?v=rut9l6nPZls)


