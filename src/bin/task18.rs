/*
    The technical specification was prepared using Gemini

 * PROJECT: RustyPipeline
 * DESCRIPTION: A generic data processing pipeline using Traits and Closures.
 *
 * GOALS:
 * 1. Define a Trait `Transform`:
 * - Method `apply(&self, value: i64) -> i64`
 *
 * 2. Implement `Transform` for two structs:
 * - `Multiplier { factor: i64 }`: Multiplies input by factor.
 * - `Adder { amount: i64 }`: Adds amount to input.
 *
 * 3. Create a struct `Pipeline`:
 * - store: Vec<Box<dyn Transform>> (A list of different transformers).
 * - Note: `Box<dyn ...>` is how Rust handles a list of different objects 
 * sharing the same Trait (Dynamic Dispatch).
 *
 * 4. Implement methods for `Pipeline`:
 * - `add_step(&mut self, step: Box<dyn Transform>)`: Add a new transformation.
 * - `run(&self, data: Vec<i64>) -> Vec<i64>`: Process the entire vector 
 * through all steps in the pipeline.
 *
 * 5. ADVANCED (Senior Level):
 * - Add a method `filter_map<F>(&self, data: Vec<i64>, f: F) -> Vec<i64>`
 * where `F` is a Closure: `Fn(i64) -> Option<i64>`.
 * This should apply the closure to each element and only keep `Some` results.
 *
 * 6. MAIN FUNCTION:
 * - Create a pipeline: Add an Adder(10), then a Multiplier(2).
 * - Run data `vec![1, 2, 3]` through it (Result should be [22, 24, 26]).
 * - Use the `filter_map` to remove numbers greater than 25.
 */

trait Transform {
    fn apply(&self, value: i64) -> i64;
}

struct Adder {
    amount: i64,
}

struct Multiplier {
    factor: i64
}

struct Pipeline {
    stroke: Vec<Box<dyn Transform>>
}

impl Transform for Adder {
    fn apply(&self, value: i64) -> i64 {
        self.amount + value
    }
}

impl Transform for Multiplier {
    fn apply(&self, value: i64) -> i64 {
        self.factor * value
    }
}

impl Pipeline {
    fn add_step(&mut self, step: Box<dyn Transform>) {
        self.stroke.push(step);
    }

    fn run(&self, data: Vec<i64>) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::new();
        
        for value in data {
            let mut v = value;

            for step in &self.stroke {
                v = step.apply(v);
            }
            result.push(v);
        }

        result
    }

    fn filter_map<F>(&self, data: Vec<i64>, f: F) -> Vec<i64>
    where
        F: Fn(i64) -> Option<i64>
    {
        let mut result = Vec::new();
        for e in data {
            if let Some(v) = f(e) {
                result.push(v);
            }
        }

        result
    }
}

fn main() {
    let mut pipeline = Pipeline { stroke: Vec::new() };

    pipeline.add_step(Box::new(Adder { amount: 10 }));
    pipeline.add_step(Box::new(Multiplier { factor: 2 }));

    let data = vec![1, 2, 3];

    let result = pipeline.run(data);
    println!("{:?}", result); // [22, 24, 26]

    let filtered = pipeline.filter_map(result, |x| {
        if x <= 25 {
            Some(x)
        } else {
            None
        }
    });

    println!("{:?}", filtered); // [22, 24]
}