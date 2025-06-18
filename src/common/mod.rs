pub mod lang;
pub(crate) mod perf;
pub(crate) mod systems;
pub(crate) mod painter;
pub(crate) mod ray;
pub mod item;
pub mod cuboid;

pub enum PointResolutionError {
    NoSuchPoint,
    NoSuchReferent,
    PropagatedError,
    Other,
}
