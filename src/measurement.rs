pub trait Measurement {
    /// This type represents an intermediate value for the measurements. It will be produced by the
    /// start function and passed to the end function. An example might be the wall-clock time as
    /// of the `start` call.
    type Intermediate;

    /// The benchmark will call this before starting the benchmark.
    fn start(&self) -> Self::Intermediate;

    /// The benchmark will call this after finishing the benchmark in order to obtain the measured value.
    fn end(&self, i: Self::Intermediate) -> serde_json::Number;

    /// This returns an (unique) identifier for the measurement.
    fn id(&self) -> &'static str;

    /// This returns a human readable name of the measurement.
    fn name(&self) -> &'static str;

    /// This returns the unit of the measurement (as SI unit string).
    fn unit(&self) -> &'static str;
}
