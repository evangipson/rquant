/// [`QuantumOperator`] is a collection of quantum logical operators that produce varying
/// output given one or many [`Qubits`](crate::quantum::types::qubit::Qubit).
#[derive(Clone, Debug, PartialEq)]
pub enum QuantumOperator {
    /// The NOT [`QuantumOperator`] flips the amplitudes of the $|0\rangle$ and
    /// $|1\rangle$ states, analagous to a classic logical NOT.
    ///
    /// The NOT [`QuantumOperator`] can be represented by the following matrix:
    /// $$N=\begin{pmatrix} 0 & 1 \\\ 1 & 0 \end{pmatrix}$$
    ///
    /// # Example
    /// The NOT [`QuantumOperator`] can flip the amplitude of a qubit such that:
    ///
    /// $$ \begin{pmatrix} 0 & 1 \\\ 1 & 0 \end{pmatrix}
    /// \begin{pmatrix} \alpha \\\ \beta \end{pmatrix} =
    /// \begin{pmatrix} \beta \\\ \alpha \end{pmatrix} $$
    NOT,
    /// The ROTATE [`QuantumOperator`] rotates a [`Qubit`](crate::quantum::types::qubit::Qubit)
    /// 180 degrees around it's Y-axis.
    ///
    /// The ROTATE [`QuantumOperator`] can be represented by the following matrix:
    /// $$R=\begin{pmatrix} 0 & -i \\\ i & 0 \end{pmatrix}$$
    ///
    /// # Example
    /// The ROTATE [`QuantumOperator`] can phase the qubit such that:
    ///
    /// $$ \begin{pmatrix} 0 & -i \\\ i & 0 \end{pmatrix}
    /// \begin{pmatrix} \alpha \\\ \beta \end{pmatrix} =
    /// \begin{pmatrix} -i \beta \\\ i \alpha \end{pmatrix} =
    /// -\beta|0\rangle + \alpha|1\rangle $$
    ROTATE,
    /// The PHASE [`QuantumOperator`] leaves the state of a [`Qubit`](crate::quantum::types::qubit::Qubit)
    /// unchanged, and flips the phase of the $|1\rangle$ state by $\pi$.
    ///
    /// The PHASE [`QuantumOperator`] can be represented by the following matrix:
    /// $$P=\begin{pmatrix} 1 & 0 \\\ 0 & -1 \end{pmatrix}$$
    ///
    /// # Example
    /// A PHASE [`QuantumOperator`] can phase and flip a [`Qubit`](crate::quantum::types::qubit::Qubit)
    /// such that:
    ///
    /// $$ \begin{pmatrix} 1 & 0 \\\ 0 & -1 \end{pmatrix}
    /// \begin{pmatrix} \alpha \\\ \beta \end{pmatrix} =
    /// \begin{pmatrix} \alpha \\\ -\beta \end{pmatrix} =
    /// \alpha|0\rangle - \beta|1\rangle $$
    PHASE,
    /// The SUPERPOSITION [`QuantumOperator`] turns the amplitude of the $|0\rangle$ and $|1\rangle$
    /// states of a [`Qubit`](crate::quantum::types::qubit::Qubit) into an equal superposition of $|0\rangle$
    /// and $|1\rangle$.
    ///
    /// Also referred to as a "Hadamard gate".
    ///
    /// The SUPERPOSITION [`QuantumOperator`] can be represented by the following matrix:
    /// $$S=\frac{1}{\sqrt{2}}\begin{pmatrix} 1 & 1 \\\ 1 & -1 \end{pmatrix}$$
    ///
    /// # Example
    /// A SUPERPOSITION [`QuantumOperator`] can modify the amplitude of a
    /// [`Qubit`](crate::quantum::types::qubit::Qubit) such that:
    ///
    /// $$H|0\rangle = \frac{1}{\sqrt{2}}(|0\rangle + |1\rangle)$$
    /// $$H|1\rangle = \frac{1}{\sqrt{2}}(|0\rangle - |1\rangle)$$
    SUPERPOSITION,
}
