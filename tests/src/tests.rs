// Include your tests here
// See https://github.com/xxuejie/ckb-native-build-sample/blob/main/tests/src/tests.rs for examples

use ckb_testtool::{
    ckb_types::{
        bytes::Bytes,
        core::{DepType, ScriptHashType, TransactionBuilder},
        packed::{CellDep, CellInput, CellOutput},
        prelude::*,
    },
    context::Context,
};

const MAX_CYCLES: u64 = 500_0000;

#[test]
fn test_exec() {
    let mut context = Context::default();
    let out_point_exec_parent = context.deploy_cell_by_name("exec-parent");
    let out_point_exec_child = context.deploy_cell_by_name("exec-child");

    let exec_child_code_hash = context
        .cells
        .get(&out_point_exec_child)
        .map(|(_, bin)| CellOutput::calc_data_hash(bin).as_bytes().to_vec())
        .unwrap();
    println!("=== exec child code hash: {:02x?}", &exec_child_code_hash);

    let lock_script = context
        .build_script_with_hash_type(
            &out_point_exec_parent,
            ScriptHashType::Data2,
            Default::default(),
        )
        .expect("script")
        .as_builder()
        .args(
            vec![exec_child_code_hash, vec![ScriptHashType::Data2.into()]]
                .concat()
                .pack(),
        )
        .build();
    let input: CellInput = CellInput::new_builder()
        .previous_output(
            context.create_cell(
                CellOutput::new_builder()
                    .capacity(1000u64.pack())
                    .lock(lock_script.clone())
                    .build(),
                Bytes::new(),
            ),
        )
        .build();

    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    // build transaction
    let tx = TransactionBuilder::default()
        // .set_inputs(vec![input, input3, input2])
        .set_inputs(vec![input])
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(
            CellDep::new_builder()
                .out_point(out_point_exec_child)
                .dep_type(DepType::Code.into())
                .build(),
        )
        .build();

    let tx = context.complete_tx(tx);

    // run
    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
}

#[test]
fn test_spawn() {
    let mut context = Context::default();
    let out_point_parent = context.deploy_cell_by_name("spawn-parent");
    let out_point_child = context.deploy_cell_by_name("spawn-child");

    let exec_child_code_hash = context
        .cells
        .get(&out_point_child)
        .map(|(_, bin)| CellOutput::calc_data_hash(bin).as_bytes().to_vec())
        .unwrap();
    println!("=== exec child code hash: {:02x?}", &exec_child_code_hash);

    let lock_script = context
        .build_script_with_hash_type(&out_point_parent, ScriptHashType::Data2, Default::default())
        .expect("script")
        .as_builder()
        .args(
            vec![exec_child_code_hash, vec![ScriptHashType::Data2.into()]]
                .concat()
                .pack(),
        )
        .build();
    let input: CellInput = CellInput::new_builder()
        .previous_output(
            context.create_cell(
                CellOutput::new_builder()
                    .capacity(1000u64.pack())
                    .lock(lock_script.clone())
                    .build(),
                Bytes::new(),
            ),
        )
        .build();

    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    // build transaction
    let tx = TransactionBuilder::default()
        // .set_inputs(vec![input, input3, input2])
        .set_inputs(vec![input])
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(
            CellDep::new_builder()
                .out_point(out_point_child)
                .dep_type(DepType::Code.into())
                .build(),
        )
        .build();

    let tx = context.complete_tx(tx);

    // run
    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
}
