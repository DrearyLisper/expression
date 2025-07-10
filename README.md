# Expression Evaluator

This is a simple expression evaluator written in Rust. It provides a REPL (Read-Eval-Print Loop) to evaluate mathematical expressions.

## How to use

1.  **Build the project:**
    ```bash
    cargo build
    ```

2.  **Run the REPL:**
    ```bash
    cargo run
    ```

3.  **Enter expressions:**
    Once the REPL starts, you can enter mathematical expressions at the `>>` prompt. The evaluator supports addition (`+`), subtraction (`-`), multiplication (`*`), and division (`/`), as well as parentheses for grouping.

## Examples

Here are some examples of expressions you can evaluate:

```
>> 1+2
> 1+2 = 3
>> 2*3
> 2*3 = 6
>> (1+2)*3
> (1+2)*3 = 9
>> 10/2
> 10/2 = 5
>> 5-3
> 5-3 = 2
```

## How to exit

To exit the REPL, you can use `CTRL-C` or `CTRL-D`.