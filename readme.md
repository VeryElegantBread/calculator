# Calculator
This is a calculator.
## Installation
### 1. Install rust
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
### 2. Clone the repository
`git clone https://github.com/VeryElegantBread/calculator/`
### 3. Go into the project Directory
`cd calculator`
### 4. Build
`cargo build --release`
### Find the built file
It should be at calculator/target/release/calculator
## Usage
Run with an equation to get the result
```
$ calculator 5 * (3 + 7)
50.0
```
or without one to get a continuous dialogue where you can input multiple equations.
```
$ calculator
--> 7 * 3.5
24.5
--> 8 ^ 2
64.0
--> 7
7.0
-->
```
In a continuous dialogue, you can set variables and they update as variables used to make them change.
```
$ calculator
--> num1 = 9
num1 = 9.0
--> num2 = 7 + num1
num2 = 16.0
--> num1 = 10
num1 = 10.0
--> num2
17.0
-->
```
Default variables:

| Name           | Value   |
| -------------- | ------- |
| pi             | 3.14159 |

The current operations you can do:

| Name           | Sign | Example   |
| -------------- | ---- | --------- |
| Addition       | +    | 1 + 7 = 8 |
| Subtraction    | -    | 7 - 1 = 6 |
| Multiplication | *    | 3 * 2 = 6 |
| Division       | /    | 8 / 2 = 4 |
| Modulos        | %    | 7 % 3 = 1 |
| Exponents      | ^    | 2 ^ 3 = 8 |
## Issues
- Can crash if you don't format your equation like it likes
- Doesn't follow order of operations
