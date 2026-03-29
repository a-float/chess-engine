use crate::evaluate::Evaluator;

pub struct SumEvaluator {
    evaluators: Vec<Box<dyn Evaluator>>,
}

impl SumEvaluator {
    pub fn new(evaluators: Vec<Box<dyn Evaluator>>) -> Self {
        Self { evaluators }
    }

    pub fn evaluate_breakdown(&self, board: &crate::Board) -> Vec<(String, i32)> {
        self.evaluators
            .iter()
            .map(|e| (e.name(), e.evaluate(board)))
            .collect()
    }
}

impl Evaluator for SumEvaluator {
    fn name(&self) -> String {
        format!(
            "sum({})",
            self.evaluators
                .iter()
                .map(|evaluator| evaluator.name())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn evaluate(&self, board: &crate::Board) -> i32 {
        self.evaluators
            .iter()
            .map(|evaluator| evaluator.evaluate(board))
            .sum()
    }
}
