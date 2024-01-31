use byteorder::{ByteOrder, LittleEndian};
use cairo_vm::{
    types::instruction::Instruction,
    utils::PRIME_STR,
    vm::{decoding::decoder::decode_instruction, trace::trace_entry::RelocatedTraceEntry},
    Felt252,
};
use num_bigint::BigUint;
use serde::{Serialize, Serializer};
use serde_json::json;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

pub struct InstructionSerializable(Instruction);

impl Serialize for InstructionSerializable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let instruction = &self.0;

        // Create a JSON object
        let j = json!({
            "off0": format!("{:?}", instruction.off0),
            "off1": format!("{:?}", instruction.off1),
            "off2": format!("{:?}", instruction.off2),
            "dst_register": format!("{:?}", instruction.dst_register),
            "op0_register": format!("{:?}", instruction.op0_register),
            "op1_addr": format!("{:?}", instruction.op1_addr),
            "res": format!("{:?}", instruction.res),
            "pc_update": format!("{:?}", instruction.pc_update),
            "ap_update": format!("{:?}", instruction.ap_update),
            "fp_update": format!("{:?}", instruction.fp_update),
            "opcode": format!("{:?}", instruction.opcode),
        });

        // Serialize the JSON object
        j.serialize(serializer)
    }
}

#[derive(Serialize)]
pub struct TracerData {
    pub pc_inst_map: HashMap<usize, InstructionSerializable>,
    pub trace: Vec<RelocatedTraceEntry>,
    pub memory: HashMap<usize, String>,
}

pub fn make_tracer_data(
    trace: Vec<RelocatedTraceEntry>,
    memory: Vec<Option<Felt252>>,
) -> TracerData {
    let mut pc_inst_map: HashMap<usize, InstructionSerializable> = HashMap::new();

    for entry in trace.iter() {
        let (instruction_encoding, _) = get_instruction_encoding(entry.pc, &memory)
            .expect("Failed to get instruction encoding");
        let instruction_encoding = instruction_encoding.to_bytes_le();
        let instruction_encoding = LittleEndian::read_u64(&instruction_encoding[..]);
        let instruction =
            decode_instruction(instruction_encoding).expect("Failed to decode instruction");
        pc_inst_map.insert(entry.pc, InstructionSerializable(instruction));
    }

    let memory = memory
        .iter()
        .filter_map(|x| x.as_ref().map(|_| x.clone().unwrap()))
        .map(|x| x.to_hex_string())
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .collect();

    TracerData {
        pc_inst_map,
        trace,
        memory,
    }
}

// Returns the encoded instruction (the value at pc) and the immediate value (the value at
// pc + 1, if it exists in the memory).
pub fn get_instruction_encoding(
    pc: usize,
    memory: &[Option<Felt252>],
) -> Result<(Felt252, Option<Felt252>), Error> {
    if memory[pc].is_none() {
        return Err(Error::new(ErrorKind::Other, ""));
    }
    let instruction_encoding = memory[pc].clone().unwrap();
    let prime = BigUint::parse_bytes(PRIME_STR[2..].as_bytes(), 16).unwrap();

    let imm_addr = BigUint::from(pc + 1) % prime;
    let imm_addr =
        usize::try_from(imm_addr.clone()).map_err(|_| Error::new(ErrorKind::Other, ""))?;
    let optional_imm = memory[imm_addr].clone();
    Ok((instruction_encoding, optional_imm))
}
