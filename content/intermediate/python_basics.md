# Python Basics

## Introduction to Python

Python is a high-level, interpreted programming language known for its readability and simplicity. Created by Guido van Rossum and first released in 1991, Python has become one of the most popular programming languages in the world.

## Why Learn Python?

- **Readability**: Clear, readable syntax
- **Versatility**: Used in web development, data science, AI, automation, and more
- **Large Community**: Extensive libraries and frameworks
- **Beginner-Friendly**: Easy to learn and use
- **High Demand**: Many job opportunities

## Setting Up Python

### Installation

1. Visit [python.org](https://www.python.org/downloads/)
2. Download the latest version for your operating system
3. Run the installer (make sure to check "Add Python to PATH" on Windows)
4. Verify installation by opening a terminal or command prompt and typing:

```bash
python --version
```

### Development Environments

- **IDLE**: Python's built-in IDE
- **Visual Studio Code**: Popular editor with Python extensions
- **PyCharm**: Full-featured Python IDE
- **Jupyter Notebooks**: Great for data science and learning

## Python Syntax Basics

### Comments

```python
# This is a single-line comment

"""
This is a 
multi-line comment (also called a docstring)
"""
```

### Variables and Data Types

Python is dynamically typed - you don't need to declare variable types:

```python
# Basic data types
name = "John"                # String
age = 30                     # Integer
height = 5.11                # Float
is_student = True            # Boolean

# Data structures
my_list = [1, 2, 3, 4, 5]    # List (mutable)
my_tuple = (1, 2, 3)         # Tuple (immutable)
my_dict = {"name": "John", "age": 30}  # Dictionary
my_set = {1, 2, 3, 4, 5}     # Set
```

### Basic Operations

```python
# Arithmetic operations
x = 10
y = 3

print(x + y)    # 13 (Addition)
print(x - y)    # 7 (Subtraction)
print(x * y)    # 30 (Multiplication)
print(x / y)    # 3.3333... (Division)
print(x // y)   # 3 (Integer division)
print(x % y)    # 1 (Modulus - remainder)
print(x ** y)   # 1000 (Exponentiation)

# String operations
first_name = "John"
last_name = "Doe"
full_name = first_name + " " + last_name  # Concatenation
print(full_name)  # "John Doe"

greeting = f"Hello, {full_name}!"  # f-string formatting
print(greeting)  # "Hello, John Doe!"
```

## Control Flow

### Conditional Statements

```python
age = 18

if age >= 21:
    print("You can vote and drink")
elif age >= 18:
    print("You can vote but not drink")
else:
    print("You cannot vote or drink")
```

### Loops

```python
# For loop
for i in range(5):  # range(5) generates 0, 1, 2, 3, 4
    print(i)

# Looping through a list
fruits = ["apple", "banana", "cherry"]
for fruit in fruits:
    print(fruit)

# While loop
count = 0
while count < 5:
    print(count)
    count += 1

# Loop control statements
for i in range(10):
    if i == 3:
        continue  # Skip this iteration
    if i == 8:
        break  # Exit the loop
    print(i)
```

## Functions

```python
# Defining a function
def greet(name):
    """This function greets the person passed in as parameter"""
    return f"Hello, {name}!"

# Calling a function
message = greet("Alice")
print(message)  # "Hello, Alice!"

# Default parameters
def greet_with_time(name, time_of_day="day"):
    return f"Good {time_of_day}, {name}!"

print(greet_with_time("Bob"))         # "Good day, Bob!"
print(greet_with_time("Bob", "evening"))  # "Good evening, Bob!"

# Arbitrary arguments
def add_numbers(*args):
    result = 0
    for num in args:
        result += num
    return result

print(add_numbers(1, 2, 3, 4))  # 10
```

## Working with Files

```python
# Writing to a file
with open("example.txt", "w") as file:
    file.write("Hello, World!\n")
    file.write("This is a test file.")

# Reading from a file
with open("example.txt", "r") as file:
    content = file.read()
    print(content)

# Reading line by line
with open("example.txt", "r") as file:
    for line in file:
        print(line.strip())  # strip() removes trailing newline
```

## Error Handling

```python
try:
    # Code that might cause an exception
    x = 10 / 0
except ZeroDivisionError:
    # Handle specific exception
    print("Cannot divide by zero")
except Exception as e:
    # Handle any other exception
    print(f"An error occurred: {e}")
finally:
    # This code runs no matter what
    print("This always executes")
```

## Modules and Packages

Python's functionality can be extended using modules and packages:

```python
# Importing a module
import math
print(math.sqrt(16))  # 4.0

# Importing specific functions
from math import sqrt, pi
print(sqrt(16))  # 4.0
print(pi)       # 3.141592653589793

# Importing with alias
import math as m
print(m.sqrt(16))  # 4.0
```

## Next Steps with Python

After mastering these basics, you can explore:

- Object-Oriented Programming in Python
- Data manipulation with NumPy and Pandas
- Web development with Django or Flask
- Data visualization with Matplotlib
- Machine learning with scikit-learn or TensorFlow

Python's versatility makes it a valuable tool for almost any programming task!
