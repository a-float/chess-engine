use crate::evaluate::Evaluator;

pub struct SumEvaluator {
    evaluators: Vec<Box<dyn Evaluator>>,
}

impl SumEvaluator {
    pub fn new(evaluators: Vec<Box<dyn Evaluator>>) -> Self {
        Self { evaluators }
    }

    pub fn get_evaluators(&self) -> &Vec<Box<dyn Evaluator>> {
        &self.evaluators
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
