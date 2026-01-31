mod material;
mod mobility;
mod positioning;
mod sum;

pub use material::MaterialEvaluator;
pub use mobility::MobilityEvaluator;
pub use positioning::PositioningEvaluator;
pub use sum::SumEvaluator;

pub trait Evaluator {
    fn evaluate(&self, board: &crate::Board) -> i32;
    fn evaluate_for_white(&self, board: &crate::Board) -> i32 {
        self.evaluate(board) * board.get_active_color().get_value()
    }
    fn name(&self) -> String;
}
