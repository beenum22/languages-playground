# Data Structures with Rust

This is a practice arena for all the learnings from the Data Structures Udemy course, ["Learn, Analyse and Implement Data Structure using C and C++."](https://www.udemy.com/course/datastructurescncpp/?couponCode=ST18MT62524). In parallel, I am also learning to code in Rust and therefore, I decided to implement all the foudational Data Structures from scratch in Rust.

# Udemy Notes

## Introduction
The memory segment is divided into three sections; Stack, Heap and Code Section.
Machine bytecode for our program is stored in the Code section of the memory.

The declared variables in the code for example constant integers, etc are stored in the Stack Frame section of the Stack and they are called Activation Records. There's separate Activation Record for each function in the code. A Stack Frames/Activation Records are stacked over one another following the FILO concept. Due to this behavior, this section of the memory is called a Stack.

This is also called static memory allocation because the memory required by the variables is known/decided at the compile time.

Heap in general can refer to both organized and unorganized items. However in terms of memory, it only refers to unorganized data. Heap should be treated as a resource that should only be requested or used if needed by the application/program.

Programs cannot directly access the Heap memory. It can only access it using the Pointers. The size of the Pointer depends on the size of the integer stored but generally it can be 2 Bytes. Heap memory is explicitly allocated and deallocated by the Program. In case the memory is not deallocated, it's called a memory leak.

Examples:
Arrays are stored in Heap memory and the respective Pointer is stored in Stack memory.

### Data structures Types
Data Structures are divided into Physical and Logical Data Structures.

#### Physical Data Structure
These are called Physical Data structures because they define how the memory should be organized for storing the elements/data. These are meant to actually store the data in memory.

1. Array: The size of an Array is static. It can be created either in Stack or Heap. All the elements are stored together in the memory.

2. Linked List: It is a collected of nodes. The size of a Linked List can change dynamically. It is always created in a Heap.

Arrays are used when the number of elements is static or fixed and Linked Lists are used when the number of elements is not fixed.

#### Logical Data Structure

The mechanisms or ways to perform different operations on data stored by Physical Data Structures are called Logical Data Structures.

1. Stack (Linear DS) (FILO)
2. Queues (Linear DS) (FIFO)
3. Trees (Non-linear DS) (Hierarchy)
4. Graph (Non-linear DS) (Collection of Nodes)
5. Hash Table (Both linear and non-linear DS)

To implement these Logical Data Structures, we have to use the Physical Data Structures.

### Abstract Datatype (ADT)

A datatype defines how the data is stored and which operations can be performed on it.

#### Example
int x. Assuming we need 2 Bytes for an integer.

| | | | | | | | | | | | | | | | |   <-- 16 bits Data representation in Memory
1 bit        15 bits
Sign       for data

Integer datatype supports arithmetic operations.

Abstract in here means that much of the information is hidden from us such as how the operations are performed and how the data is stored. However, the integer datatype shown above is not Abstract but rather Primitive datatype. The concept of Abstract comes from OOP where we can define our own classes with custom datatypes whose internal details are hidden.

### Time Complexity

Computers are primarily used for computational tasks and we want to know how time would it take to solve a problem/computation by using certain method or data structures. The time complexity is defined as Order of n/O(n)/Big O notation where n is the number of elements.

We have an example list with `n` number of elements.

Array repesentation --> O(n)

We can also calculate the BigO by looking at the code for example if there is a for loop over n elements, it means O(n).

When we divide something ( a list, etc) successfully until it reaches 1, that process is represented as Log. So for example we have a list which process by constantly dividing it by half and then processing, we can represent the Big O by:

log2 n  (2 because we were dividing by half).

### Space Complexity

It just specifies how much space is required in memory. It is not concerned with the number of Bytes or data types.

For example, a list of n elements will have Space Complexity of n.

Space Complexity is also represented with Big O notation.

## Recursion
If a function is calling itself then it's called a recursive function. Recursion has two phase; calling and returning. These phases are also called Ascending and Descending.

Recursion is also repeating just like a Loop. However, the difference between the two is that Loop will have only Ascending phase while Recursion has both.

Example:
```
void func (int n) {
    if (n > 0) {
        printf("Yo");      # Ascending/Calling
        func(n-1);
        printf("Mo");     # Descending/Returning
    }
}
```
Recursive functions utilize the Stack memory just like any other functions. These functions are considered memory intensive as the number of activation records will be added for each function call. In the example shown in the video, the both the Time and Space Complexities are O(n).

Practice: Implement different recursion scenarios in code.

### Recurrence Relation

#### Example
```
void func1(n) {                 ------> T(n)
    if (n > 0)                  ------> 1
    {
        printf("%d", n);        ------> 1
        func1(n-1);             ------> T(n - 1)
    }
}
```

```
T(n) = T(n - 1) + 2
```

#### Relation
```
T(n) = [ 1                  n = 0
       [ T(n-1) + 2         n > 0
```
Note: We should always change constants to 1 before solving the equation. Let's break down the first entry and perform substitutions.
```
T(n) = T(n-1) + 1
T(n-1) = T(n-2) + 1

T(n) = T(n-2) + 1 + 1
T(n) = T(n-2) + 2
T(n) = T(n-3) + 3 ....
T(n) = T(n-k) + k
What if n = k then we have n-k=0
T(n) = T(0) + n
T(n) = 1 + n    ---> O(n)
```

### Indirect Recursion
When multiple functions call each other in a circular pattern. e.g. A -> B -> C -> A

### Nested Recursion
When a function calls itself but passes itself as argument to this called function.

### Sum of n Natural Numbers (Recursion)
```
1 + 2 + 3 + 4 .... + n
Sum(n) = 1 + 2 + 3 + ... + (n-1) + n
Sum(n) = Sum(n-1) + n

Sum(n) = [ 0                           n=0
         [ Sum(n-1) + n                n>0
```
```
int sum(int n) {
    if (n>0) {
        return sum(n-1);
    }
    else {
        return 0;
    }
}
```

There's a direct formula for sum of n numbers. We can write code for this directly.
```
n(n + 1)/2
```

This problem can be solved using Iteration (loop), Recursion or directly computing the `n(n+1)/2` formula. Direct computation has both O(1) Space and Time Complexities. The Iteration has O(n) Time Complexity and O(1) Space Complexity. The Recursion has both O(n) Time and Space Complexities. In programming, we would normally solve this using Iteration (loop), however, we use recursive functions to solve it in Mathematics.

### Factorial (Recursion)
Recursive Relation:
```
n! = 1 * 2 * 3 * ... * n
fact(n) = 1 * 2 * 3 * ... * (n-1) * n
fact(n) = fact(n-1) * n
fact(n) = [ 1                          n=0
          [ fact(n-1) * n              n>0
```
This problem in programming can be solved using multiple methods, e.g. Iteration (loop), Recursion, etc. Every method might have different Time and Space Complexities.

### Power/Exponet m^n (Recursion)
```
2^5 = 2 * 2 * 2 * 2 * 2
m^n = m * m * ... for n times
pow(m,n) = m * m * ... * (n-1) times * m
Recursively:
pow(m,n) = pow(m, n-1) * m
Recursive Relation:
pow(m,n) = [1                                 n=0
           [pow(m, n-1) * m                   n>0
```

e.g: `2^8 = (2^2)^4    or 2^9 = 2 * (2^2)^4`

This shows that the power becomes half if we break it down like this. This can improve our code.

### Taylor's Series c^x (Recursion)
```
c^x = 1 + x/1 + x^2/2! + ... + x^n/n!
It has three aspects or types of values. Summation, power and factorial.
sum(n) = 1 + 2 + 3 + ... + n        => sum(n-1) + n
fact(n) = 1 * 2 * 3 * ... * n           => fact(n-1) * n
pow(x, n) = x * x * ... * n times => pow(n-1) * x
```

We can make use of static variables here by calling the process of factorial 'f' and of power 'p'. Initial values would be; p = x^0 = 1, f = o! = 1. This would be make it easier to solve and later code.

If n is 3 and the function is e(x, n), we will have:
```
e(x, 0) = 1
e(x, 1) = e(x, 0) + p/f = e(x, 0) + x^1/1! = 1 + x/1
e(x, 2) = e(x, 1) + p/f = e(x, 1) + x^2/2! = 1 + x/1 + x^2/2!
e(x, 3) = e(x, 2) + p/f = e(x, 2) + x^3/3! = 1 + x/1 + x^2/2! + x^3/3!
```

Big O = O(n^2)

We can improve it with Horner's Rule.

### Taylor's Series c^x (Horner's Rule)

In this series, let's count the number of multiplications required and come up with a Big O notation for it.
```
c^x = 1 + x/1 + x^2/2! +x^3/3! + x^4/4! ... n times
0 +  0  +   2  +     4  +     6            ...       => Multiplication counts
2[1 + 2 + 3 ....]
2[n(n+1)/2]
n(n+1) = O(n^2)    => Quadratic Time
```
Let's reduce the number of multiplications somehow to improve our code.
```
c^x = 1 + x/1 + x^2/2! +x^3/3! + x^4/4! ... n times
= 1 + x/1 + x^2/1*2 + x^3/1*2*3 + x^4/1*2*3*4 + ....
= 1 + x/1[1 + x/2 + x^2/2*3 + x^3/2*3*4]
= 1 + x/1[1 + x/2[1 + x/3[1 + x/4]]] => Reduced multiplications now  => O(n) Linear Time
```
ver here, we always first multiple and then call rather than the opposite. This problem always follows the ascending order. We can use both loop and recursion to solve it.

Cos and Sine series are homework.

Fibonacci Series (Recursion)

In this series, each value is computed by adding the last two values. e.g.
0, 1, 1, 2, 3, 5, 8, 13, 21 ...

### Recursion Relation
```
fib(n) = [ 0                              n=0
         [ 1                              n=1
         [fib(n-2) + fib(n-1)             n>1
```

In programming, we can solve it using Iteration (Loop) or Recursion. Iteration gives us O(n) while Recursion gives us O(2^n). However, we can improve or reduce computation by Recursion because we have duplicate/redundant function calls that can be avoided by storing the function results and re-using them. We were able to reduce the number of calls to (n+1) which is O(n). The process of storing or caching results is called Memoization.

### Combination nCr using Recursion
We can solve it using Pascal's Triangle as that gives us the Recursive Relation

### Tower of Hanoi using Recursion

We have three towers A, B and C, and the first tower A has some disks arranged with smaller ones on top and larger ones at the bottom. We have to move all the disks from A to C without breaking the order on any of the towers. We can also move only one disk at a time. This is considered to be an unsolvable problem historically, however, we can solve it using recursion.

```
ToH(n, A, B, C)       => n is the number of disks, A is source, B is intermediary and C is destination.
Steps
=> ToH (n-1, A, C, B)
=> Move Disk from A to C using B
=> ToH(n-1, B, A, C)
```
The Time Complexity is O(2^n) that is exponential.

## Arrays

Collection/list of similar data typed elements.

Normal variables such `int c` is called a scalar variable while Array is called a vector variable as it has a dimension. Arrays are allocated contiguous memory block which means it is continuous, uninterrupted and together.

Declaration means creating a variable without any value where as Initialization also sets the value.

When the Array is declared or initialized with a defined size (?), it is called Static Array and its memory allocated in the Stack. Static Arrays size is decided at compile time usually. By default, Arrays are allocated memory from the Stack. We can also allocate memory from Heap and such Array's size and type is decided at runtime and is called a Dynamic Array.

Size is usually fixed, however, resizable Arrays are only possible in Heap and not in Stack.

### Dimensional Arrays

In programming languages, we can create n dimension Arrays. The memory allocation of n dimensional Array is still linear in memory exactly like a single dimension array. For example in 2D Array, the addresses start from 1st row and 1st column and traverse over columns first and then rows (top left to right). This only applies to Arrays in Stack.

Another method to create 2D Array uses both the Stack and the Heap. We have to create an Array of pointers in the Stack and is of the size of first dimension. Each element will point to another Array that exists in the Heap and holds the actual elements/values.

Last method is using double pointers where all the Arrays are created in the Heap. The main pointer will still be in Stack that will point to a pointer Array in Heap which again will point to actual Arrays in the Heap that has the elements.

### Compiler Handling

Memory is allocated at runtime to any Array which means the addresses are unknown at compile time. So if we have an operation that needs to be performed on specific Array index, the compiler uses a formula that is later computed at runtime to find the address. Let's say we have a static Array of 5 int elements and we want to modify the index 3:
```
A[3]) = 10;
addr(A[3]) = Lo + 3 * 2   => 2 because we are assuming 2 bytes for int
addr(A[i]) = Lo + i * w
```
This a relative formula based on base address.

#### 2D Array
We do it using two ways; row major mapping and column major mapping.

For example:
```
int A[2][3];
[a00][a01][a02]
[a10][a11][a12]
[a20][a21][a22]
```
In row mapping, we take rows in order from the first row with a00 and map them linearly. Such as; row1, row2, row3. `|a00, a01, a02, a10, a11, a12, a20, a21, a22|`. If we have to change any of the element in array, we have to come up with a formula for the compiler since the address is unknown at compile time. For example for `A[1][2]` assuming 200 is the base address:
```
addr(A[1][2]) = 200 + [3 + 2] * 2
              = 200 + 10 = 210
```
So for A[m][n] Array:
```
addr(A[i][j]) = Lo + [ i * n + j ] * w
```
While in column major mapping, we go column by column to map linearly.
`|a00, a10, a20, a01, a11, a21, a02, a12, a22|`.
```
addr(A[1][2]) = 200 + [2*3 + 1]*2 = 214
addr(A[i][j]) = Lo + [j * m + i] * w
```
It depends on the compiler on what mapping method it uses. Both are equally efficient.

#### 3D Array
```
int A[l][m][n];
Row Major: addr(A[i][j][k]) = Lo + [i*m*n + j*n + k] * w
Column Major: addr(A[i][j][k]) = Lo + [k*l*m + j*l + i] * w
```

#### nD Array

For example 4D Array: `A[d1][d2][d3][d4]`

##### Row Major
```
Addr(A[i1][i2][i3][i4]) = Lo + [ i1*d2*d3*d4 +  i2*d3*d4 + i3*d4 + i4] * w
Left to right ----->
```
##### Column Major
```
Addr(A[i1][i2][i3][i4]) = Lo + [i4*d1*d2*d3 + i3*d1*d2 + i2*d1 + i1] * w
Right to Left <----
```

For nD Array, the formulas would be:
```
Row Major form = Lo + [n—p=1Σ ip * n—q=p+1π dq ] * w
Column Major form = Lo + [n—p=1Σ ip * p-1—1π dq ] * w
```

These operations are very compute intensive due to large number of multiplications. It has `O(n^2)` Time complexity. We can use Horner's Rule earlier and reduce the number of multiplications by taking common values out. It will reduce Time complexity to `O(n)`.

## Array as Abstract Data Type (ADT)
ADT means representation of data and set of operations on data.

### Binary Search

As a pre-requisite, the keys must be sorted first before performing the binary search. After sorting, the elements are split into two. We also need 3 index variables.
```
l;  lowest index
h; highest index
mid: [l+h]/2 => Use floor value in case of a float output.
```
*Note*: If you have two indexes left on any side, choose the one closer to mid for next mid.

During the search, if the target value is greater than mid, change low by setting it to mid+1 and if it is lower than mid then change the high by setting it to mid-1.

Binary search can be implemented using Recursion as well as Iteration because it has a tail recursion and we saw before, tail recursion can also be implemented by loops.

In a Binary Tree, number of comparisons or height of a tree is `log2(n)` where n is total elements.

We can find the max number of operations with `log2(n)`.
```
n = 2^m, log2(n) = m
```
Total time taken by all cases:
```
Summation (i=1, logn) i*2^i
avg=(Summation (i=1, logn) i*2^i)/n
```
At the end of the tree, we have square boxes representing 'no data found' path. Let's call these external operations and let's call the normal circle tree paths as internal. The number of operations/calls/paths for E will be:
```
E = I + 2n

E is external paths
I is internal paths
n is number of nodes/elements
```
Also, the number of external nodes 'e' will be:
```
e = i + 1

i is internal nodes
e is external nodes
```

Average successful paths time:
```
As(n) = 1 + I/n
```

Average unsuccessful paths time:
```
Au(n) = E/(n+1)
```

E can also be written as E=(n+1)logn ~= nlogn because all the external nodes are at the same height that is logn. For all external nodes, this will become: 
```
E = e * logn = (n+1)logn.
```

We can use this in our Average unsuccessful search time Au(n);
```
Au(n) = E/(n+1) = (n+1)logn/(n+1) = logn
```

Similarly, As(n) would be;
```
As(n) = 1 + I/n = 1 + (E - 2n)/n = 1 + E/n - 2 = nlogn/n - 1 = logn - 1
```
