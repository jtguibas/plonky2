use crate::circuit_builder::CircuitBuilder;
use crate::field::extension_field::target::ExtensionTarget;
use crate::field::extension_field::Extendable;
use crate::field::field::Field;
use crate::gates::gate::{Gate, GateRef};
use crate::generator::{SimpleGenerator, WitnessGenerator};
use crate::target::Target;
use crate::vars::{EvaluationTargets, EvaluationVars};
use crate::wire::Wire;
use crate::witness::PartialWitness;

/// A gate for inserting a value into a list at a non-deterministic location.
pub struct InsertionGate;

impl InsertionGate {
    pub fn get<F: Extendable<D>, const D: usize>() -> GateRef<F, D> {

        GateRef::new(InsertionGate)
    }

    pub const CONST_INPUT: usize = 0;

    pub const WIRE_OUTPUT: usize = 0;
}

impl<F: Extendable<D>, const D: usize> Gate<F, D> for InsertionGate {
    fn id(&self) -> String {
        format!("{:?}<D={}>", self, D)
    }

    fn eval_unfiltered(&self, vars: EvaluationVars<F, D>) -> Vec<F::Extension> {
        let input = vars.local_constants[Self::CONST_INPUT];
        let output = vars.local_wires[Self::WIRE_OUTPUT];
        vec![output - input]
    }

    fn eval_unfiltered_recursively(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        vars: EvaluationTargets<D>,
    ) -> Vec<ExtensionTarget<D>> {
        let input = vars.local_constants[Self::CONST_INPUT];
        let output = vars.local_wires[Self::WIRE_OUTPUT];
        vec![builder.sub_extension(output, input)]
    }

    fn generators(
        &self,
        gate_index: usize,
        local_constants: &[F],
    ) -> Vec<Box<dyn WitnessGenerator<F>>> {
        let gen = ConstantGenerator {
            gate_index,
            constant: local_constants[0],
        };
        vec![Box::new(gen)]
    }

    fn num_wires(&self) -> usize {
        1
    }

    fn num_constants(&self) -> usize {
        1
    }

    fn degree(&self) -> usize {
        1
    }

    fn num_constraints(&self) -> usize {
        1
    }
}

#[derive(Debug)]
struct InsertionGenerator<F: Field> {
    gate_index: usize,
    
}

impl<F: Field> SimpleGenerator<F> for InsertionGenerator<F> {
    fn dependencies(&self) -> Vec<Target> {
        Vec::new()
    }

    fn run_once(&self, _witness: &PartialWitness<F>) -> PartialWitness<F> {
        let wire = Wire {
            gate: self.gate_index,
            input: ConstantGate::WIRE_OUTPUT,
        };
        PartialWitness::singleton_target(Target::Wire(wire), self.constant)
    }
}

#[cfg(test)]
mod tests {
    
}
