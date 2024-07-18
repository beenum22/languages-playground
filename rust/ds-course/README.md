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

#### Merging Arrays
The Big O or time complexity in case of merging is denoted by a theta:
```
θ(m+n)
```

## String Data Type

Character Set is a set of characters that are supported by the programming language. Computer systems work on binary numbers. We define every character as a binary number/code which is standardized for every machine and system. This standard is called American Standard Code for Information Interchange (ASCII) codes. These codes are for English language specifically. For other languages, the standard that defines the codes is called Unicodes.
```
A -> 65
B -> 66
...
Z -> 90

a -> 97
b -> 98
...
z -> 122

0 -> 48
1 -> 49
...
9 -> 57

Enter -> 10
Space -> 13
Escape -> 27
```
There are total `128` codes. An ASCII code only requires `7 bits` to represent because `2^7 = 128`. This is why we say a char requires `1 Byte` in the memory.

Unicodes are represented by `16 bits` as it covers extensive range of languages. Also, it's codes are written in hexadecimal. In memory, it takes `2 Bytes` for each character.

Array of characters is called a string. In C, character is enclosed in single quotes and string in double quotes.

### Bitwise Operations

Finding out if a bit is `on` in memory or not is called **Masking**. To find out the status of a desired bit, we can have another byte with that specific bit `on` only and perform bitwise `AND` with the desired byte in memory to find its status. Similarly, setting a bit in memory is called Merging.

There are three common bitwise operations:
* Shifting - Moving bits from right to left.
* Merging - Setting another bit on in a byte. OR operation needed
* Masking - Finding status of a bit in a byte. AND operation needed.

### Permutations of a String
If we create a tree for the permutation, the tree itself is called State Space Tree and the paths are kind of called Back Tracking. In permutation tree, all the possible outcomes are shown and hence it is called Brute Force. The end of the tree branches are called Leaf Nodes.

Any process where we have to Back Track and perform something, it can be achieved using Recursion.

## Matrix
### Square Matrix

Any matrix that has n rows and n columns i.e nxn. There are different sub types of a square matrix depending on the arranging or type of values. e.g. Diagonal matrix where all elements are zero except the middle diagonal line.

Diagonal Matrix: `M[i, j] = 0 if i != j`

In Matrices where we have a pattern of zeroes for example Diagonal, we don't have to store these zeroes or perform operations on them as it is waste of time and space. We can rather convert them into some other form. e.g. We can convert 2D diagonal array to a 1D array.

### Diagonal Matrix
```
M[i, j] is non-zero if i == j
M[i, j] = 0 if i != j
```

### Lower Triangular Matrix
```
M[i, j] is non-zero if i >= j
M[i, j] = 0 if i < j
```

Lower half/non-zero elements are: `n(n + 1)/2`

Upper half/zero elements are: `n^2 - n(n + 1)/2 = n(n - 1)/2`

We can store values both using row-major or column major methods. For row-major, we can find a value at an index: `[ i(i - 1)/2 ] + j - 1`.

For column-major, we can find a value at an index: `[ n(j - 1) + (j - 2)(j - 1)/2 ] + (j - i)`

### Upper Triangular Matrix
```
M[i, j] is non-zero if i <= j
M[i, j] = 0 if i > j
```
Lower half/non-zero elements are: `n(n + 1)/2`

Upper half/zero elements are: `n^2 - n(n + 1)/2 = n(n - 1)/2`

We can store values both using row-major or column major methods.
For row-major, we can find a value at an index: `[ n(i - 1) + (ji- 2)(i - 1)/2 ] + (j- i)`

For column-major, we can find a value at an index: `[ j(j - 1)/2 ] + i- 1`

### Symmetric Matrix

When `M[i, j] == M[j, i]`

In programming, we can represent it using Upper or Lower Triangular Matrix.

### Tridiagonal Matrix

We have three diagonal bands such that:
Main diagonal: `i - j = 0`
Lower diagonal: `i - j = 1`
Upper diagonal: `i - j = -1`
```
|i - j| <= 1
M[i, j] is non-zero if |i - j| <= 1
M[i, j] = 0 if |i -j| > 1
Total non-zero elements: n + (n - 1) + (n - 1)
```
We want to avoid storing zeroes, however, we can't use row-major or column-major because the elements in each row/column are non-uniform. We will go diagonal by diagonal, e.g, lower first, main then and upper. To find indexes, we can use:
```
if i - j = 1 then index: i - 1
if i - j = 0 then index: n - 1 + i - 1
if i - j = -1 then index: 2n - 1 + i - 1
```

### Square Band Matrix

When there are more than one diagonals on each side and they are equal number of diagonals then it's called Square Band Matrix.

### Toeplitz Matrix
```
M[i, j] = M[i - 1, j - 1]
```
Total elements: `n + n - 1`

First store a row and then a column in our 1D array.

Case 1: if i <= j then index = j - i   similar to our upper triangle matrix.

Case 2: if i > j then index = n + i - j - 1  similar to our lower triangle matrix.

### Sparse Matrix

A matrix where there are more zero elements.

There are two ways to implement such matrices:
* Co-ordinate List / 3-column Representation
* Compressed Sparse Row

#### For 3-column Representation
We need 3 values; row, col and val that will form a tuple. We will have an array of tuples. First index will have a tuple with total rows, cols and elements.

#### Compressed Sparse Row
In this method, we first create an array and store all the non-zero values and then we initialize another array and go over our matrix row by row and count the number of non-zero elements appearing. We store a 0 at index 0 to represent the 0,0 index of a matrix(which doesn't exist) and then we move downwards counting the elements appearing and storing the accumulated value for each row in a separate index in our initialized array. Then we initialize another/third array, go over our first array of elements and store column numbers of each element in our new array.

#### Sparse Matrix Addition
For adding any matrices, the dimensions must be same.

Addition using Co-ordinate List:

## Polynomial

Polynomial are represented on paper using a single line formulas such as:
```
p(x) = 3x^5 + 2x^4 + 2x + 7
```

The above formula uses a single variable and is called a Univariate Formulas.

In programming, we can represent this polynomial using an array where we keep track of co-efficient and exponent of each term.

## Linked Lists

Arrays have a problem that we always have to set the size of it whether it is in Stack or Heap. Array have a fixed size. In cases where we don't know the number of elements, the size of the array would be either insufficient or excess.

We can use linked list for such cases. A linked list is a chain of nodes connected to each other using pointers. Every node has a value and a pointer to the next node. This node is created in Heap. Disadvantage here is that memory is not contiguous like arrays.

We always create linked lists in Heap memory as Stack allocation is not suitable for dynamic memory allocation.

First node in a list is called a Head and is pointing to the first data node. Rest of the nodes are called Data Nodes or just Nodes. The pointer in nodes of type Node.

A structure/struct that points to structs of the same type are called self-referential structures.

In C++, Class and Struct is the same but everything in a Class is private by default and in a Struct is public.

Size in memory for Node struct is data Datatype Bytes plus next Pointer Bytes. e.g. if data is int, the struct will take 2+2=4 Bytes.

In a compiler if Integer takes 2 Bytes, Pointer will also take 2 Bytes and if Integer takes 4 Bytes, Pointer will also take 4 Bytes.

If we display node values using Loops or Recursion, both the Time and Space complexities will be `O(n)`.

Using Recursion if we call first and then print, the Linked List will be printed in reverse order.

### Linked Lists Search

We can search through the list using two methods; Linear and Binary. Binary works only on a sorted items. We saw in Arrays that the Binary search improves Time complexity, however, that's not the case here. We don't know the middle of the Linked List so we always have to traverse to find the middle which make it as efficient as Linear search only. So in case of Linked Lists, we don't use Binary search.

We can improve the Linear search with Move To Head search though.

### Linked List Insert

Linked Lists don't have indices like Arrays so we have to assume that every Node has an index starting from 0 or 1. We can further divide insertion into two types; Insert Before first and Insert after given position.

## Stack

ADT Stack Data Structure follows LIFO (Last-in First-out) pattern.

We are used to such a data structure that we know as Stack already (in terms of memory).

One example of Stack use-case is when you want to convert a Recursive approach to Iterative approach.

P.S. ADT means representation and operations on certain data.