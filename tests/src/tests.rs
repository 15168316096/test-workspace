// Include your tests here
// See https://github.com/xxuejie/ckb-native-build-sample/blob/main/tests/src/tests.rs for examples

use ckb_testtool::{
    ckb_types::{
        bytes::Bytes,
        core::{ScriptHashType, TransactionBuilder},
        packed::{CellInput, CellOutput},
        prelude::*,
    },
    context::Context,
};

const MAX_CYCLES: u64 = 500_0000;

#[test]
fn test_ttt() {
    let mut context = Context::default();
    let test_r1_out_point = context.deploy_cell_by_name("c1");
    let lock_script = context
        .build_script_with_hash_type(
            &test_r1_out_point,
            ScriptHashType::Data2,
            Default::default(),
        )
        .expect("script")
        .as_builder()
        .args([11u8; 30].to_vec().pack())
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

    let test_r2_out_point = context.deploy_cell_by_name("c2-dbg");
    let lock_script2 = context
        .build_script_with_hash_type(
            &test_r2_out_point,
            ScriptHashType::Data2,
            Default::default(),
        )
        .expect("script")
        .as_builder()
        .args([11u8; 30].to_vec().pack())
        .build();
    let input2 = CellInput::new_builder()
        .previous_output(
            context.create_cell(
                CellOutput::new_builder()
                    .capacity(1000u64.pack())
                    .lock(lock_script2)
                    .build(),
                Bytes::new(),
            ),
        )
        .build();

    let test_r3_out_point = context.deploy_cell_by_name("c3-sim");
    let lock_script3 = context
        .build_script_with_hash_type(
            &test_r3_out_point,
            ScriptHashType::Data2,
            Default::default(),
        )
        .expect("script")
        .as_builder()
        .args([11u8; 30].to_vec().pack())
        .build();
    let input3 = CellInput::new_builder()
        .previous_output(
            context.create_cell(
                CellOutput::new_builder()
                    .capacity(1000u64.pack())
                    .lock(lock_script3)
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
        .set_inputs(vec![input,input3, input2, ])
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();
    let tx = context.complete_tx(tx);

    // run
    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
}
